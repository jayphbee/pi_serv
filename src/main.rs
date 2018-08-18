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
extern crate pi_store;
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

#[macro_use]
extern crate lazy_static;

pub mod jsloader;
pub mod depend;
pub mod init_js;
pub mod util;
pub mod handler;
pub mod js_httpc;
pub mod js_db;
pub mod js_net;
pub mod js_base;
mod js_util;
mod async_call;
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
mod pi_store_build;
mod pi_net_httpc_build;

use std::fs::{File};
use std::path::Path;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

#[cfg(not(unix))]
use pi_vm::adapter::load_lib_backtrace;
use pi_vm::adapter::{register_native_object};
use pi_vm::bonmgr::BON_MGR;
use clap::{Arg, App};

use pi_base::util::now_millisecond;
use pi_base::worker_pool::WorkerPool;
use pi_base::pi_base_impl::{JS_TASK_POOL, STORE_TASK_POOL, EXT_TASK_POOL};
use pi_base::timer::TIMER;

use json::{JsonValue, parse};
use depend::{FileDes, Depend};
use init_js::{init_js};
use jsloader::Loader;
use js_base::IS_END;

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

fn read_file_str(path: &str) -> String{
	let mut file = match File::open(path) {
		Ok(f) => f,
		Err(e) => panic!("no such file {} exception:{}", path, e)
	};
	let mut str_val = String::new();
	match file.read_to_string(&mut str_val) {
		Ok(_) => str_val,
		Err(e) => panic!("Error Reading file: {}", e)
	}
}

fn read_file_list(dir: &str, pre_dir: &str) -> Vec<FileDes>{
	let path = Path::new(dir).join(pre_dir).join(".depend");
	let path = path.to_str().unwrap();
	let content = read_file_str(path);
	let content = unsafe{content.slice_unchecked(11, content.len() - 14)};
	parse_file_list(content, pre_dir)
}

fn parse_file_list(s: &str, pre_dir: &str) -> Vec<FileDes>{
	let r = parse(s).expect("???????????????json");
	match r {
		JsonValue::Array(mut v) => {
			let mut arr = Vec::new();
			for _ in 0..v.len() {
				arr.push(FileDes::from(v.pop().unwrap(), pre_dir));
			}
			arr
		},
		_ => {panic!("???????array??????????Vec<FileDes>");},
	}

}

fn create_depend(sp: &[String]) -> Depend{
	let mut vec: Vec<FileDes> = Vec::new();
	let mut root = "";

	for e in sp{
		let mut e = e.as_str();
		if e.ends_with("/"){
			e = &e[0..e.len() - 1];
		}
		let mut index = e.rfind("/");
		if index.is_none() {
			panic!("illegal parameter:{}", e);
		}
		let i = index.unwrap();

		let pre_dir = String::from(unsafe{e.slice_unchecked(i + 1, e.len())}) + "/";
		root = unsafe{e.slice_unchecked(0, i)};
		let arr = read_file_list(root, &pre_dir);
		vec.extend(arr.into_iter());
	}
	Depend::new(vec, root)
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
    async_call::register(&BON_MGR);
	pi_p2p_build::register(&BON_MGR);
    pi_store_build::register(&BON_MGR);
    pi_net_httpc_build::register(&BON_MGR);

	let matches = args();
	let config = matches.value_of("config").unwrap();
	let dirs: Vec<String> = config.split(",").map(|e| {String::from(e)}).collect();
	let dirs = dirs.as_slice();
	let depend = create_depend(dirs);

    init_js(dirs, &depend);
    
    while !IS_END.lock().unwrap().0 {
        println!("###############loop, {}", now_millisecond());
        thread::sleep(Duration::from_millis(10000));
    }
    // loop {
    //     println!("###############loop, {}", now_millisecond());
    //     thread::sleep(Duration::from_millis(60000));
    // }
}

