use std::collections::HashMap;
use std::sync::Arc;

use pi_vm::adapter::{JS};
use pi_vm::bonmgr::{ptr_jstype, NativeObjsAuth};
use pi_db::mgr::{Mgr};
use pi_db::memery_db::DB;
use pi_db::db::{SResult, TabKV, TabMeta};
use pi_lib::guid::{GuidGen};
use pi_lib::atom::Atom;
use pi_lib::sinfo::{EnumType, StructInfo};
use pi_lib::bon::{WriteBuffer, Encode};
use std::thread;
use std::time::Duration;

use pi_base::util::now_millisecond;

use depend::Depend;
use jsloader::Loader;

pub fn init_js(dirs: &[String], dp: &Depend){
    let mut dir_c = Vec::from(dirs);
    push_pre(&mut dir_c);

    let file_map = Loader::load_dir_sync(dir_c.as_slice(), dp);
    let js = JS::new(0x100, Arc::new(NativeObjsAuth::new(None, None))).unwrap();
    let mgr = Mgr::new(GuidGen::new(0,0)); //创建数据库管理器
    mgr.register(Atom::from("memory"), Arc::new(DB::new()));//注册一个内存数据库
    create_code_tab(&mgr);//创建代码表
    let global_code = bind_global(&mgr, &js);//插入全局变量定义函数的字节码
    let file_map = code_store(&mgr, file_map, &js);//插入其他所有js代码的字节码
    js.load(&global_code);//加载全局变量定义函数的字节码

    let list: Vec<String> = Loader::list(dirs, dp);//列出目录下的所有文件
    let mut list_c = Vec::new();
    let mut list_i = Vec::new();
    //let mut start_path = String::from("");
    for e in list.into_iter(){
        if e.ends_with(".s.js") || e.ends_with(".c.js"){
            list_c.push(e);
        }else if e.ends_with(".i.js"){
            list_i.push(e);
        }/*else if e.ends_with(".st.js"){
            start_path = e;
        }*/
    }
    //list_c.push(start_path);
    list_c.extend_from_slice(&list_i);
    push_pre(&mut list_c);

    let list = Loader::list_with_depend(&list_c, dp);
    for des in list.iter(){
        let path = String::from(des.borrow().path.as_ref());
        //println!("des:{}", &path);
        if path.ends_with(".js"){
            let u8arr = file_map.get(&path).unwrap().as_slice();
            js.load(u8arr);
            if path == "bin/evn.js"{//如果是"bin/evn.js", 表示self已经定义， 此时可以为self绑定变量
                
                //调用全局变量定义函数， 定义全局变量_$mgr
                js.get_js_function("_$defineGlobal".to_string());
                js.new_str(String::from("_$db_mgr"));
                let ptr = Box::into_raw(Box::new(mgr.clone())) as usize;
                ptr_jstype(js.get_objs(), js.clone(), ptr, 2976191628); //new native obj作为参数
                js.call(2);

                //调用全局变量定义函数， 定义全局变量_$mgr
                js.get_js_function("_$defineGlobal".to_string());
                js.new_str(String::from("_$depend"));
                let ptr = dp as *const Depend as usize;
                ptr_jstype(js.get_objs_ref(), js.clone(), ptr, 1797798710); //new native obj作为参数
                js.call(2);
            }
        }
    }

    // let code_number = add_line_number(&js_code);
    // println!("{}", &js_code);

    // let bytes = js.compile("init_js".to_string(), js_code).unwrap();
    
    println!("vm:meta运行成功！!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
}

pub fn create_code_tab(mgr: &Mgr){
    let ware = Atom::from("memory");
    let tab = Atom::from("_$code");
    let tr = mgr.transaction(true);
    tr.alter(&ware, &tab, Some(Arc::new(TabMeta::new(EnumType::Str, EnumType::Bin))), Arc::new(|_r: SResult<()>|{}));
    tr.prepare(Arc::new(|_x|{}));
    tr.commit(Arc::new(|_x|{}));
}

//将代码存入数据库，因为是内存数据库， 暂时没有关心异步插入的情况，后面会改 
pub fn code_store(mgr: &Mgr, map: HashMap<String, Vec<u8>>, js: &JS) -> HashMap<String, Arc<Vec<u8>>>{
    let ware = Atom::from("memory");
    let tab = Atom::from("_$code");
    let mut items = Vec::new();
    let mut m = HashMap::new();
    for (key, v) in map.into_iter(){
        let mut bb = WriteBuffer::new();
        key.encode(&mut bb);
        let mut item = TabKV::new(ware.clone(), tab.clone(), Arc::new(bb.unwrap()));
        let c = match String::from_utf8(v) {
            Ok(v) => v,
            Err(_) => panic!("code from_utf8 err, path: {}", key.clone()),
        };
        println!("!!!!!!key: {}", key);
        let code = Arc::new(js.compile(key.clone(), c).unwrap());
        item.value = Some(code.clone());
        items.push(item);
        m.insert(key.clone(), code.clone());
    }
    let tr = mgr.transaction(true);
    tr.modify(items, None, false, Arc::new(|_r: SResult<()>|{}));
    tr.prepare(Arc::new(|_x|{}));
    tr.commit(Arc::new(|_x|{}));
    m
}


pub fn push_pre(list:&mut Vec<String>){
    let evn = String::from("bin/evn.js");
	let core = String::from("bin/core.js");
	let firstjs = String::from("bin/first.js");
	let nextjs = String::from("bin/next.js");
	let lastjs = String::from("bin/last.js");

	list.insert(0, nextjs);
    list.insert(0, firstjs);//初始js
    list.insert(0, core);//初始js
    list.insert(0, evn);//初始js
	list.push(lastjs);
}

//编译_$defineGlobal函数， 得到字节码（_$defineGlobal用于定义全局变量）
pub fn bind_global(mgr: &Mgr, js: &JS) -> Vec<u8>{
    let jscode = r#"function _$defineGlobal(name, value){
        console.log("_$defineGlobal is call, name:" + name);
        if(self[name]){
            throw "There has been a global variable " + name;
        }

        self[name] = value;
        console.log(value);
    }"#;
    let key = String::from("_$define_global.js");
    let code = js.compile("_$define_global.js".to_string(), jscode.to_string()).unwrap();
    let ware = Atom::from("memory");
    let tab = Atom::from("_$code");
    let tr = mgr.transaction(true);
    let mut arr = Vec::new();
    let mut bb = WriteBuffer::new();
    key.encode(&mut bb);
    let mut item = TabKV::new(ware.clone(), tab.clone(), Arc::new(bb.unwrap()));
    item.value = Some(Arc::new(code.clone()));
    arr.push(item);
    tr.modify(arr, None, false, Arc::new(|_r: SResult<()>|{}));
    return code;
}

pub fn add_line_number(s: &str) -> String{
    let jc: Vec<&str> = s.split("\n").collect();
    let mut s = String::from("");
    for i in 1..jc.len() + 1{
        s = s + "\n" + i.to_string().as_str() + jc[i-1];
    }
    s
}