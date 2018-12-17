use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use pi_vm::adapter::{JS};
use pi_vm::bonmgr::{ptr_jstype, NativeObjsAuth};
use pi_db::mgr::{Mgr};
use pi_db::memery_db::DB;
use pi_db::db::{SResult, TabKV, TabMeta};
use guid::{GuidGen};
use atom::Atom;
use sinfo::{EnumType};
use bon::{WriteBuffer, Encode};
use util::store_depend;
use lib_util::now_millisecond;

use depend::{Depend, FileDes};
use jsloader::Loader;

pub fn init_js(dirs: &[String], file_list: Vec<FileDes>, root: String){
    let mgr = Mgr::new(GuidGen::new(0,0)); //创建数据库管理器
    mgr.register(Atom::from("memory"), Arc::new(DB::new()));//注册一个内存数据库

    let t1 = now_millisecond();
    store_depend(&mgr, &file_list);
    let t = now_millisecond();
    println!("init_js store_depend----------------------------------{}", t - t1);

    let dp = Depend::new(file_list, root.clone());
    let mut dir_c = Vec::from(dirs);
    push_pre(&mut dir_c);

    let file_map = Loader::load_dir_sync(dir_c.as_slice(), &dp);
    let js = JS::new(0x100, Arc::new(NativeObjsAuth::new(None, None))).unwrap();
    create_code_tab(&mgr);//创建代码表
    let global_code = bind_global(&mgr, &js);//插入全局变量定义函数的字节码
    let file_map = code_store(&mgr, file_map, &js);//插入其他所有js代码的字节码
    js.load(&global_code);//加载全局变量定义函数的字节码

    let list: Vec<String> = Loader::list(dirs, &dp);//列出目录下的所有文件
    let mut list_c = Vec::new();
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();
    let mut list_i = Vec::new();
    //let mut start_path = String::from("");
    for e in list.into_iter(){
        if e.ends_with(".s.js") || e.ends_with(".c.js"){
            list_c.push(e);
        }else if e.ends_with(".i.js"){
            list_i.push(e);
        }else if e.ends_with(".a.js"){
            list_a.push(e);
        }else if e.ends_with(".b.js"){
            list_b.push(e);
        }
    }
    list_c.extend_from_slice(&list_a);
    list_c.extend_from_slice(&list_b);
    list_c.extend_from_slice(&list_i);

    let mut list = Loader::list_with_depend(&list_c, &dp);

    push_pre(&mut list);
    {
        let path = &list[0];//如果是"bin/evn.js", 表示self已经定义， 此时可以为self绑定变量
        let u8arr = file_map.get(path).unwrap().as_slice();
        js.load(u8arr);
        //调用全局变量定义函数， 定义全局变量_$mgr
        js.get_js_function("_$defineGlobal".to_string());
        js.new_str(String::from("_$db_mgr"));
        let ptr = Box::into_raw(Box::new(mgr.clone())) as usize;
        ptr_jstype(js.get_objs(), js.clone(), ptr, 2976191628); //new native obj作为参数
        js.call(2);

        //调用全局变量定义函数， 定义全局变量_$mgr
        js.get_js_function("_$defineGlobal".to_string());
        js.new_str(String::from("_$depend"));
        let ptr = Box::into_raw(Box::new(dp)) as usize;
        ptr_jstype(js.get_objs(), js.clone(), ptr, 1797798710); //new native obj作为参数
        js.call(2);

        //调用全局变量定义函数， 定义全局变量_$mgr
        js.get_js_function("_$defineGlobal".to_string());
        js.new_str(String::from("_$root"));
        js.new_str(root);
        js.call(2);
    }
    for i in 1..list.len(){
        let path = &list[i];
        //println!("des:{}", &path);
        if path.ends_with(".js"){
            let u8arr = file_map.get(path).unwrap().as_slice();
            js.load(u8arr);
            loop{
                if js.is_ran(){
                    break;
                }
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
    
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
    let t1 = now_millisecond();
    let ware = Atom::from("memory");
    let tab = Atom::from("_$code");
    let mut items = Vec::new();
    let mut m = HashMap::new();
    for (key, v) in map.into_iter(){
        if !key.ends_with(".js"){
            continue;
        }
        let mut bb = WriteBuffer::new();
        key.encode(&mut bb);
        let mut item = TabKV::new(ware.clone(), tab.clone(), Arc::new(bb.unwrap()));
        let c = match String::from_utf8(v) {
            Ok(v) => v,
            Err(_) => panic!("code from_utf8 err, path: {}", key.clone()),
        };
        let code = Arc::new(js.compile(key.clone(), c).unwrap());
        item.value = Some(code.clone());
        items.push(item);
        m.insert(key.clone(), code.clone());
    }
    let t = now_millisecond();
    println!("code_store compile----------------------------------{}", t - t1);
    let tr = mgr.transaction(true);
    tr.modify(items, None, false, Arc::new(|_r: SResult<()>|{}));
    tr.prepare(Arc::new(|_x|{}));
    tr.commit(Arc::new(|_x|{}));
    let t1 = now_millisecond();
    println!("code_store store----------------------------------{}", t1 - t);
    m
}


pub fn push_pre(list:&mut Vec<String>){
    let evn = "evn.js".to_string();
    let core = "core.js".to_string();
    let first = "first.js".to_string();
    let next = "next.js".to_string();
    let last = "last.js".to_string();

	list.insert(0, next);
    list.insert(0, first);//初始js
    list.insert(0, core);//初始js
    list.insert(0, evn);//初始js
	list.push(last);
}

//编译_$defineGlobal函数， 得到字节码（_$defineGlobal用于定义全局变量）
pub fn bind_global(mgr: &Mgr, js: &JS) -> Vec<u8>{
    let key = String::from("_$define_global.js");
    let code = compeil_global(js);
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

pub fn compeil_global(js: &JS) -> Vec<u8>{
    let jscode = r#"function _$defineGlobal(name, value){
        console.log("_$defineGlobal is call, name:" + name);
        if(self[name]){
            throw "There has been a global variable " + name;
        }

        self[name] = value;
        console.log(value);
    }"#;
    let code = js.compile("_$define_global.js".to_string(), jscode.to_string()).unwrap();
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