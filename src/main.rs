#![feature(fs_read_write)]
#![feature(splice)]
#![feature(generic_associated_types)]
extern crate clap;
extern crate json;
extern crate toml;
extern crate pi_vm;
extern crate pi_test;
extern crate pi_math;
extern crate pi_crypto;
extern crate pi_db;
extern crate pi_lib;
extern crate core;

pub mod jsloader;
pub mod depend;
pub mod vm;
pub mod init_js;
pub mod util;
mod pi_crypto_build;
mod pi_math_build;
mod pi_test_build;
mod pi_db_build;
mod pi_lib_build;
mod def_build;


use clap::{Arg, App};
use json::{JsonValue, parse};
use depend::{FileDes, Depend};
use init_js::{init_js};
use jsloader::Loader;


// use pi_vm::util::now_nanosecond;
// use pi_vm::task_pool::TaskPool;
// use pi_vm::task::TaskType;
// use pi_vm::worker_pool::WorkerPool;
use pi_vm::adapter::{register_native_object};
use pi_vm::bonmgr::BON_MGR;

use std::fs::{File};
use std::path::Path;
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

// fn read_config(file_path: &str) -> toml::Value {
// 	let mut file = match File::open(file_path) {
// 		Ok(f) => f,
// 		Err(e) => panic!("no such file {} exception:{}", file_path, e)
// 	};
// 	let mut str_val = String::new();
// 	match file.read_to_string(&mut str_val) {
// 		Ok(s) => s
// 		,
// 		Err(e) => panic!("Error Reading file: {}", e)
// 	};
// 	let config: toml::Value = toml::from_str(&str_val).unwrap();
// 	// println!("toml config: {}", config);
// 	return config
// }

fn read_file_list(dir: &str, pre_dir: &str) -> Vec<FileDes>{
	let path = Path::new(dir).join(pre_dir).join(".depend");
	let path = path.to_str().unwrap();
	let content = read_file_str(path);
	let content = unsafe{content.slice_unchecked(11, content.len() - 14)};
	parse_file_list(content, pre_dir)
}

fn parse_file_list(s: &str, pre_dir: &str) -> Vec<FileDes>{
	let r = parse(s).expect("无法将字符串解析为json");
	match r {
		JsonValue::Array(mut v) => {
			let mut arr = Vec::new();
			for _ in 0..v.len() {
				arr.push(FileDes::from(v.pop().unwrap(), pre_dir));
			}
			arr
		},
		_ => {panic!("不是一个array，无法解析为Vec<FileDes>");},
	}

}

//根据目录创建依赖表
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
    register_native_object();
    pi_test_build::register(&BON_MGR);
    pi_crypto_build::register(&BON_MGR);
    pi_math_build::register(&BON_MGR);
    pi_lib_build::register(&BON_MGR);
    pi_db_build::register(&BON_MGR);
    def_build::register(&BON_MGR);

	let matches = args();
	let config = matches.value_of("config").unwrap();
	//let arg = CmdArg::from(parse(config).expect("config参数应该为一个jsonObject"));
	let dirs: Vec<String> = config.split(",").map(|e| {String::from(e)}).collect();
	let dirs = dirs.as_slice();
	let depend = create_depend(dirs);
	let file_map = Loader::load_dir_sync(dirs, &depend);

    init_js(dirs, &file_map, &depend);

    //  let cfg = String::from_utf8(read(Path::new("./init.cfg")).expect("未找到文件：./init.cfg")).unwrap();
    //  init_cfg(&cfg, &file_map, &depend);
	                    

	// for dir in dirs{
	// 	let mut file = match File::open(file_path) {
	// 	}
	// }
	// let cfg = read_config(config);
	//println!("config: {}", config);
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