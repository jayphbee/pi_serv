#![feature(fs_read_write)]
#![feature(splice)]
#![feature(generic_associated_types)]
#![feature(unboxed_closures)]
#![feature(vec_remove_item)]
#![feature(nll)]
#[warn(dead_code)]
extern crate clap;
extern crate json;
extern crate toml;
extern crate pi_vm;
extern crate pi_crypto;
extern crate pi_db;
extern crate core;
extern crate fnv;
extern crate net;
extern crate nodec;
extern crate mqtt_tmp;
extern crate rpc_tmp;
extern crate magnetic;
extern crate rand;
// extern crate pi_p2p;
extern crate mqtt3;
extern crate httpc;
extern crate https;
extern crate tcp;
extern crate ws;
extern crate mqtt;
extern crate base;
extern crate rpc;
extern crate atom;
extern crate handler;
extern crate worker;
extern crate bon;
extern crate file;
extern crate gray;
extern crate guid;
extern crate util as lib_util;
extern crate sinfo;
extern crate hash_value;
extern crate timer;
extern crate ordmap;
extern crate apm;
extern crate pi_store;
extern crate libc;
extern crate time;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate env_logger;

#[cfg(any(unix))]
extern crate glob;

pub mod jsloader;
pub mod depend;
pub mod init_js;
pub mod util;
pub mod js_httpc;
pub mod js_db;
pub mod js_net;
pub mod js_net_rpc_client;
pub mod js_base;
pub mod js_lib;
pub mod js_async;
pub mod hotfix;
pub mod webshell;

mod js_util;
mod pi_crypto_build;
mod pi_math_hash_build;
mod pi_db_build;
mod pi_lib_guid_build;
mod pi_lib_gray_build;
mod pi_lib_sinfo_build;
mod def_build;
mod pi_net_mqtt_build;
mod pi_net_net_build;
mod pi_net_rpc_build;
mod pi_serv_build;
mod pi_vm_build;
// mod pi_p2p_build;
mod pi_net_httpc_build;
mod pi_net_https_build;
mod pi_store_build;
mod pi_net_rpc_tmp_build;

use std::thread;
use std::time::Duration;
use std::path::{Path, PathBuf};
use std::io;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::io::{Write, Result as IOResult};
use std::str::{FromStr};

#[cfg(not(unix))]
use pi_vm::adapter::load_lib_backtrace;
use pi_vm::adapter::{register_native_object, set_vm_timeout, register_global_vm_heap_collect_timer};
use pi_vm::shell::SHELL_MANAGER;
use pi_vm::bonmgr::{BON_MGR, BonMgr, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use pi_vm::pi_vm_impl::push_callback;
use clap::{Arg, App};

use time::now_millisecond;
use worker::worker_pool::WorkerPool;
use worker::impls::{TASK_POOL_TIMER, JS_TASK_POOL, STORE_TASK_POOL, NET_TASK_POOL, JS_WORKER_WALKER, STORE_WORKER_WALKER, NET_WORKER_WALKER};
use timer::TIMER;
use worker::worker::WorkerType;
use atom::Atom;

use init_js::{init_js};
use js_base::IS_END;
use util::{read_file_list};

use apm::common::SysStat;
use apm::allocator::{CounterSystemAllocator, set_max_alloced_limit, get_max_alloced_limit};

#[global_allocator]
static ALLOCATOR: CounterSystemAllocator = CounterSystemAllocator;

#[cfg(any(windows))]
fn args() -> clap::ArgMatches<'static> {
	let matches = App::new("pi_server")
						.version("1.0")
						.author("test. <test@gmail.com>")
						.about("aboutXXXX")
						.args_from_usage("[root] -r --root <DIR> 'The directory where the dependent file is located'")
                        .args_from_usage("[list] -l --list <PATH>... 'Files or directories to run'")
						.args_from_usage("[mod] -m --mod <MOD>... 'start module(example: -m httpServer)'")
						.args_from_usage("[httpServerPort] -p --httpServerPort <NUMBER>... 'httpServer port'")
						.args_from_usage("[httpServerSingleFile] -s --httpServerSingleFile <DIR>... 'single file download root'")
						.args_from_usage("[httpServerBatchFile] -b --httpServerBatchFile <DIR>... 'batch file download root'")
						.args_from_usage("[httpServerUploadFile] -u --httpServerUploadFile <DIR>... 'file upload root'")
                        .arg(Arg::with_name("shell")
                            .short("s")
                            .long("shell")
                            .value_name("BOOL")
                            .takes_value(true)
                            .help("Open the console at startup"))
						.get_matches_from(wild::args());
	matches
}

#[cfg(any(unix))]
fn args() -> clap::ArgMatches<'static> {
    let matches = App::new("pi_server")
        .version("1.0")
        .author("test. <test@gmail.com>")
        .about("aboutXXXX")
        .args_from_usage("<root> -r --root <DIR> 'The directory where the dependent file is located'")
        .args_from_usage("[list] -l --list <PATH>... 'Files or directories to run'")
        .arg(Arg::with_name("shell")
            .short("s")
            .long("shell")
            .value_name("BOOL")
            .takes_value(true)
            .help("Open the console at startup"))
        .get_matches();
    matches
}

fn main() {
    env_logger::init();
	#[cfg(not(unix))]
    load_lib_backtrace();
    TIMER.run();
    TASK_POOL_TIMER.run();
    register_native_object();
    let sys = SysStat::new();
    let processor = sys.processor_count();
    let worker_pool0 = Box::new(WorkerPool::new("JS Worker".to_string(), WorkerType::Js,  processor * 2, 1024 * 1024, 5000, JS_WORKER_WALKER.clone()));
    worker_pool0.run(JS_TASK_POOL.clone());

    let worker_pool1 = Box::new(WorkerPool::new("Store Worker".to_string(), WorkerType::Store,  processor, 1024 * 1024, 10000, STORE_WORKER_WALKER.clone()));
    worker_pool1.run(STORE_TASK_POOL.clone());

    let worker_pool = Box::new(WorkerPool::new("Network Worker".to_string(), WorkerType::Net,  processor, 1024 * 1024, 30000, NET_WORKER_WALKER.clone()));
    worker_pool.run(NET_TASK_POOL.clone());
    #[cfg(any(windows))]
    {
        let (total_memory, _, _, _, _, _) = sys.memory_usage();
        set_max_alloced_limit(((total_memory * 1024) as f64 * 0.75).floor() as usize);
        println!("===> Set Max Heap Limit Ok, size: {}", get_max_alloced_limit());
    }
    #[cfg(any(unix))]
    {
        let sys = sys.special_platform().unwrap();
        match sys.sys_virtual_memory_detal() {
            None => {
                //获取内存占用信息失败，则使用默认最大堆限制
                println!("!!!> Set Max Heap Limit Failed, used default max heap limit, size: {}", get_max_alloced_limit());
            },
            Some(info) => {
                //获取内存占用信息成功
                let total_memory = info.0;
                set_max_alloced_limit((total_memory as f64 * 0.75).floor() as usize);
                println!("===> Set Max Heap Limit Ok, size: {}", get_max_alloced_limit());
            },
        }
    }
    set_vm_timeout(60000);
    register_global_vm_heap_collect_timer(10000);


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

	let matches = args();

	if let Some(root) = matches.value_of("root") {
		let mut root = root.to_string();
		if !root.ends_with("/"){
			root += "/";
		}
		if root.starts_with("./") {
			root = root[2..].to_string();
		}

		let r_len = root.len();
		let list: Vec<String> = collect(root.clone(), match matches.values_of("list"){
			Some(r) => r.collect(),
			None => Vec::default(),
		});
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

		let file_list = read_file_list( &Path::new(&(root.clone() + ".depend")).to_path_buf());
		if files.len() == 0{
			init_js(&[root.clone()], file_list, root.clone());
		}else{
			init_js(&files[..], file_list, root.clone());
		}
	}

	// 启动http服务器
	start_simple_https(&matches);

    match matches.value_of("shell") {
        Some("true") => {
            let (req_sender, req_receiver) = channel();
            let (resp_sender, resp_receiver) = channel();

            let req_sender_copy = req_sender.clone();
            let resp = Arc::new(move |result: IOResult<Arc<Vec<u8>>>, req: Option<Box<FnOnce(Arc<Vec<u8>>)>>| {
                resp_sender.send(result);
                req_sender.send(req);
            });


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
                            },
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
                        Ok(result) => {
                            match result {
                                Err(e) => eprintln!("{:?}", e),
                                Ok(r) => println!("{output}", output = String::from_utf8_lossy(&r[..]).as_ref()),
                            }
                        }
                    }
                }
            }
        },
        _ => {
            while !IS_END.lock().unwrap().0 {
                println!("###############loop, {}", now_millisecond());
                thread::sleep(Duration::from_millis(10000));
            }
        },
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
                        Err(ref e) => panic!("collect list args failed, path: {:?}, reason: {:?}", path, e),
                        Ok(r) => {
                            if let Ok(x) = r.into_os_string().into_string() {
                                vec.push(x);
                            }
                        },
                    }
                }
            }
        }
    }

    vec
}

#[cfg(any(windows))]
fn collect(_: String, list: Vec<&str>) -> Vec<String> {
    list.iter().map(|path| {
        path.to_string()
    }).collect()
}

/**
* 同步的设置定时异步回调
* @param ms 间隔的时长，单位毫秒
* @param cb 异步回调
* @returns 返回定时任务的编号
*/
fn call_3344344275_async( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

    let param_error = "param error in set_timeout";
	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2884638791, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut Arc<pi_vm::adapter::JS>)}.clone();

	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32();

	let jst2 = &v[2];
	if !jst2.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_u32();

	let jst3 = &v[3];
    if !jst3.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst3 = Atom::from(jst3.get_str());

    let call_index = &v[4];
    if !call_index.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
    let call_index = call_index.get_u32();
    
    let jscopy = js.clone();

	match push_callback(
		jscopy.clone(),
		call_index,
		Box::new(move |js: Arc<JS>| {
			let ptr = Box::into_raw(Box::new(js.clone())) as usize;
			ptr_jstype(js.get_objs(), js.clone(), ptr,2884638791);
			1
		}),
		Some(jst2), 
		Atom::from("call_3344344275_async1")
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

fn register(mgr: &BonMgr){
	mgr.regist_fun_meta(FnMeta::CallArg(call_3344344275_async), 3344344275);
}

// import {HttpsCfg, HttpsTlsCfg} from "./server_cfg.s";
// import { cfgMgr } from "../../pi/util/cfg";
// import { startHttpMount, startHttpsMount } from "../rust/https/https_impl";
// import { Mount } from "../rust/https/mount";
// import { StaticFile } from "../rust/https/file";
// import { StaticFileBatch } from "../rust/https/files";
// import { FileUpload } from "../rust/https/upload";

use https::mount::Mount;
use https::file::{StaticFile};
use https::files::StaticFileBatch;
use https::upload::FileUpload;
use https::https_impl::start_http;

// 启动http服务器
fn start_simple_https(matches: &clap::ArgMatches<'static>){
	if let Some(m) = matches.value_of("mod") {
		let m = m.to_string();
		if m == "httpServer" {
			let port = match matches.value_of("httpServerPort") {
				Some(r) =>  match u16::from_str(r) {
					Ok(r) => r,
					Err(e) => {
						println!("httpServer port error: {:?}, r: {}", e, r);
						return;
					},
				},
				None => 80,
			};

			let file_root = match matches.value_of("httpServerSingleFile") {
				Some(r) => r,
				None => "./",
			};

			let files_root = match matches.value_of("httpServerBatchFile") {
				Some(r) => r,
				None => "./",
			};

			let upload_root = match matches.value_of("httpServerUploadFile") {
				Some(r) => r,
				None => "./",
			};


			let mut mount = Mount::new();
			let static_file = StaticFile::new(file_root);
			let static_file_batch = StaticFileBatch::new(files_root);
			// addGenHead(staticFile,r.gen_head);
			// addGenHead(staticFileBatch,r.gen_head);
			mount.mount("/", static_file);
			mount.mount("/files", static_file_batch);
			mount.mount("/upload",  FileUpload::new(upload_root));
			start_http(mount, Atom::from("0.0.0.0"), port, 30*1000, 20 * 1000);
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