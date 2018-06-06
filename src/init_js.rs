use std::collections::HashMap;
use depend::Depend;
use jsloader::Loader;
use pi_vm::adapter::{JS};

pub fn init_js(dirs: &[String], file_map: &HashMap<String, Vec<u8>>, dp: &Depend){
    let list: Vec<String> = Loader::list(dirs, dp);//列出目录下的所有文件
    let mut list_c = Vec::new();
    let mut list_i = Vec::new();
    for e in list.into_iter(){
        if e.ends_with(".s.js") || e.ends_with(".c.js"){
            list_c.push(e);
        }else if e.ends_with(".i.js"){
            list_i.push(e);
        }
    }
    list_c.extend_from_slice(&list_i);
    push_pre(&mut list_c);

    let list = Loader::list_with_depend(&list_c, dp);
    let mut js_code = String::from("");
    for des in list.iter(){
        let path = String::from(des.borrow().path.as_ref());
        println!("des:{}", &path);
        if path.ends_with(".js"){
            let u8arr = file_map.get(&path).unwrap().as_slice();
            js_code = js_code + "\n" + &String::from_utf8_lossy(u8arr);
        }
    }

    //let code_number = add_line_number(&js_code);
    //println!("{}", &js_code);
    
    let js = JS::new().unwrap();
    let bytes = js.compile("init_js".to_string(), js_code).unwrap();
    js.load(bytes.as_slice());
    println!("vm:meta运行成功！");
}


pub fn push_pre(list:&mut Vec<String>){
    let evn = String::from("pi/rt/evn.js");
	let core = String::from("pi/rt/core.js");
	let firstjs = String::from("pi/rt/first.js");
	let nextjs = String::from("pi/rt/next.js");
	let lastjs = String::from("pi/rt/last.js");

	list.insert(0, nextjs);
    list.insert(0, firstjs);//初始js
    list.insert(0, core);//初始js
    list.insert(0, evn);//初始js
	list.push(lastjs);
}

pub fn add_line_number(s: &str) -> String{
    let jc: Vec<&str> = s.split("\n").collect();
    let mut s = String::from("");
    for i in 1..jc.len() + 1{
        s = s + "\n" + i.to_string().as_str() + jc[i-1];
    }
    s
}




