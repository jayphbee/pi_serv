use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use pi_vm::pi_vm_impl::{ block_reply, block_throw, push_callback};
use worker::task::TaskType;
use atom::Atom;
use std::mem::{transmute, uninitialized};
use pi_vm;
use mqtt_tmp;
use bon;
use pi_db;
use pi_store;
use pi_db::mgr::Monitor;
use std::env::VarError;
use sinfo;
use gray;
use std::sync::RwLock;
use guid;
use atom;
use httpc;
use handler;
use parking_lot;
use rpc_tmp;
use std::io::Error;
use base;
use rpc;
use js_db;
use util;
use js_vm;
use js_env;
use js_file;
use js_base;
use js_lib;
use js_httpc;
use js_net;
use js_async;
use hotfix;
use webshell;
use js_net_rpc_client;



fn call_3763610783_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in next";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3289224548, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut js_db::DBIter) };

    let jscopy = js.clone();
	let call_back = move |r: Result<Option<(Arc<Vec<u8>>,Arc<Vec<u8>>)>,String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {let mut r = match r{
        Some(v) => { 
	let array = js.new_array();
    let mut v_elem = v.0;
    let ptr = Box::into_raw(Box::new(v_elem)) as usize;let mut v_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,2886438122);

js.set_index(&array, 0, &mut v_elem);
    let mut v_elem = v.1;
    let ptr = Box::into_raw(Box::new(v_elem)) as usize;let mut v_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,2886438122);

js.set_index(&array, 1, &mut v_elem);    let mut v = array;
 v}
        None => js.new_null()
    };

            } ), Atom::from("sync,3763610783"));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    let r = js_db::DBIter::next(jst0,Arc::new(call_back));
	if r.is_some(){
        let r = r.unwrap();let mut r = match r{
        Ok(r) => { let mut r = match r{
        Some(v) => { 
	let array = js.new_array();
    let mut v_elem = v.0;
    let ptr = Box::into_raw(Box::new(v_elem)) as usize;let mut v_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,2886438122);

js.set_index(&array, 0, &mut v_elem);
    let mut v_elem = v.1;
    let ptr = Box::into_raw(Box::new(v_elem)) as usize;let mut v_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,2886438122);

js.set_index(&array, 1, &mut v_elem);    let mut v = array;
 v}
        None => js.new_null()
    };
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

        return Some(CallResult::Ok);
    }
	None
}


fn call_2701929727_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in next_elem";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3289224548, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut js_db::DBIter) };

    let jscopy = js.clone();
	let call_back = move |r: Result<Option<pi_vm::adapter::JSType>,String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {let mut r = match r{
        Some(v) => { let mut v = js.new_undefined(); v}
        None => js.new_null()
    };

            } ), Atom::from("sync,2701929727"));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    let r = js_db::DBIter::next_elem(jst0,Arc::new(call_back),&js);
	if r.is_some(){
        let r = r.unwrap();let mut r = match r{
        Ok(r) => { let mut r = match r{
        Some(v) => {  v}
        None => js.new_null()
    };
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

        return Some(CallResult::Ok);
    }
	None
}


fn call_1993779671(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2484911420, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const mqtt_tmp::server::ServerNode) };


	let jst1 = &v[1];
	if !jst1.is_uint8_array() && !jst1.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst1 = jst1.to_bytes();



    let result = js_db::DBToMqttMonitor::new(jst0,jst1);let mut result = match result{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,2627601653);

 r }
        Err(v) => { 
            return Some(CallResult::Err(v.to_string() + "Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_4117819797(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = js_db::DBToGlobalMqttMonitor::new(jst0);let mut result = match result{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,1632158050);

 r }
        Err(v) => { 
            return Some(CallResult::Err(v.to_string() + "Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_1168492209(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in notify";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1495847839, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const js_db::JSDBMonitor) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 3165549746, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut pi_db::db::Event) };


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 2976191628, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut pi_db::mgr::Mgr) };


    js_db::JSDBMonitor::notify(jst0,jst1,jst2);
    Some(CallResult::Ok)
}


fn call_2153620660(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 730519735, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut pi_vm::pi_vm_impl::VMFactory) };


    let result = js_db::JSDBMonitor::new(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1495847839);


    Some(CallResult::Ok)
}


fn call_1967373661_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in iter_db";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = jst2.get_str();


	let jst3 = &v[3];
    let jst3 = if jst3.is_undefined() || jst3.is_null(){
        None
    }else{
	if !jst3.is_uint8_array() && !jst3.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst3 = jst3.to_bytes();


        Some(jst3)
    };


	let jst4 = &v[4];
	if !jst4.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst4 = jst4.get_boolean();
    

	let jst5 = &v[5];
    let jst5 = if jst5.is_undefined() || jst5.is_null(){
        None
    }else{
	if !jst5.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst5 = jst5.get_str();

        Some(jst5)
    };

    let jscopy = js.clone();
	let call_back = move |r: Result<js_db::DBIter,String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,3289224548);


            } ), Atom::from("sync,1967373661"));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    let r = js_db::iter_db(jst0,jst1,jst2,jst3,jst4,jst5,Arc::new(call_back));
	if r.is_some(){
        let r = r.unwrap();let mut r = match r{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,3289224548);

 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

        return Some(CallResult::Ok);
    }
	None
}


fn call_1420275781(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in clone_db_mgr";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };


    let result = js_db::clone_db_mgr(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2976191628);


    Some(CallResult::Ok)
}


fn call_1905006775(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in register_memery_db";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 1237457629, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut pi_db::memery_db::DB) };


    let result = js_db::register_memery_db(jst0,jst1,jst2);let mut result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_3038249291(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in register_file_db";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 568147534, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut pi_store::lmdb_file::DB) };


    let result = js_db::register_file_db(jst0,jst1,jst2);let mut result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_2215620835(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in register_file_mem_db";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 2325173571, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut pi_store::file_mem_db::FileMemDB) };


    let result = js_db::register_file_mem_db(jst0,jst1,jst2);let mut result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_360427781(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in get_all_wares";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };


    let result = js_db::get_all_wares(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1542823015);


    Some(CallResult::Ok)
}


fn call_2573413979(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in get_tabmeta_buffer";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4164638564, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut Arc<pi_db::db::TabMeta>)}.clone();


    let result = js_db::get_tabmeta_buffer(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,104530634);


    Some(CallResult::Ok)
}


fn call_2097131752(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tabkv_with_value";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = &jst0.get_str();


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = &jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_uint8_array() && !jst2.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst2 = jst2.to_bytes();



	let jst3 = &v[3];
	if !jst3.is_uint8_array() && !jst3.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst3 = jst3.to_bytes();



    let result = js_db::tabkv_with_value(jst0,jst1,jst2,jst3);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,4000136370);


    Some(CallResult::Ok)
}


fn call_1247562096(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tabkv_new";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = &jst0.get_str();


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = &jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_uint8_array() && !jst2.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst2 = jst2.to_bytes();



    let result = js_db::tabkv_new(jst0,jst1,jst2);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,4000136370);


    Some(CallResult::Ok)
}


fn call_1579404380(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tabkv_get_value";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4000136370, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::db::TabKV) };


    let result = js_db::tabkv_get_value(jst0);let mut result = match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let mut v = ptr_jstype(js.get_objs(), js.clone(), ptr,2886438122);

 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_3169463176(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in list_all_tables";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


    let result = js_db::list_all_tables(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1542823015);


    Some(CallResult::Ok)
}


fn call_2680255887_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in alter";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = jst2.get_str();


	let jst3 = &v[3];
    let jst3 = if jst3.is_undefined() || jst3.is_null(){
        None
    }else{
	if !jst3.is_uint8_array() && !jst3.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst3 = jst3.to_bytes();


        Some(jst3)
    };

    let jscopy = js.clone();
	let call_back = move |r: Result<(),String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
	let array = js.new_array();    let mut r = array;

            } ), Atom::from("sync,2680255887"));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    let r = js_db::alter(jst0,jst1,jst2,jst3,Arc::new(call_back));
	if r.is_some(){
        let r = r.unwrap();let mut r = match r{
        Ok(r) => { 
	let array = js.new_array();    let mut r = array;
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

        return Some(CallResult::Ok);
    }
	None
}


fn call_2725879080_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in modify";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };


	let jst1 = &v[1];


	let jst2 = &v[2];
    let jst2 = if jst2.is_undefined() || jst2.is_null(){
        None
    }else{
	if !jst2.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_u32() as usize;

        Some(jst2)
    };


	let jst3 = &v[3];
	if !jst3.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst3 = jst3.get_boolean();
    
    let jscopy = js.clone();
	let call_back = move |r: Result<(),String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
	let array = js.new_array();    let mut r = array;

            } ), Atom::from("sync,2725879080"));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    let r = js_db::modify(jst0,jst1,jst2,jst3,Arc::new(call_back));
	if r.is_some(){
        let r = r.unwrap();let mut r = match r{
        Ok(r) => { 
	let array = js.new_array();    let mut r = array;
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

        return Some(CallResult::Ok);
    }
	None
}


fn call_583163851_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in query";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };


	let jst1 = &v[1];


	let jst2 = &v[2];
    let jst2 = if jst2.is_undefined() || jst2.is_null(){
        None
    }else{
	if !jst2.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_u32() as usize;

        Some(jst2)
    };


	let jst3 = &v[3];
	if !jst3.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst3 = jst3.get_boolean();
    
    let jscopy = js.clone();
	let call_back = move |r: Result<pi_vm::adapter::JSType,String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {let mut r = js.new_undefined();
            } ), Atom::from("sync,583163851"));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    let r = js_db::query(jst0,jst1,jst2,jst3,Arc::new(call_back),&js);
	if r.is_some(){
        let r = r.unwrap();let mut r = match r{
        Ok(r) => {  r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

        return Some(CallResult::Ok);
    }
	None
}


fn call_2986122496_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in tab_size";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = &jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = &jst2.get_str();

    let jscopy = js.clone();
	let call_back = move |r: Result<usize,String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {let mut r = js.new_u32(r as u32);

            } ), Atom::from("sync,2986122496"));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    let r = js_db::tab_size(jst0,jst1,jst2,Arc::new(call_back));
	if r.is_some(){
        let r = r.unwrap();let mut r = match r{
        Ok(r) => { let mut r = js.new_u32(r as u32);
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

        return Some(CallResult::Ok);
    }
	None
}


fn call_1869880364(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in register_db_to_mqtt_monitor";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 2627601653, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut js_db::DBToMqttMonitor) };


    js_db::register_db_to_mqtt_monitor(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_1719526885(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in register_db_to_global_mqtt_monitor";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1632158050, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut js_db::DBToGlobalMqttMonitor) };


    js_db::register_db_to_global_mqtt_monitor(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_4281318477_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in dump";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = jst2.get_str();


	let jst3 = &v[3];
	if !jst3.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst3 = jst3.get_str();

    let jscopy = js.clone();
	let call_back = move |r: Result<(),String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
	let array = js.new_array();    let mut r = array;

            } ), Atom::from("sync,4281318477"));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    js_db::dump(jst0,jst1,jst2,jst3,Arc::new(call_back));
	None
}


fn call_479322726_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in restore";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = jst2.get_str();


	let jst3 = &v[3];
	if !jst3.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst3 = jst3.get_str();

    let jscopy = js.clone();
	let call_back = move |r: Result<(),String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
	let array = js.new_array();    let mut r = array;

            } ), Atom::from("sync,479322726"));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    js_db::restore(jst0,jst1,jst2,jst3,Box::new(call_back));
	None
}


fn call_2176133173(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in register_db_js_db_monitor";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1495847839, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut js_db::JSDBMonitor) };


    js_db::register_db_js_db_monitor(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_2239806005(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in read_file";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = &jst0.get_str();


    let result = util::read_file(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,104530634);


    Some(CallResult::Ok)
}


fn call_1263843384(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in get_byte_code";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


    let result = js_vm::get_byte_code(jst0);let mut result = match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let mut v = ptr_jstype(js.get_objs(), js.clone(), ptr,2886438122);

 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_1749960077(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in remove_byte_code_cache";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


    js_vm::remove_byte_code_cache(jst0);
    Some(CallResult::Ok)
}


fn call_3619493605(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in rename_byte_code_cache";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


    js_vm::rename_byte_code_cache(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_3830865479_async( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

    let param_error = "param error in compile";
	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();

	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();

    let call_index = &v[2];
    if !call_index.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
    let call_index = call_index.get_u32();
    
    let jscopy = js.clone();
	let call_back = move |r: Result<Arc<Vec<u8>>,String>| {
		push_callback(jscopy.clone(), call_index, Box::new(move |js: Arc<JS>| {let mut r = match r{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,2886438122);

 r }
        Err(v) => { js.new_str(v + ", Result is Err").unwrap()
        }
    };

            1
        } ), None, Atom::from("call_3830865479_async1"));
    };

    js_vm::compile(jst0,jst1,Box::new(call_back));
	Some(CallResult::Ok)
}


fn call_2450233359(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in compile_sync";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


    let result = js_vm::compile_sync(jst0,jst1);let mut result = match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let mut v = ptr_jstype(js.get_objs(), js.clone(), ptr,2886438122);

 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_1380265392(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in load_module";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2886438122, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut Arc<Vec<u8>>)}.clone();


    js_vm::load_module(jst0,&js);
    Some(CallResult::Ok)
}


fn call_4192708231(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in next";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1694133887, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut js_env::Args) };


    let result = js_env::Args::next(jst0);let mut result = match result{
        Some(v) => { let mut v = js.new_str(v).unwrap();
 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_2544700472(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in next";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 591726708, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut js_env::EnvVars) };


    let result = js_env::EnvVars::next(jst0);let mut result = match result{
        Some(v) => { 
	let array = js.new_array();
    let mut v_elem = v.0;let mut v_elem = js.new_str(v_elem).unwrap();
js.set_index(&array, 0, &mut v_elem);
    let mut v_elem = v.1;let mut v_elem = js.new_str(v_elem).unwrap();
js.set_index(&array, 1, &mut v_elem);    let mut v = array;
 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_692858595(js: Arc<JS>) -> Option<CallResult>{

    let result = js_env::args();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1694133887);


    Some(CallResult::Ok)
}


fn call_76907791(js: Arc<JS>) -> Option<CallResult>{

    let result = js_env::current_dir();let mut result = match result{
        Ok(r) => { let mut r = js.new_str(r).unwrap();
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_3151666217(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in set_current_dir";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = &jst0.get_str();


    let result = js_env::set_current_dir(jst0);let mut result = match result{
        Ok(r) => { 
	let array = js.new_array();    let mut r = array;
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_4072555389(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in set_env_var";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = &jst0.get_str();


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = &jst1.get_str();


    js_env::set_env_var(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_3300744712(js: Arc<JS>) -> Option<CallResult>{

    let result = js_env::current_exe();let mut result = match result{
        Ok(r) => { let mut r = js.new_str(r).unwrap();
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_341310298(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in env_var";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = &jst0.get_str();


    let result = js_env::env_var(jst0);let mut result = match result{
        Ok(r) => { let mut r = js.new_str(r).unwrap();
 r }
        Err(v) => { 
            return Some(CallResult::Err(v.to_string() + "Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_2758093424(js: Arc<JS>) -> Option<CallResult>{

    let result = js_env::env_vars();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,591726708);


    Some(CallResult::Ok)
}


fn call_215229799_async( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

    let param_error = "param error in read_file_buffer";
	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();

    let call_index = &v[1];
    if !call_index.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
    let call_index = call_index.get_u32();
    
    let jscopy = js.clone();
	let call_back = move |r: Result<Vec<u8>,String>| {
		push_callback(jscopy.clone(), call_index, Box::new(move |js: Arc<JS>| {let mut r = match r{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,104530634);

 r }
        Err(v) => { js.new_str(v + ", Result is Err").unwrap()
        }
    };

            1
        } ), None, Atom::from("call_215229799_async1"));
    };

    js_file::read_file_buffer(jst0,Box::new(call_back));
	Some(CallResult::Ok)
}


fn call_3061910455_async( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

    let param_error = "param error in read_file_string";
	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();

    let call_index = &v[1];
    if !call_index.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
    let call_index = call_index.get_u32();
    
    let jscopy = js.clone();
	let call_back = move |r: Result<String,String>| {
		push_callback(jscopy.clone(), call_index, Box::new(move |js: Arc<JS>| {let mut r = match r{
        Ok(r) => { let mut r = js.new_str(r).unwrap();
 r }
        Err(v) => { js.new_str(v + ", Result is Err").unwrap()
        }
    };

            1
        } ), None, Atom::from("call_3061910455_async1"));
    };

    js_file::read_file_string(jst0,Box::new(call_back));
	Some(CallResult::Ok)
}


fn call_3728513126(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in read_file_buffer_sync";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = &jst0.get_str();


    let result = js_file::read_file_buffer_sync(jst0);let mut result = match result{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,104530634);

 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_2674074487(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in read_file_string_sync";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = &jst0.get_str();


    let result = js_file::read_file_string_sync(jst0);let mut result = match result{
        Ok(r) => { let mut r = js.new_str(r).unwrap();
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_3649129955(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in walk_dir_sync";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = &jst0.get_str();


    let result = js_file::walk_dir_sync(jst0);let mut result = match result{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,1542823015);

 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_3007613864(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in is_absolute";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = &jst0.get_str();


    let result = js_file::is_absolute(jst0);let mut result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_3595492395(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in is_relative";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = &jst0.get_str();


    let result = js_file::is_relative(jst0);let mut result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_1347190475(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in create_sinfo";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = js_base::create_sinfo(jst0);let mut result = match result{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,1846921536);

 r }
        Err(v) => { 
            return Some(CallResult::Err(v.to_string() + "Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_3993207385(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in clone_vm_factory";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 730519735, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_vm::pi_vm_impl::VMFactory) };


    let result = js_base::clone_vm_factory(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,730519735);


    Some(CallResult::Ok)
}


fn call_4111533257(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in arc_new_async_request_handler";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 259136547, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut js_async::AsyncRequestHandler) };


    let result = js_base::arc_new_async_request_handler(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,374659923);


    Some(CallResult::Ok)
}


fn call_3272869145(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in register_async_handler";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 374659923, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const Arc<js_async::AsyncRequestHandler>) };


    js_base::register_async_handler(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_3741531906(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in arc_deref";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2886438122, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Arc<Vec<u8>>) };


    let result = js_base::arc_deref(jst0);
    let ptr = result as *const Vec<u8> as usize;let mut result = ptr_jstype(js.get_objs_ref(), js.clone(), ptr,104530634);


    Some(CallResult::Ok)
}


fn call_1810043215_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in sleep";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32();

    let jscopy = js.clone();
    let call_back = move || {
		block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {js.new_null();}), Atom::from("call_1810043215_sync"));
    };
    js_base::sleep(jst0,Box::new(call_back));
	None
}


fn call_3285798497(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in clear_timeout";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    js_base::clear_timeout(jst0);
    Some(CallResult::Ok)
}


fn call_59144274(js: Arc<JS>) -> Option<CallResult>{

    let result = js_base::create_rand();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3355421248);


    Some(CallResult::Ok)
}


fn call_3881780156(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in next_u32";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3355421248, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut js_base::Rand) };


    let result = js_base::next_u32(jst0);let mut result = js.new_u32(result);

    Some(CallResult::Ok)
}


fn call_3908949488(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in next_u64";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3355421248, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut js_base::Rand) };


    let result = js_base::next_u64(jst0);let mut result = js.new_u64(result);

    Some(CallResult::Ok)
}


fn call_2556550051(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in fill_bytes";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3355421248, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut js_base::Rand) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = js_base::fill_bytes(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,104530634);


    Some(CallResult::Ok)
}


fn call_957759389(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in try_fill_bytes";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3355421248, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut js_base::Rand) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = js_base::try_fill_bytes(jst0,jst1);let mut result = match result{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,104530634);

 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_370495443(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in drop_native_obj";

	let jst0 = &v[0];


    let result = js_base::drop_native_obj(jst0,&js);let mut result = match result{
        Ok(r) => { let mut r = js.new_boolean(r);
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_2041214057(js: Arc<JS>) -> Option<CallResult>{

    js_base::end(&js);
    Some(CallResult::Ok)
}


fn call_2697841501(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 2643678751, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut Arc<pi_vm::pi_vm_impl::VMFactory>)}.clone();


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = &jst2.get_str();


    let result = js_lib::JSGray::new(jst0,jst1,jst2);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2566315655);


    Some(CallResult::Ok)
}


fn call_3635855143(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in create_gray_tab";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2566315655, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut js_lib::JSGray) };


    let result = js_lib::create_gray_tab(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3386914360);


    Some(CallResult::Ok)
}


fn call_1199149424(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in create_arc_vmfactory";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 730519735, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_vm::pi_vm_impl::VMFactory) };


    let result = js_lib::create_arc_vmfactory(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2643678751);


    Some(CallResult::Ok)
}


fn call_3557646357(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in guid_gen";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1736136244, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const guid::GuidGen) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u16();


    let result = js_lib::guid_gen(jst0,jst1);let mut result = js.new_str(result.to_string()).unwrap(); 
    Some(CallResult::Ok)
}


fn call_3906048478(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bonbuf_cmp";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



	let jst1 = &v[1];
	if !jst1.is_uint8_array() && !jst1.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst1 = jst1.to_bytes();



    let result = js_lib::bonbuf_cmp(jst0,jst1);let mut result = match result{
        Some(v) => { let mut v = js.new_i32(v);
 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_373179692(js: Arc<JS>) -> Option<CallResult>{

    let result = js_httpc::HttpClientOptions::default();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1131624585);


    Some(CallResult::Ok)
}


fn call_145125716(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in normal";

	let jst0 = &v[0];
	if !jst0.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.get_boolean();
    

	let jst1 = &v[1];
	if !jst1.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst1 = jst1.get_boolean();
    

	let jst2 = &v[2];
	if !jst2.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst2 = jst2.get_boolean();
    

	let jst3 = &v[3];
	if !jst3.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst3 = jst3.get_i32() as isize;


	let jst4 = &v[4];
    if !jst4.is_uint8_array() && !jst4.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let arr = unsafe{*(jst4.to_bytes().as_ptr() as usize as *const [u8; 8])};
    let jst4 = unsafe {
        transmute::<[u8; 8], u64>(arr)
    }; 


    let result = js_httpc::HttpClientOptions::normal(jst0,jst1,jst2,jst3,jst4);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1131624585);


    Some(CallResult::Ok)
}


fn call_2887071833(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in vaild_host";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = jst2.get_str();


	let jst3 = &v[3];
	if !jst3.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst3 = jst3.get_boolean();
    

	let jst4 = &v[4];
	if !jst4.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst4 = jst4.get_boolean();
    

	let jst5 = &v[5];
	if !jst5.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst5 = jst5.get_i32() as isize;


	let jst6 = &v[6];
    if !jst6.is_uint8_array() && !jst6.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let arr = unsafe{*(jst6.to_bytes().as_ptr() as usize as *const [u8; 8])};
    let jst6 = unsafe {
        transmute::<[u8; 8], u64>(arr)
    }; 


    let result = js_httpc::HttpClientOptions::vaild_host(jst0,jst1,jst2,jst3,jst4,jst5,jst6);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1131624585);


    Some(CallResult::Ok)
}


fn call_2011091417(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in proxy";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


	let jst1 = &v[1];
	if !jst1.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst1 = jst1.get_boolean();
    

	let jst2 = &v[2];
	if !jst2.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst2 = jst2.get_boolean();
    

	let jst3 = &v[3];
	if !jst3.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst3 = jst3.get_boolean();
    

	let jst4 = &v[4];
	if !jst4.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst4 = jst4.get_i32() as isize;


	let jst5 = &v[5];
    if !jst5.is_uint8_array() && !jst5.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let arr = unsafe{*(jst5.to_bytes().as_ptr() as usize as *const [u8; 8])};
    let jst5 = unsafe {
        transmute::<[u8; 8], u64>(arr)
    }; 


    let result = js_httpc::HttpClientOptions::proxy(jst0,jst1,jst2,jst3,jst4,jst5);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1131624585);


    Some(CallResult::Ok)
}


fn call_2937777264(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in valid_host_proxy";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = jst2.get_str();


	let jst3 = &v[3];
	if !jst3.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst3 = jst3.get_str();


	let jst4 = &v[4];
	if !jst4.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst4 = jst4.get_boolean();
    

	let jst5 = &v[5];
	if !jst5.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst5 = jst5.get_boolean();
    

	let jst6 = &v[6];
	if !jst6.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst6 = jst6.get_i32() as isize;


	let jst7 = &v[7];
    if !jst7.is_uint8_array() && !jst7.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let arr = unsafe{*(jst7.to_bytes().as_ptr() as usize as *const [u8; 8])};
    let jst7 = unsafe {
        transmute::<[u8; 8], u64>(arr)
    }; 


    let result = js_httpc::HttpClientOptions::valid_host_proxy(jst0,jst1,jst2,jst3,jst4,jst5,jst6,jst7);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1131624585);


    Some(CallResult::Ok)
}


fn call_2175286088(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in get_json_val";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4139279264, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const js_httpc::HttpClientBody<Vec<u8>>) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


    let result = js_httpc::HttpClientBody::<Vec<u8>>::get_json_val(jst0,jst1);let mut result = match result{
        Some(v) => { let mut v = js.new_str(String::from(v.as_str())).unwrap();
 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_1065006446(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in add_json_kv";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4139279264, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = jst2.get_str();


    let result = js_httpc::HttpClientBody::<Vec<u8>>::add_json_kv(jst0,jst1,jst2);let mut result = js.new_u32(result as u32);

    Some(CallResult::Ok)
}


fn call_1500292772(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in remove_json_kv";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4139279264, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


    let result = js_httpc::HttpClientBody::<Vec<u8>>::remove_json_kv(jst0,jst1);let mut result = match result{
        Some(v) => { let mut v = js.new_str(v).unwrap();
 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_2345066455(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in clear_json_kvs";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4139279264, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };


    js_httpc::HttpClientBody::<Vec<u8>>::clear_json_kvs(jst0);
    Some(CallResult::Ok)
}


fn call_1016322459(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in add_form_kv";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4139279264, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = jst2.get_str();


    let result:js_httpc::HttpClientBody<Vec<u8>> = js_httpc::HttpClientBody::<Vec<u8>>::add_form_kv(jst0,jst1,jst2);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,4139279264);


    Some(CallResult::Ok)
}


fn call_2344044784(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in add_form_file";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4139279264, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = jst2.get_str();


    let result = js_httpc::HttpClientBody::<Vec<u8>>::add_form_file(jst0,jst1,jst2);let mut result = match result{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,4139279264);

 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_2113618061(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in body";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut Vec<u8>) };


    let result:js_httpc::HttpClientBody<Vec<u8>> = js_httpc::HttpClientBody::<Vec<u8>>::body(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,4139279264);


    Some(CallResult::Ok)
}


fn call_794872933(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in body";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


    let result:js_httpc::HttpClientBody<String> = js_httpc::HttpClientBody::<String>::body(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3642917301);


    Some(CallResult::Ok)
}


fn call_965054041(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in json";

	let jst0 = &v[0];
    if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = Atom::from(jst0.get_str());


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


    let result:js_httpc::HttpClientBody<String> = js_httpc::HttpClientBody::<String>::json(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3642917301);


    Some(CallResult::Ok)
}


fn call_2118843620(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in form";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


    let result:js_httpc::HttpClientBody<String> = js_httpc::HttpClientBody::<String>::form(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3642917301);


    Some(CallResult::Ok)
}


fn call_997239765(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in create_http_client";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1131624585, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientOptions) };


    let result = js_httpc::create_http_client(jst0,jst1);let mut result = match result{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,1107924793);

 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_2282211344_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in get";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1107924793, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Arc<httpc::HttpClient>) };


	let jst1 = &v[1];
    if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = Atom::from(jst1.get_str());


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 4139279264, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };

    let jscopy = js.clone();
	let call_back = move |r: Result<(Arc<httpc::HttpClient>,httpc::HttpClientResponse),String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
	let array = js.new_array();
    let mut r_elem = r.0;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,1107924793);

js.set_index(&array, 0, &mut r_elem);
    let mut r_elem = r.1;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,606449873);

js.set_index(&array, 1, &mut r_elem);    let mut r = array;

            } ), Atom::from("sync,2282211344"));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    js_httpc::get(jst0,jst1,jst2,Box::new(call_back));
	None
}


fn call_739596726_async( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

    let param_error = "param error in get";
	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1107924793, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Arc<httpc::HttpClient>) };

	let jst1 = &v[1];
    if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = Atom::from(jst1.get_str());

	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 4139279264, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };

    let call_index = &v[3];
    if !call_index.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
    let call_index = call_index.get_u32();
    
    let jscopy = js.clone();
	let call_back = move |r: Result<(Arc<httpc::HttpClient>,httpc::HttpClientResponse),String>| {
		push_callback(jscopy.clone(), call_index, Box::new(move |js: Arc<JS>| {let mut r = match r{
        Ok(r) => { 
	let array = js.new_array();
    let mut r_elem = r.0;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,1107924793);

js.set_index(&array, 0, &mut r_elem);
    let mut r_elem = r.1;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,606449873);

js.set_index(&array, 1, &mut r_elem);    let mut r = array;
 r }
        Err(v) => { js.new_str(v + ", Result is Err").unwrap()
        }
    };

            1
        } ), None, Atom::from("call_739596726_async1"));
    };

    js_httpc::get(jst0,jst1,jst2,Box::new(call_back));
	Some(CallResult::Ok)
}


fn call_4177861558_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in get";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1107924793, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Arc<httpc::HttpClient>) };


	let jst1 = &v[1];
    if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = Atom::from(jst1.get_str());


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 3642917301, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<String>) };

    let jscopy = js.clone();
	let call_back = move |r: Result<(Arc<httpc::HttpClient>,httpc::HttpClientResponse),String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
	let array = js.new_array();
    let mut r_elem = r.0;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,1107924793);

js.set_index(&array, 0, &mut r_elem);
    let mut r_elem = r.1;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,606449873);

js.set_index(&array, 1, &mut r_elem);    let mut r = array;

            } ), Atom::from("sync,4177861558"));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    js_httpc::get(jst0,jst1,jst2,Box::new(call_back));
	None
}


fn call_2173630691_async( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

    let param_error = "param error in get";
	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1107924793, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Arc<httpc::HttpClient>) };

	let jst1 = &v[1];
    if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = Atom::from(jst1.get_str());

	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 3642917301, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<String>) };

    let call_index = &v[3];
    if !call_index.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
    let call_index = call_index.get_u32();
    
    let jscopy = js.clone();
	let call_back = move |r: Result<(Arc<httpc::HttpClient>,httpc::HttpClientResponse),String>| {
		push_callback(jscopy.clone(), call_index, Box::new(move |js: Arc<JS>| {let mut r = match r{
        Ok(r) => { 
	let array = js.new_array();
    let mut r_elem = r.0;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,1107924793);

js.set_index(&array, 0, &mut r_elem);
    let mut r_elem = r.1;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,606449873);

js.set_index(&array, 1, &mut r_elem);    let mut r = array;
 r }
        Err(v) => { js.new_str(v + ", Result is Err").unwrap()
        }
    };

            1
        } ), None, Atom::from("call_2173630691_async1"));
    };

    js_httpc::get(jst0,jst1,jst2,Box::new(call_back));
	Some(CallResult::Ok)
}


fn call_3729751590_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in post";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1107924793, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Arc<httpc::HttpClient>) };


	let jst1 = &v[1];
    if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = Atom::from(jst1.get_str());


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 4139279264, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };

    let jscopy = js.clone();
	let call_back = move |r: Result<(Arc<httpc::HttpClient>,httpc::HttpClientResponse),String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
	let array = js.new_array();
    let mut r_elem = r.0;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,1107924793);

js.set_index(&array, 0, &mut r_elem);
    let mut r_elem = r.1;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,606449873);

js.set_index(&array, 1, &mut r_elem);    let mut r = array;

            } ), Atom::from("sync,3729751590"));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    js_httpc::post(jst0,jst1,jst2,Box::new(call_back));
	None
}


fn call_1358301807_async( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

    let param_error = "param error in post";
	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1107924793, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Arc<httpc::HttpClient>) };

	let jst1 = &v[1];
    if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = Atom::from(jst1.get_str());

	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 4139279264, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };

    let call_index = &v[3];
    if !call_index.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
    let call_index = call_index.get_u32();
    
    let jscopy = js.clone();
	let call_back = move |r: Result<(Arc<httpc::HttpClient>,httpc::HttpClientResponse),String>| {
		push_callback(jscopy.clone(), call_index, Box::new(move |js: Arc<JS>| {let mut r = match r{
        Ok(r) => { 
	let array = js.new_array();
    let mut r_elem = r.0;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,1107924793);

js.set_index(&array, 0, &mut r_elem);
    let mut r_elem = r.1;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,606449873);

js.set_index(&array, 1, &mut r_elem);    let mut r = array;
 r }
        Err(v) => { js.new_str(v + ", Result is Err").unwrap()
        }
    };

            1
        } ), None, Atom::from("call_1358301807_async1"));
    };

    js_httpc::post(jst0,jst1,jst2,Box::new(call_back));
	Some(CallResult::Ok)
}


fn call_2383978915_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in post";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1107924793, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Arc<httpc::HttpClient>) };


	let jst1 = &v[1];
    if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = Atom::from(jst1.get_str());


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 3642917301, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<String>) };

    let jscopy = js.clone();
	let call_back = move |r: Result<(Arc<httpc::HttpClient>,httpc::HttpClientResponse),String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
	let array = js.new_array();
    let mut r_elem = r.0;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,1107924793);

js.set_index(&array, 0, &mut r_elem);
    let mut r_elem = r.1;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,606449873);

js.set_index(&array, 1, &mut r_elem);    let mut r = array;

            } ), Atom::from("sync,2383978915"));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    js_httpc::post(jst0,jst1,jst2,Box::new(call_back));
	None
}


fn call_3423707807_async( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

    let param_error = "param error in post";
	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1107924793, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Arc<httpc::HttpClient>) };

	let jst1 = &v[1];
    if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = Atom::from(jst1.get_str());

	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 3642917301, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<String>) };

    let call_index = &v[3];
    if !call_index.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
    let call_index = call_index.get_u32();
    
    let jscopy = js.clone();
	let call_back = move |r: Result<(Arc<httpc::HttpClient>,httpc::HttpClientResponse),String>| {
		push_callback(jscopy.clone(), call_index, Box::new(move |js: Arc<JS>| {let mut r = match r{
        Ok(r) => { 
	let array = js.new_array();
    let mut r_elem = r.0;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,1107924793);

js.set_index(&array, 0, &mut r_elem);
    let mut r_elem = r.1;
    let ptr = Box::into_raw(Box::new(r_elem)) as usize;let mut r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,606449873);

js.set_index(&array, 1, &mut r_elem);    let mut r = array;
 r }
        Err(v) => { js.new_str(v + ", Result is Err").unwrap()
        }
    };

            1
        } ), None, Atom::from("call_3423707807_async1"));
    };

    js_httpc::post(jst0,jst1,jst2,Box::new(call_back));
	Some(CallResult::Ok)
}


fn call_545958709(js: Arc<JS>) -> Option<CallResult>{

    let result = js_net::NetMgr::new();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2462173101);


    Some(CallResult::Ok)
}


fn call_471202658(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result = js_net::TlsNetMgr::new(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,4120821321);


    Some(CallResult::Ok)
}


fn call_1849109725(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 2566315655, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut js_lib::JSGray) };


    let result = js_net::NetHandler::new(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1707332364);


    Some(CallResult::Ok)
}


fn call_2637800921(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3386914360, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Arc<RwLock<gray::GrayTab<js_lib::JSGray>>>) };


    let result = js_net::TopicHandler::new(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,15779622);


    Some(CallResult::Ok)
}


fn call_3844141423(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 2566315655, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut js_lib::JSGray) };


    let result = js_net::NetEventHandler::new(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2899437702);


    Some(CallResult::Ok)
}


fn call_3244057673(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2913244961, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Arc<parking_lot::RwLock<hotfix::GrayTable>>) };


    let result = js_net::RequestHandler::new(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,4208533229);


    Some(CallResult::Ok)
}


fn call_357009886(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in mqtt_bind";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2484911420, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const mqtt_tmp::server::ServerNode) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 2462173101, false, param_error).expect("");
	let jst1 = unsafe { &mut *(ptr as *mut js_net::NetMgr) };


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = jst2.get_str();


	let jst3 = &v[3];
	if !jst3.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst3 = jst3.get_str();


	let jst4 = &v[4];
	if !jst4.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst4 = jst4.get_u32() as usize;


	let jst5 = &v[5];
	if !jst5.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst5 = jst5.get_u32() as usize;


    js_net::mqtt_bind(jst0,jst1,jst2,jst3,jst4,jst5);
    Some(CallResult::Ok)
}


fn call_3222050891(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in net_connect_bind";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2462173101, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut js_net::NetMgr) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = jst2.get_str();


	let jst3 = &v[3];
    let ptr = jstype_ptr(&jst3, js.clone(), 1707332364, false, param_error).expect("");
	let jst3 = unsafe { &*(ptr as *const js_net::NetHandler) };


	let jst4 = &v[4];
    let ptr = jstype_ptr(&jst4, js.clone(), 1707332364, false, param_error).expect("");
	let jst4 = unsafe { &*(ptr as *const js_net::NetHandler) };


    js_net::net_connect_bind(jst0,jst1,jst2,jst3,jst4);
    Some(CallResult::Ok)
}


fn call_3574413612(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in mqtt_bind_tls";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2484911420, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const mqtt_tmp::server::ServerNode) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 4120821321, false, param_error).expect("");
	let jst1 = unsafe { &mut *(ptr as *mut js_net::TlsNetMgr) };


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = jst2.get_str();


	let jst3 = &v[3];
	if !jst3.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst3 = jst3.get_str();


	let jst4 = &v[4];
	if !jst4.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst4 = jst4.get_str();


	let jst5 = &v[5];
	if !jst5.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst5 = jst5.get_str();


	let jst6 = &v[6];
	if !jst6.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst6 = jst6.get_u32() as usize;


	let jst7 = &v[7];
	if !jst7.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst7 = jst7.get_u32() as usize;


    js_net::mqtt_bind_tls(jst0,jst1,jst2,jst3,jst4,jst5,jst6,jst7);
    Some(CallResult::Ok)
}


fn call_2877879633(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in net_connect_bind_tls";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4120821321, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut js_net::TlsNetMgr) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = jst2.get_str();


	let jst3 = &v[3];
	if !jst3.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst3 = jst3.get_str();


	let jst4 = &v[4];
	if !jst4.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst4 = jst4.get_str();


	let jst5 = &v[5];
    let ptr = jstype_ptr(&jst5, js.clone(), 1707332364, false, param_error).expect("");
	let jst5 = unsafe { &*(ptr as *const js_net::NetHandler) };


	let jst6 = &v[6];
    let ptr = jstype_ptr(&jst6, js.clone(), 1707332364, false, param_error).expect("");
	let jst6 = unsafe { &*(ptr as *const js_net::NetHandler) };


    js_net::net_connect_bind_tls(jst0,jst1,jst2,jst3,jst4,jst5,jst6);
    Some(CallResult::Ok)
}


fn call_2248917003(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in clone_server_node";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2484911420, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const mqtt_tmp::server::ServerNode) };


    let result = js_net::clone_server_node(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2484911420);


    Some(CallResult::Ok)
}


fn call_3695051784(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in clone_rpc_server";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3913457295, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const rpc_tmp::server::RPCServer) };


    let result = js_net::clone_rpc_server(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3913457295);


    Some(CallResult::Ok)
}


fn call_2482429183(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in set_mqtt_topic";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2484911420, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const mqtt_tmp::server::ServerNode) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst2 = jst2.get_boolean();
    

	let jst3 = &v[3];
	if !jst3.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst3 = jst3.get_boolean();
    

    let result = js_net::set_mqtt_topic(jst0,jst1,jst2,jst3);let mut result = match result{
        Ok(r) => { let mut r = js.new_boolean(r);
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_2867121613(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in unset_mqtt_topic";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2484911420, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const mqtt_tmp::server::ServerNode) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


    let result = js_net::unset_mqtt_topic(jst0,jst1);let mut result = match result{
        Ok(r) => { 
	let array = js.new_array();    let mut r = array;
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_1551231400(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in mqtt_publish";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2484911420, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const mqtt_tmp::server::ServerNode) };


	let jst1 = &v[1];
	if !jst1.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst1 = jst1.get_boolean();
    

	let jst2 = &v[2];
    if !jst2.is_number(){return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = match jst2.get_u32(){
        0 => js_net::QoS::AtMostOnce,
        1 => js_net::QoS::AtLeastOnce,
        2 => js_net::QoS::ExactlyOnce,
        _ => panic!("enum type error")
    };


	let jst3 = &v[3];
	if !jst3.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst3 = jst3.get_str();


	let jst4 = &v[4];
	if !jst4.is_uint8_array() && !jst4.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst4 = jst4.to_bytes();



    let result = js_net::mqtt_publish(jst0,jst1,jst2,jst3,jst4);let mut result = match result{
        Ok(r) => { 
	let array = js.new_array();    let mut r = array;
 r }
        Err(v) => { 
            return Some(CallResult::Err(v.to_string() + "Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_2874114884(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in mqtt_respond";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 717646231, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Arc<mqtt_tmp::session::Session>) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_uint8_array() && !jst2.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst2 = jst2.to_bytes();



    js_net::mqtt_respond(jst0,jst1,jst2);
    Some(CallResult::Ok)
}


fn call_138660483(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in register_rpc_handler";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3913457295, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut rpc_tmp::server::RPCServer) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst2 = jst2.get_boolean();
    

	let jst3 = &v[3];
    let ptr = jstype_ptr(&jst3, js.clone(), 3776892844, false, param_error).expect("");
	let jst3 = unsafe { &*(ptr as *const Arc<js_net::TopicHandler>) };


    let result = js_net::register_rpc_handler(jst0,jst1,jst2,jst3);let mut result = match result{
        Ok(r) => { 
	let array = js.new_array();    let mut r = array;
 r }
        Err(v) => { 
            return Some(CallResult::Err(v.to_string() + "Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_527952504(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in arc_new_topic_handler";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 15779622, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut js_net::TopicHandler) };


    let result = js_net::arc_new_topic_handler(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3776892844);


    Some(CallResult::Ok)
}


fn call_2333272468(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in create_rpc_service";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4208533229, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const js_net::RequestHandler) };


    let result = js_net::create_rpc_service(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1562130667);


    Some(CallResult::Ok)
}


fn call_466468899(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in register_rcp_listener";

	let jst0 = &v[0];
    let jst0 = if jst0.is_undefined() || jst0.is_null(){
        None
    }else{
    let ptr = jstype_ptr(&jst0, js.clone(), 2899437702, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const js_net::NetEventHandler) };

        Some(jst0)
    };


	let jst1 = &v[1];
    let jst1 = if jst1.is_undefined() || jst1.is_null(){
        None
    }else{
    let ptr = jstype_ptr(&jst1, js.clone(), 2899437702, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const js_net::NetEventHandler) };

        Some(jst1)
    };


    let result = js_net::register_rcp_listener(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,619541818);


    Some(CallResult::Ok)
}


fn call_1703898312(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in register_rpc_topic";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1562130667, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const Arc<base::service::BaseService>) };


    js_net::register_rpc_topic(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_2329614290(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in unregister_rpc_topic";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


    js_net::unregister_rpc_topic(jst0);
    Some(CallResult::Ok)
}


fn call_4082873914(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in rpc_reply";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3092548949, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Arc<rpc::connect::RpcConnect>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32();


	let jst2 = &v[2];
	if !jst2.is_uint8_array() && !jst2.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst2 = jst2.to_bytes();



    js_net::rpc_reply(jst0,jst1,jst2);
    Some(CallResult::Ok)
}


fn call_2617351137(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in rpc_send";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3092548949, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Arc<rpc::connect::RpcConnect>) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_u32();


	let jst3 = &v[3];
	if !jst3.is_uint8_array() && !jst3.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst3 = jst3.to_bytes();



    js_net::rpc_send(jst0,jst1,jst2,jst3);
    Some(CallResult::Ok)
}


fn call_3293246594(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in global_mqtt_bind_tcp_ports";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


	let jst1 = &v[1];
	if !jst1.is_array(){return Some(CallResult::Err(String::from(param_error)));}
	let a_len = jst1.get_array_length();

    let mut jst1_ = Vec::new();
    for i in 0..a_len{
		let jst1_e = jst1.get_index(i as u32);
	if !jst1_e.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1_e = jst1_e.get_u16();
    jst1_.push(jst1_e);
    }
    let jst1 = jst1_.as_slice();


	let jst2 = &v[2];
	if !jst2.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_u32() as usize;


	let jst3 = &v[3];
	if !jst3.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst3 = jst3.get_u32() as usize;


	let jst4 = &v[4];
	if !jst4.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst4 = jst4.get_u32() as usize;


	let jst5 = &v[5];
	if !jst5.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst5 = jst5.get_u32() as usize;


	let jst6 = &v[6];
	if !jst6.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst6 = jst6.get_u32() as usize;


	let jst7 = &v[7];
	if !jst7.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst7 = jst7.get_u32() as usize;


	let jst8 = &v[8];
	if !jst8.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst8 = jst8.get_u32() as usize;


	let jst9 = &v[9];
	if !jst9.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst9 = jst9.get_str();


    js_net::global_mqtt_bind_tcp_ports(jst0,jst1,jst2,jst3,jst4,jst5,jst6,jst7,jst8,jst9);
    Some(CallResult::Ok)
}


fn call_3953247239(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in add_global_mqtt_topic";

	let jst0 = &v[0];
	if !jst0.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.get_boolean();
    

	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


    js_net::add_global_mqtt_topic(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_1449642520(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in publish_global_mqtt_topic";

	let jst0 = &v[0];
	if !jst0.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.get_boolean();
    

	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_uint8_array() && !jst2.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst2 = jst2.to_bytes();



    js_net::publish_global_mqtt_topic(jst0,jst1,jst2);
    Some(CallResult::Ok)
}


fn call_466051911(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2566315655, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut js_lib::JSGray) };


    let result = js_async::AsyncRequestHandler::new(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,259136547);


    Some(CallResult::Ok)
}


fn call_2208297260(js: Arc<JS>) -> Option<CallResult>{

    let result = hotfix::GrayTable::new();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2208640946);


    Some(CallResult::Ok)
}


fn call_4057105552(js: Arc<JS>) -> Option<CallResult>{

    let result = hotfix::get_gray_table();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2913244961);


    Some(CallResult::Ok)
}


fn call_1337865535(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in register_jsgray";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2913244961, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut Arc<parking_lot::RwLock<hotfix::GrayTable>>)}.clone();


	let jst1 = &v[1];
    let jst1 = if jst1.is_undefined() || jst1.is_null(){
        None
    }else{
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;

        Some(jst1)
    };


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 2566315655, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut js_lib::JSGray) };


    hotfix::register_jsgray(jst0,jst1,jst2);
    Some(CallResult::Ok)
}


fn call_1332096267(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in get_byte_code";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


    let result = hotfix::get_byte_code(jst0);let mut result = match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let mut v = ptr_jstype(js.get_objs(), js.clone(), ptr,2886438122);

 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_1590345565(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in compile_byte_code";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


    let result = hotfix::compile_byte_code(jst0,jst1);let mut result = match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let mut v = ptr_jstype(js.get_objs(), js.clone(), ptr,2886438122);

 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_3668445806(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in hotfix_listen";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


    hotfix::hotfix_listen(jst0);
    Some(CallResult::Ok)
}


fn call_451831207(js: Arc<JS>) -> Option<CallResult>{

    let result = webshell::WebShell::new();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,937567010);


    Some(CallResult::Ok)
}


fn call_3060877404(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in exec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 937567010, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const webshell::WebShell) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


    let result = webshell::WebShell::exec(jst0,jst1);let mut result = js.new_str(result).unwrap();

    Some(CallResult::Ok)
}


fn call_1752410735(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in create";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = &jst0.get_str();


    let result = js_net_rpc_client::RPCClient::create(jst0);let mut result = match result{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,4088898725);

 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_3581032719_async( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

    let param_error = "param error in connect";
	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4088898725, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const js_net_rpc_client::RPCClient) };

	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u16();

	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = &jst2.get_str();

	let jst3 = &v[3];
	if !jst3.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst3 = jst3.get_u8();

	let jst4 = &v[4];
    let jst4 = if jst4.is_undefined() || jst4.is_null(){
        None
    }else{
    let ptr = jstype_ptr(&jst4, js.clone(), 4288401962, true, param_error).expect("");
	let jst4 = *unsafe { Box::from_raw(ptr as *mut js_net_rpc_client::CloseHandler) };

        Some(jst4)
    };

    let call_index = &v[5];
    if !call_index.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
    let call_index = call_index.get_u32();
    
    let jscopy = js.clone();
	let call_back = move |r: Result<Option<Vec<u8>>,String>| {
		push_callback(jscopy.clone(), call_index, Box::new(move |js: Arc<JS>| {let mut r = match r{
        Ok(r) => { let mut r = match r{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let mut v = ptr_jstype(js.get_objs(), js.clone(), ptr,104530634);

 v}
        None => js.new_null()
    };
 r }
        Err(v) => { js.new_str(v + ", Result is Err").unwrap()
        }
    };

            1
        } ), None, Atom::from("call_3581032719_async1"));
    };

    js_net_rpc_client::RPCClient::connect(jst0,jst1,jst2,jst3,jst4,Arc::new(call_back));
	Some(CallResult::Ok)
}


fn call_3808530099_async( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

    let param_error = "param error in request";
	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4088898725, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const js_net_rpc_client::RPCClient) };

	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();

	let jst2 = &v[2];
	if !jst2.is_uint8_array() && !jst2.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst2 = jst2.to_bytes();


	let jst3 = &v[3];
	if !jst3.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst3 = jst3.get_u8();

    let call_index = &v[4];
    if !call_index.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
    let call_index = call_index.get_u32();
    
    let jscopy = js.clone();
	let call_back = move |r: Result<Option<Vec<u8>>,String>| {
		push_callback(jscopy.clone(), call_index, Box::new(move |js: Arc<JS>| {let mut r = match r{
        Ok(r) => { let mut r = match r{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let mut v = ptr_jstype(js.get_objs(), js.clone(), ptr,104530634);

 v}
        None => js.new_null()
    };
 r }
        Err(v) => { js.new_str(v + ", Result is Err").unwrap()
        }
    };

            1
        } ), None, Atom::from("call_3808530099_async1"));
    };

    js_net_rpc_client::RPCClient::request(jst0,jst1,jst2,jst3,Arc::new(call_back));
	Some(CallResult::Ok)
}


fn call_298029700(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in close";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4088898725, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const js_net_rpc_client::RPCClient) };


    js_net_rpc_client::RPCClient::close(jst0);
    Some(CallResult::Ok)
}


fn call_146889029(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = jst0.get_str();


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 2566315655, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut js_lib::JSGray) };


    let result = js_net_rpc_client::CloseHandler::new(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,4288401962);


    Some(CallResult::Ok)
}

fn drop_3289224548(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_db::DBIter) };
}

fn drop_2886438122(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<Vec<u8>>) };
}

fn drop_4252329727(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_vm::adapter::JSType) };
}

fn drop_2484911420(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut mqtt_tmp::server::ServerNode) };
}

fn drop_2627601653(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_db::DBToMqttMonitor) };
}

fn drop_1632158050(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_db::DBToGlobalMqttMonitor) };
}

fn drop_1495847839(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_db::JSDBMonitor) };
}

fn drop_3165549746(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_db::db::Event) };
}

fn drop_2976191628(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_db::mgr::Mgr) };
}

fn drop_730519735(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_vm::pi_vm_impl::VMFactory) };
}

fn drop_1754972364(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_db::mgr::Tr) };
}

fn drop_1237457629(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_db::memery_db::DB) };
}

fn drop_568147534(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_store::lmdb_file::DB) };
}

fn drop_2325173571(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_store::file_mem_db::FileMemDB) };
}

fn drop_1542823015(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Vec<String>) };
}

fn drop_4164638564(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<pi_db::db::TabMeta>) };
}

fn drop_104530634(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Vec<u8>) };
}

fn drop_4000136370(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_db::db::TabKV) };
}

fn drop_1675843967(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_db::DBWare) };
}

fn drop_1694133887(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_env::Args) };
}

fn drop_591726708(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_env::EnvVars) };
}

fn drop_1846921536(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<sinfo::StructInfo>) };
}

fn drop_259136547(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_async::AsyncRequestHandler) };
}

fn drop_374659923(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<js_async::AsyncRequestHandler>) };
}

fn drop_3355421248(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_base::Rand) };
}

fn drop_2643678751(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<pi_vm::pi_vm_impl::VMFactory>) };
}

fn drop_2566315655(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_lib::JSGray) };
}

fn drop_3386914360(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<RwLock<gray::GrayTab<js_lib::JSGray>>>) };
}

fn drop_1736136244(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut guid::GuidGen) };
}

fn drop_1131624585(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientOptions) };
}

fn drop_4139279264(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };
}

fn drop_3642917301(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<String>) };
}

fn drop_1107924793(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<httpc::HttpClient>) };
}

fn drop_606449873(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut httpc::HttpClientResponse) };
}

fn drop_2462173101(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_net::NetMgr) };
}

fn drop_4120821321(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_net::TlsNetMgr) };
}

fn drop_1707332364(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_net::NetHandler) };
}

fn drop_851644454(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<parking_lot::RwLock<gray::GrayTab<js_lib::JSGray>>>) };
}

fn drop_15779622(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_net::TopicHandler) };
}

fn drop_2899437702(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_net::NetEventHandler) };
}

fn drop_2913244961(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<parking_lot::RwLock<hotfix::GrayTable>>) };
}

fn drop_4208533229(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_net::RequestHandler) };
}

fn drop_3913457295(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut rpc_tmp::server::RPCServer) };
}

fn drop_2688700187(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_net::QoS) };
}

fn drop_717646231(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<mqtt_tmp::session::Session>) };
}

fn drop_3776892844(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<js_net::TopicHandler>) };
}

fn drop_1562130667(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<base::service::BaseService>) };
}

fn drop_619541818(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<rpc::service::RpcListener>) };
}

fn drop_3092548949(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<rpc::connect::RpcConnect>) };
}

fn drop_2208640946(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut hotfix::GrayTable) };
}

fn drop_937567010(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut webshell::WebShell) };
}

fn drop_4088898725(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_net_rpc_client::RPCClient) };
}

fn drop_4288401962(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut js_net_rpc_client::CloseHandler) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("js_db::DBIter"), drop_fn: drop_3289224548}, 3289224548);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<Vec<u8>>"), drop_fn: drop_2886438122}, 2886438122);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_vm::adapter::JSType"), drop_fn: drop_4252329727}, 4252329727);
    mgr.regist_struct_meta(StructMeta{name:String::from("mqtt_tmp::server::ServerNode"), drop_fn: drop_2484911420}, 2484911420);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_db::DBToMqttMonitor"), drop_fn: drop_2627601653}, 2627601653);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_db::DBToGlobalMqttMonitor"), drop_fn: drop_1632158050}, 1632158050);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_db::JSDBMonitor"), drop_fn: drop_1495847839}, 1495847839);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::db::Event"), drop_fn: drop_3165549746}, 3165549746);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::mgr::Mgr"), drop_fn: drop_2976191628}, 2976191628);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_vm::pi_vm_impl::VMFactory"), drop_fn: drop_730519735}, 730519735);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::mgr::Tr"), drop_fn: drop_1754972364}, 1754972364);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::memery_db::DB"), drop_fn: drop_1237457629}, 1237457629);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_store::lmdb_file::DB"), drop_fn: drop_568147534}, 568147534);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_store::file_mem_db::FileMemDB"), drop_fn: drop_2325173571}, 2325173571);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<String>"), drop_fn: drop_1542823015}, 1542823015);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<pi_db::db::TabMeta>"), drop_fn: drop_4164638564}, 4164638564);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<u8>"), drop_fn: drop_104530634}, 104530634);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::db::TabKV"), drop_fn: drop_4000136370}, 4000136370);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_db::DBWare"), drop_fn: drop_1675843967}, 1675843967);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_env::Args"), drop_fn: drop_1694133887}, 1694133887);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_env::EnvVars"), drop_fn: drop_591726708}, 591726708);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<sinfo::StructInfo>"), drop_fn: drop_1846921536}, 1846921536);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_async::AsyncRequestHandler"), drop_fn: drop_259136547}, 259136547);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<js_async::AsyncRequestHandler>"), drop_fn: drop_374659923}, 374659923);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_base::Rand"), drop_fn: drop_3355421248}, 3355421248);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<pi_vm::pi_vm_impl::VMFactory>"), drop_fn: drop_2643678751}, 2643678751);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_lib::JSGray"), drop_fn: drop_2566315655}, 2566315655);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<RwLock<gray::GrayTab<js_lib::JSGray>>>"), drop_fn: drop_3386914360}, 3386914360);
    mgr.regist_struct_meta(StructMeta{name:String::from("guid::GuidGen"), drop_fn: drop_1736136244}, 1736136244);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_httpc::HttpClientOptions"), drop_fn: drop_1131624585}, 1131624585);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_httpc::HttpClientBody<Vec<u8>>"), drop_fn: drop_4139279264}, 4139279264);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_httpc::HttpClientBody<String>"), drop_fn: drop_3642917301}, 3642917301);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<httpc::HttpClient>"), drop_fn: drop_1107924793}, 1107924793);
    mgr.regist_struct_meta(StructMeta{name:String::from("httpc::HttpClientResponse"), drop_fn: drop_606449873}, 606449873);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_net::NetMgr"), drop_fn: drop_2462173101}, 2462173101);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_net::TlsNetMgr"), drop_fn: drop_4120821321}, 4120821321);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_net::NetHandler"), drop_fn: drop_1707332364}, 1707332364);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<parking_lot::RwLock<gray::GrayTab<js_lib::JSGray>>>"), drop_fn: drop_851644454}, 851644454);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_net::TopicHandler"), drop_fn: drop_15779622}, 15779622);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_net::NetEventHandler"), drop_fn: drop_2899437702}, 2899437702);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<parking_lot::RwLock<hotfix::GrayTable>>"), drop_fn: drop_2913244961}, 2913244961);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_net::RequestHandler"), drop_fn: drop_4208533229}, 4208533229);
    mgr.regist_struct_meta(StructMeta{name:String::from("rpc_tmp::server::RPCServer"), drop_fn: drop_3913457295}, 3913457295);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_net::QoS"), drop_fn: drop_2688700187}, 2688700187);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<mqtt_tmp::session::Session>"), drop_fn: drop_717646231}, 717646231);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<js_net::TopicHandler>"), drop_fn: drop_3776892844}, 3776892844);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<base::service::BaseService>"), drop_fn: drop_1562130667}, 1562130667);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<rpc::service::RpcListener>"), drop_fn: drop_619541818}, 619541818);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<rpc::connect::RpcConnect>"), drop_fn: drop_3092548949}, 3092548949);
    mgr.regist_struct_meta(StructMeta{name:String::from("hotfix::GrayTable"), drop_fn: drop_2208640946}, 2208640946);
    mgr.regist_struct_meta(StructMeta{name:String::from("webshell::WebShell"), drop_fn: drop_937567010}, 937567010);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_net_rpc_client::RPCClient"), drop_fn: drop_4088898725}, 4088898725);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_net_rpc_client::CloseHandler"), drop_fn: drop_4288401962}, 4288401962);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3763610783_sync), 3763610783);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2701929727_sync), 2701929727);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1993779671), 1993779671);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4117819797), 4117819797);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1168492209), 1168492209);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2153620660), 2153620660);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1967373661_sync), 1967373661);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1420275781), 1420275781);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1905006775), 1905006775);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3038249291), 3038249291);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2215620835), 2215620835);
    mgr.regist_fun_meta(FnMeta::CallArg(call_360427781), 360427781);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2573413979), 2573413979);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2097131752), 2097131752);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1247562096), 1247562096);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1579404380), 1579404380);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3169463176), 3169463176);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2680255887_sync), 2680255887);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2725879080_sync), 2725879080);
    mgr.regist_fun_meta(FnMeta::CallArg(call_583163851_sync), 583163851);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2986122496_sync), 2986122496);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1869880364), 1869880364);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1719526885), 1719526885);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4281318477_sync), 4281318477);
    mgr.regist_fun_meta(FnMeta::CallArg(call_479322726_sync), 479322726);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2176133173), 2176133173);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2239806005), 2239806005);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1263843384), 1263843384);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1749960077), 1749960077);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3619493605), 3619493605);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3830865479_async), 3830865479);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2450233359), 2450233359);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1380265392), 1380265392);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4192708231), 4192708231);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2544700472), 2544700472);
    mgr.regist_fun_meta(FnMeta::Call(call_692858595), 692858595);
    mgr.regist_fun_meta(FnMeta::Call(call_76907791), 76907791);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3151666217), 3151666217);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4072555389), 4072555389);
    mgr.regist_fun_meta(FnMeta::Call(call_3300744712), 3300744712);
    mgr.regist_fun_meta(FnMeta::CallArg(call_341310298), 341310298);
    mgr.regist_fun_meta(FnMeta::Call(call_2758093424), 2758093424);
    mgr.regist_fun_meta(FnMeta::CallArg(call_215229799_async), 215229799);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3061910455_async), 3061910455);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3728513126), 3728513126);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2674074487), 2674074487);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3649129955), 3649129955);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3007613864), 3007613864);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3595492395), 3595492395);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1347190475), 1347190475);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3993207385), 3993207385);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4111533257), 4111533257);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3272869145), 3272869145);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3741531906), 3741531906);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1810043215_sync), 1810043215);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3285798497), 3285798497);
    mgr.regist_fun_meta(FnMeta::Call(call_59144274), 59144274);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3881780156), 3881780156);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3908949488), 3908949488);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2556550051), 2556550051);
    mgr.regist_fun_meta(FnMeta::CallArg(call_957759389), 957759389);
    mgr.regist_fun_meta(FnMeta::CallArg(call_370495443), 370495443);
    mgr.regist_fun_meta(FnMeta::Call(call_2041214057), 2041214057);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2697841501), 2697841501);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3635855143), 3635855143);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1199149424), 1199149424);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3557646357), 3557646357);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3906048478), 3906048478);
    mgr.regist_fun_meta(FnMeta::Call(call_373179692), 373179692);
    mgr.regist_fun_meta(FnMeta::CallArg(call_145125716), 145125716);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2887071833), 2887071833);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2011091417), 2011091417);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2937777264), 2937777264);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2175286088), 2175286088);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1065006446), 1065006446);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1500292772), 1500292772);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2345066455), 2345066455);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1016322459), 1016322459);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2344044784), 2344044784);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2113618061), 2113618061);
    mgr.regist_fun_meta(FnMeta::CallArg(call_794872933), 794872933);
    mgr.regist_fun_meta(FnMeta::CallArg(call_965054041), 965054041);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2118843620), 2118843620);
    mgr.regist_fun_meta(FnMeta::CallArg(call_997239765), 997239765);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2282211344_sync), 2282211344);
    mgr.regist_fun_meta(FnMeta::CallArg(call_739596726_async), 739596726);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4177861558_sync), 4177861558);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2173630691_async), 2173630691);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3729751590_sync), 3729751590);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1358301807_async), 1358301807);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2383978915_sync), 2383978915);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3423707807_async), 3423707807);
    mgr.regist_fun_meta(FnMeta::Call(call_545958709), 545958709);
    mgr.regist_fun_meta(FnMeta::CallArg(call_471202658), 471202658);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1849109725), 1849109725);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2637800921), 2637800921);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3844141423), 3844141423);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3244057673), 3244057673);
    mgr.regist_fun_meta(FnMeta::CallArg(call_357009886), 357009886);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3222050891), 3222050891);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3574413612), 3574413612);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2877879633), 2877879633);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2248917003), 2248917003);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3695051784), 3695051784);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2482429183), 2482429183);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2867121613), 2867121613);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1551231400), 1551231400);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2874114884), 2874114884);
    mgr.regist_fun_meta(FnMeta::CallArg(call_138660483), 138660483);
    mgr.regist_fun_meta(FnMeta::CallArg(call_527952504), 527952504);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2333272468), 2333272468);
    mgr.regist_fun_meta(FnMeta::CallArg(call_466468899), 466468899);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1703898312), 1703898312);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2329614290), 2329614290);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4082873914), 4082873914);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2617351137), 2617351137);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3293246594), 3293246594);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3953247239), 3953247239);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1449642520), 1449642520);
    mgr.regist_fun_meta(FnMeta::CallArg(call_466051911), 466051911);
    mgr.regist_fun_meta(FnMeta::Call(call_2208297260), 2208297260);
    mgr.regist_fun_meta(FnMeta::Call(call_4057105552), 4057105552);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1337865535), 1337865535);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1332096267), 1332096267);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1590345565), 1590345565);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3668445806), 3668445806);
    mgr.regist_fun_meta(FnMeta::Call(call_451831207), 451831207);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3060877404), 3060877404);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1752410735), 1752410735);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3581032719_async), 3581032719);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3808530099_async), 3808530099);
    mgr.regist_fun_meta(FnMeta::CallArg(call_298029700), 298029700);
    mgr.regist_fun_meta(FnMeta::CallArg(call_146889029), 146889029);
}