#![feature(fs_read_write)]
#![feature(splice)]
#![feature(generic_associated_types)]
#![feature(fnbox)]
#![feature(unboxed_closures)]
#[warn(dead_code)]
extern crate clap;
extern crate json;
extern crate toml;
extern crate pi_vm;
extern crate pi_math;
extern crate pi_crypto;
extern crate pi_db;
extern crate pi_lib;
extern crate core;
extern crate pi_base;
extern crate fnv;
extern crate net;
extern crate mqtt;
extern crate rpc;
extern crate magnetic;
extern crate rand;
extern crate pi_p2p;
extern crate mqtt3;
extern crate httpc;
extern crate https;

#[macro_use]
extern crate lazy_static;

pub mod jsloader;
pub mod depend;
pub mod init_js;
pub mod util;
pub mod js_httpc;
pub mod js_db;
pub mod js_net;
pub mod js_base;
pub mod js_lib;
pub mod js_async;
mod js_util;
mod pi_crypto_build;
mod pi_math_build;
mod pi_db_build;
mod pi_lib_build;
mod def_build;
mod pi_net_mqtt_build;
mod pi_net_net_build;
mod pi_net_rpc_build;
mod pi_serv_build;
mod pi_vm_build;
mod pi_p2p_build;
mod pi_net_httpc_build;
mod pi_net_https_build;

use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::path::Path;

#[cfg(not(unix))]
use pi_vm::adapter::load_lib_backtrace;
use pi_vm::adapter::{register_native_object};
use pi_vm::bonmgr::BON_MGR;
use clap::{Arg, App};

use pi_base::util::now_millisecond;
use pi_base::worker_pool::WorkerPool;
use pi_base::pi_base_impl::{JS_TASK_POOL, STORE_TASK_POOL, EXT_TASK_POOL};
use pi_base::timer::TIMER;

use init_js::{init_js};
use js_base::IS_END;
use util::{read_file_list};

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
						.get_matches();
	matches
	
}

fn main() {
	#[cfg(not(unix))]
    load_lib_backtrace();
    TIMER.run();
    register_native_object();
    let worker_pool0 = Box::new(WorkerPool::new(3, 1024 * 1024, 1000));
    worker_pool0.run(JS_TASK_POOL.clone());

    let worker_pool1 = Box::new(WorkerPool::new(3, 1024 * 1024, 1000));
    worker_pool1.run(STORE_TASK_POOL.clone());

    let worker_pool = Box::new(WorkerPool::new(10, 1024 * 1024, 30000));
    worker_pool.run(EXT_TASK_POOL.clone());

    pi_crypto_build::register(&BON_MGR);
    pi_math_build::register(&BON_MGR);
    pi_lib_build::register(&BON_MGR);
    pi_db_build::register(&BON_MGR);
    def_build::register(&BON_MGR);
    pi_net_mqtt_build::register(&BON_MGR);
    pi_net_rpc_build::register(&BON_MGR);
    pi_net_net_build::register(&BON_MGR);
    pi_serv_build::register(&BON_MGR);
    pi_vm_build::register(&BON_MGR);
    js_async::register(&BON_MGR);
	pi_p2p_build::register(&BON_MGR);
    pi_net_httpc_build::register(&BON_MGR);
    pi_net_https_build::register(&BON_MGR);

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

    while !IS_END.lock().unwrap().0 {
        println!("###############loop, {}", now_millisecond());
        thread::sleep(Duration::from_millis(10000));
    }
    // loop {
    //     println!("###############loop, {}", now_millisecond());
    //     thread::sleep(Duration::from_millis(60000));
    // }
}