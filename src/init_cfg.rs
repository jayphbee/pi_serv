use vm::Factory;
use std::collections::HashMap;
use depend::Depend;
use json::{JsonValue, parse};
use jsloader::Loader;


struct InitCfg(HashMap<String, Vec<String>>);

impl InitCfg {
    pub fn from(jv: JsonValue) -> Self{
        match jv {
            JsonValue::Object(o) => {
                let mut map = HashMap::new();
                for (k, v) in o.iter(){
                    let item = match v {
                        JsonValue::Array(v) => {
                            let mut arr = Vec::new();
                            for elem in v.iter(){
                                let elem = match elem {
                                    JsonValue::String(v) => String::from(v.as_str()),
                                    JsonValue::Short(v) => String::from(v.as_str()),
                                    _ => panic!("InitCfg解析错误"),
                                };
                                arr.push(elem);
                            }
                            arr
                        },
                        _ => panic!("InitCfg解析错误"),
                    };
                    map.insert(String::from(k), item);
                }
                InitCfg(map)
            },
            _ => {panic!("InitCfg解析错误");},
        }
    }
}

pub fn init_meta(dirs: &[String], file_map: &HashMap<String, Vec<u8>>, dp: &Depend){
    let mut list: Vec<String> = Loader::list(dirs, dp);
	list = list.into_iter().filter(|e|{if e.ends_with(".sjs") {true}else{false}}).collect();
    list.push(String::from("pi/rt/reg_meta.js"));
    push_evm(&mut list);
    let vm = Factory::creat_vm(&list, dp, file_map);
    println!("vm:meta运行成功！");
}

pub fn init_cfg(s: &str, file_map: &HashMap<String, Vec<u8>>, dp: &Depend){
    let cfg = InitCfg::from(parse(s).expect("字符串无法解析为json"));
    for (key, item) in cfg.0.iter(){
        let mut list = Loader::list(item, dp);
        push_evm(&mut list);
        let vm = Factory::creat_vm(&list, dp, file_map);
        println!("vm:{}运行成功！", key);
    }
}

pub fn push_evm(list:&mut Vec<String>){
    let evn = String::from("pi/rt/evn.js");
	let core = String::from("pi/rt/core.js");
	let initjs = String::from("pi/rt/init.js");
	let nextjs = String::from("pi/rt/next.js");
	let lastjs = String::from("pi/rt/last.js");

    list.insert(0, evn);//初始js
	list.insert(1, core);//初始js
	list.insert(2, initjs);//初始js
	list.insert(3, nextjs);
	list.push(lastjs);
}
