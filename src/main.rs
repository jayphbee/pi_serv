#![allow(unused_imports)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use clap::{App, Arg, ArgMatches, SubCommand};
use env_logger;
use num_cpus::get_physical;
use parking_lot::{Condvar, Mutex, RwLock, WaitTimeoutResult};

use hash::XHashMap;
use r#async::rt::{
    multi_thread::{MultiTaskPool, MultiTaskRuntime},
    single_thread::{SingleTaskRunner, SingleTaskRuntime},
    AsyncRuntime,
};
use tcp::{
    buffer_pool::WriteBufferPool,
    connect::TcpSocket,
    driver::SocketConfig,
    server::{AsyncPortsFactory, SocketListener},
};
use vm_builtin::{ContextHandle, VmStartupSnapshot};
use vm_core::{debug, init_v8, vm};
use ws::server::WebsocketListenerFactory;

lazy_static! {
    //主线程运行状态和线程无条件休眠超时时长
    static ref MAIN_RUN_STATUS: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    static ref MAIN_UNCONDITION_SLEEP_TIMEOUT: u64 = 1;

    //主线程条件变量和线程条件休眠超时时长
    static ref MAIN_CONDVAR: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
    static ref MAIN_CONDITION_SLEEP_TIMEOUT: u64 = 10000;

    //初始化主线程异步运行时
    static ref MAIN_ASYNC_RUNNER: SingleTaskRunner<()> = SingleTaskRunner::new();
    static ref MAIN_ASYNC_RUNTIME: SingleTaskRuntime<()> = MAIN_ASYNC_RUNNER.startup().unwrap();

    //初始化文件异步运行时
    static ref FILES_ASYNC_RUNTIME: MultiTaskRuntime<()> = {
        let pool = MultiTaskPool::new("PI-SERV-FILE".to_string(), get_physical(), 2 * 1024 * 1024, 10, Some(10));
        pool.startup(false)
    };

    //工作虚拟机运行状态和条件变量表、线程条件休眠超时时长和线程无条件休眠超时时长
    static ref WORK_VM_CONDVAR_TABLE: Arc<RwLock<XHashMap<usize, Arc<AtomicBool>>>> = Arc::new(RwLock::new(XHashMap::default()));
    static ref WORK_VM_UNCONDITION_SLEEP_TIMEOUT: u64 = 1;
}

/*
* 同步执行入口，退出时会中止主线程
*/
fn main() {
    //初始化日志服务器
    env_logger::init();

    //匹配启动时的选项和参数
    let matches = App::new("Pi Serv Main")
        .version("0.2.0")
        .author("YiNeng <yineng@foxmail.com>")
        .arg(
            Arg::with_name("INIT_HEAP_SIZE") //虚拟机初始堆大小
                .short("I")
                .long("INIT_HEAP_SIZE")
                .value_name("Mbytes")
                .help("Set init vm heap size")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("MAX_HEAP_SIZE") //虚拟机最大堆大小
                .short("H")
                .long("MAX_HEAP_SIZE")
                .value_name("Mbytes")
                .help("Set max vm heap size")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("WORK_VM_MULTIPLE") //工作虚拟机倍数
                .short("W")
                .long("WORK_VM_MULTIPLE")
                .value_name("Multiple")
                .help("Set multiple of work vm amount")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("DEBUG") //工作虚拟机调试模式
                .short("D")
                .long("DEBUG")
                .value_name("Port")
                .help("Enable debug work vm on port")
                .takes_value(true),
        )
        .get_matches();

    //初始化V8环境，并启动初始虚拟机
    let (init_heap_size, max_heap_size, debug_port) = init_v8_env(&matches);
    let init_vm = create_init_vm(init_heap_size, max_heap_size, debug_port);

    //主线程循环
    let matches_copy = matches.clone();
    let init_vm_copy = init_vm.clone();
    if let Err(e) = MAIN_ASYNC_RUNTIME.spawn(MAIN_ASYNC_RUNTIME.alloc(), async move {
        async_main(
            matches_copy,
            init_vm_copy,
            init_heap_size,
            max_heap_size,
            debug_port,
        )
        .await;
    }) {
        panic!("Spawn async main task failed, reason: {:?}", e);
    }
    while MAIN_RUN_STATUS.load(Ordering::Relaxed) {
        //推动主线程异步运行时
        if let Err(e) = MAIN_ASYNC_RUNNER.run() {
            panic!("Main loop failed, reason: {:?}", e);
        }

        //推动初始虚拟机
        init_vm.run();

        if MAIN_ASYNC_RUNTIME.len() == 0 {
            //当前没有主线程任务，则休眠主线程
            let (lock, condvar) = &**MAIN_CONDVAR;
            let mut is_sleep = lock.lock();
            if !*is_sleep {
                //如果当前未休眠，则休眠
                *is_sleep = true;
                if condvar
                    .wait_for(
                        &mut is_sleep,
                        Duration::from_millis(*MAIN_CONDITION_SLEEP_TIMEOUT),
                    )
                    .timed_out()
                {
                    //条件超时唤醒，则设置状态为未休眠
                    *is_sleep = false;
                }
            }
        } else {
            //当前主线程有任务，则休眠指定时长后继续执行主线程任务
            thread::sleep(Duration::from_millis(*MAIN_UNCONDITION_SLEEP_TIMEOUT));
        }
    }
}

//初始化V8环境，如果是调试模式则返回调试端口
fn init_v8_env(matches: &ArgMatches) -> (usize, usize, Option<u16>) {
    let mut init_heap_size = 16 * 1024 * 1024; //默认虚拟机初始堆大小为16MB
    if let Some(size) = matches.value_of("INIT_HEAP_SIZE") {
        match size.parse::<usize>() {
            Err(e) => panic!("Init v8 env failed, reason: {:?}", e),
            Ok(num) => {
                if num.is_power_of_two() {
                    init_heap_size = num * 1024 * 1024;
                }
            }
        }
    }

    let mut max_heap_size = 8096 * 1024 * 1024; //默认虚拟机最大堆大小为8GB
    if let Some(size) = matches.value_of("MAX_HEAP_SIZE") {
        match size.parse::<usize>() {
            Err(e) => panic!("Init v8 env failed, reason: {:?}", e),
            Ok(num) => {
                if num.is_power_of_two() {
                    max_heap_size = num * 1024 * 1024;
                }
            }
        }
    }

    let mut debug_port: u16 = 0;
    if let Some(value) = matches.value_of("DEBUG") {
        match value.parse::<u16>() {
            Err(e) => {
                panic!("Bind debug listene port failed, reason: {:?}", e);
            }
            Ok(port) => {
                if port > 1024 {
                    debug_port = port;
                } else {
                    panic!(
                        "Bind debug listene port failed, port: {}, reason: invalid port",
                        port
                    );
                }
            }
        }
    }

    init_v8(Some(vec![
        "".to_string(),
        "--no-wasm-async-compilation".to_string(),
        "--harmony-top-level-await".to_string(),
        "--expose-gc".to_string(),
    ]));

    if debug_port > 0 {
        //启动虚拟机调试用Websocket服务
        let ws_server_factory = Arc::new(debug::InspectorWebsocketServerFactory::new::<PathBuf>(
            "", None,
        )); // TODO: 浏览器的调试不能指定路径，但是vscode的调试需要指定路径
        let mut factory = AsyncPortsFactory::<TcpSocket>::new();
        factory.bind(
            debug_port,
            Box::new(
                WebsocketListenerFactory::<TcpSocket>::with_protocol_factory(ws_server_factory),
            ),
        );
        let mut config = SocketConfig::new("0.0.0.0", factory.bind_ports().as_slice());
        config.set_option(16384, 16384, 16384, 16);
        let buffer = WriteBufferPool::new(1000, 10, 3).ok().unwrap();
        match SocketListener::bind_with_processor(
            factory,
            buffer,
            config,
            1,
            1024,
            2 * 1024 * 1024,
            1024,
            Some(10),
        ) {
            Err(e) => {
                panic!(
                    "Bind debug listene port failed, port: {}, reason: {:?}",
                    debug_port, e
                );
            }
            Ok(_) => {
                info!("Bind debug listene port ok, port: {}", debug_port);
            }
        }
    }

    info!("Init v8 env ok");
    (init_heap_size, max_heap_size, Some(debug_port))
}

//创建初始虚拟机
fn create_init_vm(init_heap_size: usize, max_heap_size: usize, debug_port: Option<u16>) -> vm::Vm {
    let mut builder = vm::VmBuilder::new().snapshot_template();
    builder = builder.heap_limit(init_heap_size, max_heap_size);

    if debug_port.is_some() {
        //允许调试
        builder = builder.enable_inspect();
    }

    builder.build()
}

/*
* 异步执行入口，退出时不会中止主线程
*/
async fn async_main(
    matches: ArgMatches<'static>,
    init_vm: vm::Vm,
    init_heap_size: usize,
    max_heap_size: usize,
    debug_port: Option<u16>,
) {
    let snapshot_context = init_snapshot(&init_vm).await;

    //TODO 加载项目的入口模块文件, 并加载其静态依赖树中的所有js模块文件

    finish_snapshot(&init_vm, snapshot_context).await;

    let workers = init_work_vm(
        &matches,
        &init_vm,
        init_heap_size,
        max_heap_size,
        debug_port,
    );
    workers[0]
        .new_context(None, workers[0].alloc_context_id(), None)
        .await
        .unwrap();
}

//初始化快照
async fn init_snapshot(init_vm: &vm::Vm) -> ContextHandle {
    let snapshot_context = init_vm
        .new_context(None, init_vm.alloc_context_id(), None)
        .await
        .unwrap();
    if let Err(e) = init_vm
        .execute(
            snapshot_context,
            "Init_Vm_Init_module.js",
            r#"
                    onerror = function(e) {
                        console.log("catch global error, e:", e.stack);
                    };
                "#,
        )
        .await
    {
        panic!("!!!!!!Init snapshot failed, reason: {:?}", e);
    }
    info!("Init snapshot ok");

    snapshot_context
}

//完成快照
async fn finish_snapshot(init_vm: &vm::Vm, snapshot_context: ContextHandle) {
    if let Err(e) = init_vm.snapshot(snapshot_context).await {
        panic!("!!!!!!Finish snapshot failed, reason: {:?}", e);
    }
    info!("Snapshot finish");
}

//初始化工作虚拟机，返回工作虚拟机
fn init_work_vm(
    matches: &ArgMatches,
    init_vm: &vm::Vm,
    init_heap_size: usize,
    max_heap_size: usize,
    debug_port: Option<u16>,
) -> Vec<vm::Vm> {
    let mut work_vm_count: usize = 2 * get_physical(); //默认工作虚拟机数量为本地cpu物理核数的2倍
    if let Some(value) = matches.value_of("WORK_VM_MULTIPLE") {
        match value.parse::<usize>() {
            Err(e) => {
                panic!("Init work vm failed, reason: {:?}", e);
            }
            Ok(count) => {
                work_vm_count = get_physical() * count;
            }
        }
    }

    let mut vec = Vec::with_capacity(work_vm_count);
    let snapshot = VmStartupSnapshot::Snapshot(init_vm.get_snapshot().unwrap());
    let snapshot_bytes = snapshot.bytes();
    for index in 0..work_vm_count {
        //使用指定快照，创建工作虚拟机
        let work_vm_snapshot = VmStartupSnapshot::Boxed(snapshot_bytes.to_vec().into_boxed_slice());
        let mut builder = vm::VmBuilder::new().startup_snapshot(work_vm_snapshot);
        builder = builder.heap_limit(init_heap_size, max_heap_size);

        if debug_port.is_some() {
            //允许调试
            builder = builder.enable_inspect();
        }
        let work_vm = builder.build();

        //注册工作虚拟机
        let work_vm_copy = work_vm.clone();
        WORK_VM_CONDVAR_TABLE
            .write()
            .insert(work_vm_copy.get_vid(), Arc::new(AtomicBool::new(true)));
        vec.push(work_vm_copy);

        //启动工作线程，并运行工作虚拟机
        if let Err(e) = thread::Builder::new()
            .name("PI-SERV-WORKER".to_string() + index.to_string().as_str())
            .stack_size(2 * 1024 * 1024)
            .spawn(move || {
                work_vm_loop(work_vm, index);
            })
        {
            panic!("Init work vm failed, reason: {:?}", e);
        }
    }

    vec
}

//工作虚拟机线程循环
fn work_vm_loop(work_vm: vm::Vm, index: usize) {
    let work_vm_vid = work_vm.get_vid();
    let mut worker_run_status = None;
    if let Some(status) = WORK_VM_CONDVAR_TABLE.read().get(&work_vm_vid) {
        //当前工作虚拟机已注册
        worker_run_status = Some(status.clone());
    }

    if let Some(worker_run_status) = worker_run_status {
        info!(
            "Worker ready, thread: {}, worker: {}",
            "PI-SERV-WORKER".to_string() + index.to_string().as_str(),
            "Vm-".to_string() + work_vm_vid.to_string().as_str()
        );

        let sleep_timeout = (*WORK_VM_UNCONDITION_SLEEP_TIMEOUT) as u128;
        while worker_run_status.load(Ordering::Relaxed) {
            //推动工作虚拟机
            let runed_time = work_vm.run().as_millis();
            if runed_time < sleep_timeout {
                thread::sleep(Duration::from_millis((sleep_timeout - runed_time) as u64));
            }
        }
    }
}
