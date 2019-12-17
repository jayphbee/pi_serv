#![feature(fs_read_write)]
#![feature(splice)]
#![feature(exclusive_range_pattern)]
#![feature(unboxed_closures)]
#![feature(vec_remove_item)]
#![feature(nll)]
#[warn(dead_code)]
extern crate clap;
extern crate core;
extern crate fnv;
extern crate json;
extern crate magnetic;
extern crate mqtt_tmp;
extern crate net;
extern crate nodec;
extern crate pi_crypto;
extern crate pi_db;
extern crate pi_vm;
extern crate rand;
extern crate rpc_tmp;
extern crate toml;
// extern crate pi_p2p;
extern crate apm;
extern crate atom;
extern crate base;
extern crate bon;
extern crate file;
extern crate gray;
extern crate guid;
extern crate handler;
extern crate hash_value;
extern crate httpc;
extern crate https;
extern crate libc;
extern crate mqtt;
extern crate mqtt3;
extern crate ordmap;
extern crate pi_store;
extern crate rpc;
extern crate sinfo;
extern crate tcp;
extern crate time;
extern crate timer;
extern crate util as lib_util;
extern crate worker;
extern crate ws;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate env_logger;

#[cfg(any(unix))]
extern crate glob;

pub mod depend;
pub mod hotfix;
pub mod init_js;
pub mod js_async;
pub mod js_base;
pub mod js_db;
pub mod js_env;
pub mod js_file;
pub mod js_httpc;
pub mod js_lib;
pub mod js_net;
pub mod js_net_rpc_client;
pub mod js_vm;
pub mod jsloader;
pub mod util;
pub mod webshell;

mod def_build;
mod js_util;
mod pi_crypto_build;
mod pi_db_build;
mod pi_lib_gray_build;
mod pi_lib_guid_build;
mod pi_lib_sinfo_build;
mod pi_math_hash_build;
mod pi_net_mqtt_build;
mod pi_net_net_build;
mod pi_net_rpc_build;
mod pi_serv_build;
mod pi_vm_build;
// mod pi_p2p_build;
mod pi_net_httpc_build;
mod pi_net_https_build;
mod pi_net_rpc_tmp_build;
mod pi_store_build;

use std::env;
use std::io;
use std::io::{Result as IOResult, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use clap::{App, Arg};
#[cfg(not(unix))]
use pi_vm::adapter::load_lib_backtrace;
use pi_vm::adapter::{
    register_global_vm_heap_collect_timer, register_native_object, set_vm_timeout,
};
use pi_vm::adapter::{JSType, JS};
use pi_vm::bonmgr::{jstype_ptr, ptr_jstype, BonMgr, CallResult, FnMeta, BON_MGR};
use pi_vm::pi_vm_impl::push_callback;
use pi_vm::shell::SHELL_MANAGER;

use atom::Atom;
use time::now_millisecond;
use timer::TIMER;
use worker::impls::{
    JS_TASK_POOL, JS_WORKER_WALKER, NET_TASK_POOL, NET_WORKER_WALKER, STORE_TASK_POOL,
    STORE_WORKER_WALKER, TASK_POOL_TIMER,
};
use worker::worker::WorkerType;
use worker::worker_pool::WorkerPool;

use init_js::init_js;
use js_base::IS_END;
use util::read_file_list;

use js_env::{current_dir, env_var, set_current_dir, set_env_var};

use apm::allocator::{get_max_alloced_limit, set_max_alloced_limit, CounterSystemAllocator};
use apm::common::SysStat;

#[global_allocator]
static ALLOCATOR: CounterSystemAllocator = CounterSystemAllocator;

#[cfg(any(windows))]
fn args() -> clap::ArgMatches<'static> {
    let matches = App::new("pi_server")
        .version("1.0")
        .author("test. <test@gmail.com>")
        .about("aboutXXXX")
        .arg(
            Arg::with_name("shell")
                .short("s")
                .long("shell")
                .value_name("BOOL")
                .takes_value(true)
                .help("Open the console at startup"),
        )
        .arg(
            Arg::with_name("max_heap")
                .short("H")
                .long("max_heap")
                .value_name("GByte")
                .takes_value(true)
                .help("Max heap limit on runtime"),
        )
        .arg(
            Arg::with_name("init_file")
                .short("e")
                .long("init")
                .value_name("INIT_FILE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("projects")
                .short("p")
                .long("projects")
                .multiple(true)
                .value_name("PROJECTS")
                .takes_value(true),
        )
        .get_matches_from(wild::args());
    matches
}

#[cfg(any(unix))]
fn args() -> clap::ArgMatches<'static> {
    let matches = App::new("pi_server")
        .version("1.0")
        .author("test. <test@gmail.com>")
        .about("aboutXXXX")
        .arg(
            Arg::with_name("shell")
                .short("s")
                .long("shell")
                .value_name("BOOL")
                .takes_value(true)
                .help("Open the console at startup"),
        )
        .arg(
            Arg::with_name("max_heap")
                .short("H")
                .long("max_heap")
                .value_name("GByte")
                .takes_value(true)
                .help("Max heap limit on runtime"),
        )
        .arg(
            Arg::with_name("init_file")
                .short("e")
                .long("init")
                .value_name("INIT_FILE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("projects")
                .short("p")
                .long("projects")
                .multiple(true)
                .value_name("PROJECTS")
                .takes_value(true),
        )
        .get_matches();
    matches
}

fn main() {
    env_logger::builder().format_timestamp_millis().init();
    #[cfg(not(unix))]
    load_lib_backtrace();
    TIMER.run();
    TASK_POOL_TIMER.run();
    register_native_object();
    let sys = SysStat::new();
    let processor = sys.processor_count();
    let worker_pool0 = Box::new(WorkerPool::new(
        "JS Worker".to_string(),
        WorkerType::Js,
        processor * 2,
        1024 * 1024,
        10000,
        JS_WORKER_WALKER.clone(),
    ));
    worker_pool0.run(JS_TASK_POOL.clone());

    let worker_pool1 = Box::new(WorkerPool::new(
        "Store Worker".to_string(),
        WorkerType::Store,
        processor,
        1024 * 1024,
        10000,
        STORE_WORKER_WALKER.clone(),
    ));
    worker_pool1.run(STORE_TASK_POOL.clone());

    let worker_pool = Box::new(WorkerPool::new(
        "Network Worker".to_string(),
        WorkerType::Net,
        processor,
        1024 * 1024,
        30000,
        NET_WORKER_WALKER.clone(),
    ));
    worker_pool.run(NET_TASK_POOL.clone());

    pi_crypto_build::register(&BON_MGR);
    pi_math_hash_build::register(&BON_MGR);
    pi_db_build::register(&BON_MGR);
    pi_lib_guid_build::register(&BON_MGR);
    pi_lib_gray_build::register(&BON_MGR);
    pi_lib_sinfo_build::register(&BON_MGR);
    pi_db_build::register(&BON_MGR);
    def_build::register(&BON_MGR);
    pi_net_mqtt_build::register(&BON_MGR);
    pi_net_rpc_build::register(&BON_MGR);
    pi_net_net_build::register(&BON_MGR);
    pi_serv_build::register(&BON_MGR);
    pi_vm_build::register(&BON_MGR);
    js_async::register(&BON_MGR);
    // pi_p2p_build::register(&BON_MGR);
    pi_net_httpc_build::register(&BON_MGR);
    pi_net_https_build::register(&BON_MGR);
    pi_net_rpc_tmp_build::register(&BON_MGR);
    pi_store_build::register(&BON_MGR);
    register(&BON_MGR);

    let matches = args();

    if let Some(max_heap) = matches.value_of("max_heap") {
        match max_heap.parse::<f64>() {
            Err(e) => {
                panic!("set max heap limit failed, reason: {:?}", e);
            }
            Ok(val) => {
                set_max_alloced_limit((val * 1024.0 * 1024.0 * 1024.0).floor() as usize);
                info!(
                    "===> Set Max Heap Limit Ok, size: {} B",
                    get_max_alloced_limit()
                );
            }
        }
    } else {
        //没有设置最大堆限制
        #[cfg(any(windows))]
        {
            let (total_memory, _, _, _, _, _) = sys.memory_usage();
            set_max_alloced_limit(((total_memory * 1024) as f64 * 0.75).floor() as usize);
            info!(
                "===> Set Max Heap Limit Ok, size: {} B",
                get_max_alloced_limit()
            );
        }
        #[cfg(any(unix))]
        {
            let sys = sys.special_platform().unwrap();
            match sys.sys_virtual_memory_detal() {
                None => {
                    //获取内存占用信息失败，则使用默认最大堆限制
                    warn!(
                        "!!!> Set Max Heap Limit Failed, used default max heap limit, size: {}B",
                        get_max_alloced_limit()
                    );
                }
                Some(info) => {
                    //获取内存占用信息成功
                    let total_memory = info.0;
                    set_max_alloced_limit((total_memory as f64 * 0.75).floor() as usize);
                    info!(
                        "===> Set Max Heap Limit Ok, size: {} B",
                        get_max_alloced_limit()
                    );
                }
            }
        }
    }

    let init_js_path = matches.value_of("init_file").unwrap_or("./init.js");
    let projs = match matches.values_of("projects") {
        Some(p) => p
            .map(|s| s.to_string().replace("\\", "/"))
            .collect::<Vec<String>>(),
        None => vec![],
    };

    let current_dir = env::current_dir().unwrap();
    let current_dir_parent =current_dir.parent().unwrap().to_str().unwrap();

    let path = Path::new(init_js_path)
        .iter()
        .filter_map(|x| if x == "." || x == ".." { None } else { Some(x) })
        .map(|x| x.to_str().unwrap())
        .collect::<Vec<&str>>();

    let root: PathBuf = [vec![current_dir_parent], path].concat().iter().collect();
    let project_root = root.parent().unwrap().parent().unwrap().to_str().unwrap().replace("\\", "/");

    let cur_dir = current_dir.to_str().unwrap();
    let mut cur_exe = env::current_exe().unwrap();
    cur_exe.pop();

    set_env_var("ENV_CORE_PATH", cur_exe.to_str().unwrap());
    set_env_var("PROJECTS", &projs.as_slice().join(" "));
    set_env_var("PROJECT_ROOT", &project_root);
    let pipt_root = Path::new(cur_dir).parent().unwrap().to_str().unwrap();
    set_env_var("PIPT_ROOT", &pipt_root.replace("\\", "/"));

    exec_js(init_js_path.to_string());

    if let Some(root) = matches.value_of("root") {
        let mut root = root.to_string();
        if !root.ends_with("/") {
            root += "/";
        }
        if root.starts_with("./") {
            root = root[2..].to_string();
        }

        let r_len = root.len();
        let list: Vec<String> = collect(
            root.clone(),
            match matches.values_of("list") {
                Some(r) => r.collect(),
                None => Vec::default(),
            },
        );
        println!("list: {:?}", list);
        let mut files: Vec<String> = Vec::default();
        for e in list.iter() {
            println!("e:{:}", e);
            let r = e.replace("\\", "/");
            println!("r:{}", r);
            let mut r = r.as_str();
            if r.starts_with("./") {
                r = &r[2..];
            }
            files.push(r[r_len..].to_string())
        }

        // let file_list = read_file_list( &Path::new(&(root.clone() + ".depend")).to_path_buf());
        // if files.len() == 0{
        // 	init_js(&[root.clone()], file_list, root.clone());
        // }else{
        // 	init_js(&files[..], file_list, root.clone());
        // }
    }

    if let Some(path) = matches.value_of("exec") {
        exec_js(path.to_string());
    }

    // 启动http服务器
    start_simple_https(&matches);

    //启动全局虚拟机堆整理
    set_vm_timeout(60000);
    register_global_vm_heap_collect_timer(3000);

    match matches.value_of("shell") {
        Some("true") => {
            let (req_sender, req_receiver) = channel();
            let (resp_sender, resp_receiver) = channel();

            let req_sender_copy = req_sender.clone();
            let resp = Arc::new(
                move |result: IOResult<Arc<Vec<u8>>>, req: Option<Box<FnOnce(Arc<Vec<u8>>)>>| {
                    resp_sender.send(result);
                    req_sender.send(req);
                },
            );

            let s = SHELL_MANAGER.write().unwrap().open();
            if let Some(shell) = s {
                let req = SHELL_MANAGER.write().unwrap().connect(shell, resp.clone());
                if req.is_none() {
                    eprintln!("Connect Error");
                }
                req_sender_copy.send(req);

                println!("Shell v0.1");

                let mut req: Option<Box<dyn FnOnce(Arc<Vec<u8>>)>> = None;
                loop {
                    print!(">");
                    io::stdout().flush();

                    let mut buffer = String::new();
                    while let Err(e) = io::stdin().read_line(&mut buffer) {
                        eprintln!("Input Error, {:?}", e);
                        print!(">");
                        io::stdout().flush();
                    }

                    if buffer.trim().as_bytes() == b"exit" {
                        println!("Shell closed");
                        return;
                    }

                    if let None = req {
                        //当前没有请求回调，则接收请求回调
                        match req_receiver.recv() {
                            Err(e) => {
                                eprintln!("Shell Suspend, {:?}", e);
                                return;
                            }
                            Ok(new) => {
                                if new.is_none() {
                                    println!("Shell closed");
                                    return;
                                }
                                req = new; //更新请求回调
                            }
                        }
                    }

                    if let Some(r) = req.take() {
                        r(Arc::new(buffer.into_bytes()));
                    }

                    //接收请求响应
                    match resp_receiver.recv() {
                        Err(e) => eprintln!("Output Error, {:?}", e),
                        Ok(result) => match result {
                            Err(e) => eprintln!("{:?}", e),
                            Ok(r) => println!(
                                "{output}",
                                output = String::from_utf8_lossy(&r[..]).as_ref()
                            ),
                        },
                    }
                }
            }
        }
        _ => {
            while !IS_END.lock().unwrap().0 {
                println!("###############loop, {}", now_millisecond());
                thread::sleep(Duration::from_millis(10000));
            }
        }
    }

    // loop {
    //     println!("###############loop, {}", now_millisecond());
    //     thread::sleep(Duration::from_millis(60000));
    // }
}

#[cfg(any(unix))]
fn collect(root: String, list: Vec<&str>) -> Vec<String> {
    let mut vec = Vec::with_capacity(list.len());

    for p in list {
        let mut buf = PathBuf::from(&root);
        buf.push(p);
        match glob::glob(buf.as_path().to_str().unwrap()) {
            Err(e) => panic!("collect list args failed, path: {:?}, reason: {:?}", p, e),
            Ok(paths) => {
                for path in paths {
                    match path {
                        Err(ref e) => panic!(
                            "collect list args failed, path: {:?}, reason: {:?}",
                            path, e
                        ),
                        Ok(r) => {
                            if let Ok(x) = r.into_os_string().into_string() {
                                vec.push(x);
                            }
                        }
                    }
                }
            }
        }
    }

    vec
}

#[cfg(any(windows))]
fn collect(_: String, list: Vec<&str>) -> Vec<String> {
    list.iter().map(|path| path.to_string()).collect()
}

/**
* 同步的设置定时异步回调
* @param ms 间隔的时长，单位毫秒
* @param cb 异步回调
* @returns 返回定时任务的编号
*/
fn call_3344344275_async(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in set_timeout";

    // timeout
    let timeout = &v[0];
    if !timeout.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let timeout = timeout.get_u32();

    // call_index
    let call_index = &v[1];
    if !call_index.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let call_index = call_index.get_u32();

    let jscopy = js.clone();

    println!("index: {}, timeout: {}", call_index, timeout);
    match push_callback(
        jscopy.clone(),
        call_index,
        Box::new(move |js: Arc<JS>| {
            let ptr = Box::into_raw(Box::new(js.clone())) as usize;
            ptr_jstype(js.get_objs(), js.clone(), ptr, 2884638791);
            1
        }),
        Some(timeout),
        Atom::from("call_3344344275_async1"),
    ) {
        Some(r) => js.new_i32(r as i32),
        None => js.new_undefined(),
    };
    //     let result = js_base::set_timeout(jst0,jst1,jst2,jst3,Box::new(call_back));let mut result = match result{
    //         Some(v) => { let mut v = js.new_i32(v as i32);
    //  v}
    //         None => js.new_null()
    //     };

    Some(CallResult::Ok)
}

fn register(mgr: &BonMgr) {
    mgr.regist_fun_meta(FnMeta::CallArg(call_3344344275_async), 3344344275);
}

// import {HttpsCfg, HttpsTlsCfg} from "./server_cfg.s";
// import { cfgMgr } from "../../pi/util/cfg";
// import { startHttpMount, startHttpsMount } from "../rust/https/https_impl";
// import { Mount } from "../rust/https/mount";
// import { StaticFile } from "../rust/https/file";
// import { StaticFileBatch } from "../rust/https/files";
// import { FileUpload } from "../rust/https/upload";

use https::batch::FileBatch;
use https::file::StaticFile;
use https::files::StaticFileBatch;
use https::https_impl::start_http;
use https::mount::Mount;
use https::upload::FileUpload;

// 启动http服务器
fn start_simple_https(matches: &clap::ArgMatches<'static>) {
    if let Some(m) = matches.value_of("mod") {
        let m = m.to_string();
        if m == "httpServer" {
            let port = match matches.value_of("httpServerPort") {
                Some(r) => match u16::from_str(r) {
                    Ok(r) => r,
                    Err(e) => {
                        println!("httpServer port error: {:?}, r: {}", e, r);
                        return;
                    }
                },
                None => 80,
            };

            let down_root = match matches.value_of("httpServerLoadRoot") {
                Some(r) => r,
                None => "./",
            };

            let upload_root = match matches.value_of("httpServerUploadRoot") {
                Some(r) => r,
                None => "./",
            };

            let mut mount = Mount::new();
            // addGenHead(staticFile,r.gen_head);
            // addGenHead(staticFileBatch,r.gen_head);
            mount.mount("/", StaticFile::new(down_root));
            mount.mount("/files", StaticFileBatch::new(down_root));
            mount.mount("/batch", FileBatch::new(down_root));
            mount.mount("/upload", FileUpload::new(upload_root));
            start_http(mount, Atom::from("0.0.0.0"), port, 30 * 1000, 20 * 1000);
        }
    }

    // let httpsTlsCfgs = cfgMgr.get(HttpsTlsCfg._$info.name);
    // if(httpsTlsCfgs){
    //     httpsTlsCfgs.forEach((r: HttpsTlsCfg,_k) => {
    //         let mount = Mount.new();
    //         let staticFile = StaticFile.newString(r.root);
    //         let staticFileBatch = StaticFileBatch.newString(r.root);
    //         addGenHead(staticFile,r.gen_head);
    //         addGenHead(staticFileBatch,r.gen_head);
    //         mount.mountStaticFile("/", staticFile);
    //         mount.mountStaticFileBatch("/files", staticFileBatch);
    //         if(r.uploadRoot){
    //             mount.mountFileUpload("/upload",  FileUpload.newString(r.uploadRoot));
    //         }
    //         startHttpsMount(mount, r.ip, r.port, r.keep_alive_timeout, r.handle_timeout, r.certPath, r.keyPath);
    //     });
    // }
}

fn exec_js(path: String) {
    use guid::GuidGen;
    use init_js::bind_global;
    use pi_db::memery_db::DB;
    use pi_db::mgr::Mgr;
    use pi_vm::duk_proc::{DukProcess, DukProcessFactory};
    use pi_vm::proc_pool::{
        close_process, name_send, name_to_pid, pid_send, set_catcher, set_factory, set_receiver,
        spawn_process,
    };

    let path = path.as_str().replace("\\", "/");
    let cur_exe = env::current_exe().unwrap();
    let auth = Arc::new(NativeObjsAuth::new(None, None));

    let mgr = Mgr::new(GuidGen::new(0, 0)); //创建数据库管理器
    mgr.register(Atom::from("memory"), Arc::new(DB::new())); //注册一个内存数据库

    // use js_vm::{get_byte_code, compile, load_module};
    use pi_vm::bonmgr::NativeObjsAuth;
    let js = JS::new(1, Atom::from("compile"), auth.clone(), None).unwrap();

    // 初始化js执行环境
    let env_code = read_code(&cur_exe.join("../env.js"));
    let core_code = read_code(&cur_exe.join("../core.js"));

    let env_code = js.compile("env.js".to_string(), env_code).unwrap();
    let core_code = js.compile("core.js".to_string(), core_code).unwrap();

    load_code(&js, env_code.as_slice());
    load_code(&js, core_code.as_slice());

    //创建一个进程工厂
    let duk_facotry_name = Atom::from("duk");
    let duk_factory = DukProcessFactory::new(
        duk_facotry_name.clone(),
        auth,
        Arc::new(vec![env_code, core_code]),
    );
    set_factory(Atom::from("duk"), Arc::new(duk_factory));

    let global_code = bind_global(&mgr, &js);
    js.load(&global_code);

    //////////////
    //调用全局变量定义函数， 定义全局变量_$mgr
    js.get_js_function("_$defineGlobal".to_string());
    js.new_str(String::from("_$db_mgr"));
    let ptr = Box::into_raw(Box::new(mgr.clone())) as usize;
    ptr_jstype(js.get_objs(), js.clone(), ptr, 2976191628); //new native obj作为参数
    js.call(2);

    //调用全局变量定义函数， 定义全局变量 _$depend
    use depend::Depend;
    let dp = Depend::new_sample(vec![]);
    js.get_js_function("_$defineGlobal".to_string());
    js.new_str(String::from("_$depend"));
    let ptr = Box::into_raw(Box::new(dp)) as usize;
    ptr_jstype(js.get_objs(), js.clone(), ptr, 1797798710); //new native obj作为参数
    js.call(2);

    //////////////
    let cur_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    let cur_dir = cur_dir.as_str().replace("\\", "/") + "/a";
    if js.get_link_function("Module.require".to_string()) {
        js.new_str(path).unwrap();
        js.new_str(cur_dir).unwrap();
        js.call(2);
    } else {
        panic!("Module.require function is not exist");
    }
}

use std::fs::File;
use std::io::Read;
fn read_code(path: &PathBuf) -> String {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {:?} exception:{}", path, e),
    };
    let mut str_val = String::new();
    if let Err(e) = file.read_to_string(&mut str_val) {
        panic!("Error Reading file: {}", e)
    }
    return str_val;
}

fn load_code(js: &Arc<JS>, code: &[u8]) -> bool {
    loop {
        if js.is_ran() {
            return js.load(&code);
        }
        pi_vm::adapter::pause();
    }
}