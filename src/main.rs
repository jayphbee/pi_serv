#![feature(generic_associated_types)]

extern crate clap;
// extern crate pi_vm;
use clap::{Arg, App, SubCommand};

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, Condvar};

// use pi_vm::util::now_nanosecond;
// use pi_vm::task_pool::TaskPool;
// use pi_vm::task::TaskType;
// use pi_vm::worker_pool::WorkerPool;
// use pi_vm::adapter::{njsc_test_main, register_data_view, register_native_object, JSTemplate, JS, sync_cast_task, sync_cast_block_reply_task};

extern crate toml;
use std::fs::File;
use std::io::prelude::*;

extern crate fnv;
extern crate string_cache;

extern crate pi_lib;
extern crate pi_db;
extern crate pi_vm;
extern crate net;
extern crate mqtt;
extern crate rpc;

mod handler;

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

fn read_config(file_path: &str) -> toml::Value {
    let mut file = match File::open(file_path) {
		Ok(f) => f,
		Err(e) => panic!("no such file {} exception:{}", file_path, e)
	};
	let mut str_val = String::new();
	match file.read_to_string(&mut str_val) {
		Ok(s) => s
		,
		Err(e) => panic!("Error Reading file: {}", e)
	};
	let config: toml::Value = toml::from_str(&str_val).unwrap();
    // println!("toml config: {}", config);
    return config
}

fn main() {
    // let matches = args();
    // let config = matches.value_of("config").unwrap();
    // println!("Value for config: {}", config);
    // let cfg = read_config(config);
    // println!("config: {}", cfg["config"]);
    // println!("global: {}", cfg["global"]);
    // println!("local: {}", cfg["local"]);
    // println!("table: {}", cfg["table"]);
    // println!("func: {}", cfg["func"]);


    // start_njx();
}

// fn start_njx() {
//     register_data_view();
//     register_native_object();
//     let js = JSTemplate::new("var obj = {}; console.log(\"!!!!!!obj: \" + obj); function call(x, y, z) { var r = NativeObject.call(0xffffffff, [x, y, z]); console.log(\"!!!!!!r: \" + r); };".to_string());
//     assert!(js.is_some());
//     let js = js.unwrap();
//     let copy = Arc::new(js.clone().unwrap());
//     copy.run();
// }