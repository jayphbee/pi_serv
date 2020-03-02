use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::env;
use std::fs::File;
use std::io::Read;

use atom::Atom;
use bon::{Encode, WriteBuffer};
use guid::GuidGen;
use pi_db::db::{SResult, TabKV, TabMeta};
use pi_db::memery_db::DB;
use pi_db::mgr::Mgr;

use pi_vm::adapter::JS;
use pi_vm::bonmgr::{ptr_jstype, NativeObjsAuth};
use pi_vm::shell::SHELL_MANAGER;
use pi_vm::duk_proc::{DukProcess, DukProcessFactory};
use pi_vm::proc_pool::set_factory;

use js_env::{env_var, set_current_dir, current_dir};

use sinfo::EnumType;
use time::now_millisecond;

pub fn load_core_env(js: &Arc<JS>) {
    let cur_exe = env::current_exe().unwrap();
    // 初始化js执行环境
    let env_code = read_code(&cur_exe.join("../env.js"));
    let core_code = read_code(&cur_exe.join("../core.js"));

    let env_code = js.compile("env.js".to_string(), env_code).unwrap();
    let core_code = js.compile("core.js".to_string(), core_code).unwrap();

    load_code(&js, env_code.as_slice());
    load_code(&js, core_code.as_slice());
}

pub fn exec_js(path: String) {
    let path = path.as_str().replace("\\", "/");
    let cur_exe = env::current_exe().unwrap();
    let auth = Arc::new(NativeObjsAuth::new(None, None));

    let mgr = Mgr::new(GuidGen::new(0, 0)); //创建数据库管理器
    mgr.register(Atom::from("memory"), Arc::new(DB::new())); //注册一个内存数据库

    let js = JS::new(1, Atom::from("init compile"), auth.clone(), None).unwrap();

    // 设置当前运行目录
    let build_out_root = env_var("PROJECT_ROOT").unwrap();
    set_current_dir(&build_out_root);

    // 初始化js执行环境
    let env_code = read_code(&cur_exe.join("../env.js"));
    let core_code = read_code(&cur_exe.join("../core.js"));

    let env_code = js.compile("env.js".to_string(), env_code).unwrap();
    let core_code = js.compile("core.js".to_string(), core_code).unwrap();

    load_code(&js, env_code.as_slice());
    load_code(&js, core_code.as_slice());

    //创建一个进程工厂
    let duk_facotry_name = Atom::from("duk");
    let duk_factory = DukProcessFactory::new(
        duk_facotry_name.clone(),
        auth,
        Arc::new(vec![env_code, core_code]),
    );
    set_factory(Atom::from("duk"), Arc::new(duk_factory));

    let global_code = bind_global(&mgr, &js);
    js.load(&global_code);

    //////////////
    //调用全局变量定义函数， 定义全局变量_$mgr
    js.get_js_function("_$defineGlobal".to_string());
    js.new_str(String::from("_$db_mgr"));
    let ptr = Box::into_raw(Box::new(mgr.clone())) as usize;
    ptr_jstype(js.get_objs(), js.clone(), ptr, 2976191628); //new native obj作为参数
    js.call(2);

    //////////////
    let cur_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    let cur_dir = cur_dir.as_str().replace("\\", "/") + "/a";
    if js.get_link_function("Module.require".to_string()) {
        js.new_str(path).unwrap();
        js.new_str(cur_dir).unwrap();
        js.call(2);
    } else {
        panic!("Module.require function is not exist");
    }
}


pub fn read_code(path: &PathBuf) -> String {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {:?} exception:{}", path, e),
    };
    let mut str_val = String::new();
    if let Err(e) = file.read_to_string(&mut str_val) {
        panic!("Error Reading file: {}", e)
    }
    return str_val;
}

pub fn load_code(js: &Arc<JS>, code: &[u8]) -> bool {
    loop {
        if js.is_ran() {
            return js.load(&code);
        }
        pi_vm::adapter::pause();
    }
}

//编译_$defineGlobal函数， 得到字节码（_$defineGlobal用于定义全局变量）
fn bind_global(mgr: &Mgr, js: &JS) -> Vec<u8> {
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
    tr.modify(arr, None, false, Arc::new(|_r: SResult<()>| {}));
    return code;
}

pub fn compeil_global(js: &JS) -> Vec<u8> {
    let jscode = r#"function _$defineGlobal(name, value){
        console.log("_$defineGlobal is call, name:" + name);
        if(self[name]){
            throw "There has been a global variable " + name;
        }

        self[name] = value;
        console.log(value);
    }"#;
    let code = js
        .compile("_$define_global.js".to_string(), jscode.to_string())
        .unwrap();
    return code;
}