#![feature(fs_read_write)]
#![feature(splice)]
#![feature(generic_associated_types)]
#![feature(fnbox)]
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
extern crate mqtt;
extern crate rpc;
extern crate magnetic;
extern crate rand;
// extern crate pi_p2p;
extern crate mqtt3;
extern crate httpc;
extern crate https;
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
extern crate nodec;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate env_logger;

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
pub mod js_net_rpc_client;
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

use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::path::Path;
use std::io;
use std::sync::Arc;
use std::boxed::FnBox;
use std::sync::mpsc::channel;
use std::io::{Read, Write, Result as IOResult};

#[cfg(not(unix))]
use pi_vm::adapter::load_lib_backtrace;
use pi_vm::adapter::{register_native_object};
use pi_vm::shell::SHELL_MANAGER;
use pi_vm::bonmgr::BON_MGR;
use clap::{Arg, App};

use time::now_millisecond;
use worker::worker_pool::WorkerPool;
use worker::impls::{JS_TASK_POOL, STORE_TASK_POOL, NET_TASK_POOL, JS_WORKER_WALKER, STORE_WORKER_WALKER, NET_WORKER_WALKER};
use timer::TIMER;
use worker::worker::WorkerType;

use init_js::{init_js};
use js_base::IS_END;
use util::{read_file_list};

use apm::common::SysStat;
use apm::allocator::CounterSystemAllocator;
#[global_allocator]
static ALLOCATOR: CounterSystemAllocator = CounterSystemAllocator;

fn args() -> clap::ArgMatches<'static> {
	let matches = App::new("pi_server")
						.version("1.0")
						.author("test. <test@gmail.com>")
						.about("aboutXXXX")
						.arg(Arg::with_name("config")
							.short("c")
							.long("config")
							.value_name("FILE")
							.help("config path")
							.takes_value(true))
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
    register_native_object();
    let sys = SysStat::new();
    let processor = sys.processor_count();
    let worker_pool0 = Box::new(WorkerPool::new("JS Worker".to_string(), WorkerType::Js,  processor * 2, 1024 * 1024, 5000, JS_WORKER_WALKER.clone()));
    worker_pool0.run(JS_TASK_POOL.clone());

    let worker_pool1 = Box::new(WorkerPool::new("Store Worker".to_string(), WorkerType::Store,  processor, 1024 * 1024, 10000, STORE_WORKER_WALKER.clone()));
    worker_pool1.run(STORE_TASK_POOL.clone());

    let worker_pool = Box::new(WorkerPool::new("Network Worker".to_string(), WorkerType::Net,  processor, 1024 * 1024, 30000, NET_WORKER_WALKER.clone()));
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
    pi_store_build::register(&BON_MGR);

	let matches = args();
	let config = matches.value_of("config").unwrap();
	let mut dirs: Vec<String> = config.split(",").map(|e|{e.to_string()}).collect();
    if !dirs[0].ends_with("/"){
        dirs[0] += "/";
    }

    let file_list = read_file_list( &Path::new(&(dirs[0].clone() + ".depend")).to_path_buf());
    if dirs.len() == 1{
        init_js(&dirs[0..1], file_list, dirs[0].clone());
    }else if dirs.len() > 1{
        init_js(&dirs[1..], file_list, dirs[0].clone());
    }else {
        panic!("load dir is none, please start with '-c rootdir' or '-c rootdir,load module1,load module1..'");
    }

    match matches.value_of("shell") {
        Some("true") => {
            let (req_sender, req_receiver) = channel();
            let (resp_sender, resp_receiver) = channel();

            let req_sender_copy = req_sender.clone();
            let resp = Arc::new(move |result: IOResult<Arc<Vec<u8>>>, req: Option<Box<FnBox(Arc<Vec<u8>>)>>| {
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

                let mut req: Option<Box<FnBox(Arc<Vec<u8>>)>> = None;
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
