use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use pi_vm::pi_vm_impl::{ block_reply, block_throw};
use pi_base::task::TaskType;
use pi_lib::atom::Atom;
use pi_vm;
use pi_db;
use pi_lib;
use net;
use mqtt;
use rpc;
use std::io::Error;
use depend;
use util;
use handler;
use js_call;



fn call_2239806005(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in util";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = &jst0.get_str();


    let result = util::read_file(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,104530634);


    Some(CallResult::Ok)
}


fn call_3133367430(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in handler::TopicHandler";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, mgr, 730519735, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut pi_vm::pi_vm_impl::VMFactory) };


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, mgr, 2976191628, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut pi_db::mgr::Mgr) };


    let result = handler::TopicHandler::new(jst0,jst1,jst2);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,585492653);


    Some(CallResult::Ok)
}


fn call_3825824874_sync( js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in js_call::DBIter";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 517878327, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut js_call::DBIter) };

    let jscopy = js.clone();
    let objs_ref = mgr.objs_ref.clone();
    let objs = mgr.objs.clone();
	let call_back = move |r: Result<Option<(Arc<Vec<u8>>,Arc<Vec<u8>>)>,String>| {
        let objs_ref = objs_ref.clone();
        let objs = objs.clone();
		block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
    match r{
        Ok(r) => { 
    match r{
        Some(v) => { 
	let array = js.new_array();
    let v_elem = v.0;
    let ptr = Box::into_raw(Box::new(v_elem)) as usize;let v_elem = ptr_jstype(objs.clone(), js.clone(), ptr,2886438122);

js.set_index(&array, 0, &v_elem);
    let v_elem = v.1;
    let ptr = Box::into_raw(Box::new(v_elem)) as usize;let v_elem = ptr_jstype(objs.clone(), js.clone(), ptr,2886438122);

js.set_index(&array, 1, &v_elem);    let v = array; }
        None => { let r = js.new_undefined(); }
    };
 }
        Err(v) => { 
            block_throw(js.clone(), v.to_string() + "Result is Err", TaskType::Sync, 10, Atom::from("block throw task"));
        }
    };

        } ), TaskType::Sync, 10, Atom::from(""));
    };
    let r = js_call::DBIter::next(jst0,Arc::new(call_back));
	if r.is_some(){
        let r = r.unwrap();
    match r{
        Ok(r) => { 
    match r{
        Some(v) => { 
	let array = js.new_array();
    let v_elem = v.0;
    let ptr = Box::into_raw(Box::new(v_elem)) as usize;let v_elem = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,2886438122);

js.set_index(&array, 0, &v_elem);
    let v_elem = v.1;
    let ptr = Box::into_raw(Box::new(v_elem)) as usize;let v_elem = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,2886438122);

js.set_index(&array, 1, &v_elem);    let v = array; }
        None => { let r = js.new_undefined(); }
    };
 return Some(CallResult::Ok); }
        Err(v) => { 
            return Some(CallResult::Err(v + "Result is Err"));
        }
    };

    }
	None
}


fn call_158707721_sync( js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in js_call";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 1754972364, false, param_error).expect("");
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
    let objs_ref = mgr.objs_ref.clone();
    let objs = mgr.objs.clone();
	let call_back = move |r: Result<js_call::DBIter,String>| {
        let objs_ref = objs_ref.clone();
        let objs = objs.clone();
		block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
    match r{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let r = ptr_jstype(objs.clone(), js.clone(), ptr,517878327);

 }
        Err(v) => { 
            block_throw(js.clone(), v.to_string() + "Result is Err", TaskType::Sync, 10, Atom::from("block throw task"));
        }
    };

        } ), TaskType::Sync, 10, Atom::from(""));
    };
    let r = js_call::iter_db(jst0,jst1,jst2,jst3,jst4,jst5,Arc::new(call_back));
	if r.is_some(){
        let r = r.unwrap();
    match r{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let r = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,517878327);

 return Some(CallResult::Ok); }
        Err(v) => { 
            return Some(CallResult::Err(v + "Result is Err"));
        }
    };

    }
	None
}


fn call_3284237535(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in js_call";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 2976191628, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_str();


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, mgr, 3176709138, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut pi_db::memery_db::MemeryDB) };


    let result = js_call::register_db(jst0,jst1,jst2);let result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_1995451612(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in js_call";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = js_call::create_sinfo(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,1721307497);


    Some(CallResult::Ok)
}


fn call_3189416152(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in js_call";

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



    let result = js_call::tabkv_with_value(jst0,jst1,jst2,jst3);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,4000136370);


    Some(CallResult::Ok)
}


fn call_1338391149(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in js_call";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = &jst0.get_str();


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = &jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_uint8_array() && !jst2.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst2 = jst2.to_bytes();



    let result = js_call::tabkv_new(jst0,jst1,jst2);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,4000136370);


    Some(CallResult::Ok)
}


fn call_2340393156(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in js_call";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 4000136370, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::db::TabKV) };


    let result = js_call::tabkv_get_value(jst0);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,2886438122);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_1209559845(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in js_call";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 730519735, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_vm::pi_vm_impl::VMFactory) };


    let result = js_call::clone_vm_factory(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,730519735);


    Some(CallResult::Ok)
}


fn call_56622988(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in js_call";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 2761082466, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const net::api::NetManager) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_str();


	let jst3 = &v[3];
	if !jst3.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst3 = jst3.get_u32() as usize;


	let jst4 = &v[4];
	if !jst4.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst4 = jst4.get_u32() as usize;


    let result = js_call::mqtt_bind(jst0,jst1,jst2,jst3,jst4);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,1751456239);


    Some(CallResult::Ok)
}


fn call_3661222231(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in js_call";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 226971089, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut Arc<mqtt::session::Session>)}.clone();


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_uint8_array() && !jst2.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst2 = jst2.to_bytes();



    js_call::mqtt_respond(jst0,jst1,jst2);
    Some(CallResult::Ok)
}


fn call_1204956194(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in js_call";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 1285687456, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut rpc::server::RPCServer) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_str();


	let jst2 = &v[2];
	if !jst2.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst2 = jst2.get_boolean();
    

	let jst3 = &v[3];
    let ptr = jstype_ptr(&jst3, mgr, 471985604, false, param_error).expect("");
	let jst3 = unsafe { &*(ptr as *const Arc<handler::TopicHandler>) };


    let result = js_call::register_rpc_handler(jst0,jst1,jst2,jst3);
    match result{
        Ok(r) => { 
	let array = js.new_array();    let result = array; return Some(CallResult::Ok); }
        Err(v) => { 
            return Some(CallResult::Err(String::from("Result is Err")));
        }
    };

    Some(CallResult::Ok)
}


fn call_690562975(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in js_call";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 585492653, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut handler::TopicHandler) };


    let result = js_call::arc_new(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,471985604);


    Some(CallResult::Ok)
}


fn call_1613784573(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in js_call";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 2886438122, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Arc<Vec<u8>>) };


    let result = js_call::arc_deref(jst0);
    let ptr = result as *const Vec<u8> as usize;let result = ptr_jstype(mgr.objs_ref.clone(), js.clone(), ptr,104530634);


    Some(CallResult::Ok)
}


fn call_1394145511(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in js_call";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 1797798710, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const depend::Depend) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_str();


    let result = js_call::get_depend(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,1542823015);


    Some(CallResult::Ok)
}


fn call_2244240226_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in js_call";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u64();

    let jscopy = js.clone();
    let call_back = move || {
		block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {js.new_undefined();}), TaskType::Sync, 10, Atom::from(""));
    };
    js_call::js_sleep(jst0,Box::new(call_back));
	None
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<u8>")}, 104530634);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_vm::pi_vm_impl::VMFactory")}, 730519735);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::mgr::Mgr")}, 2976191628);
    mgr.regist_struct_meta(StructMeta{name:String::from("handler::TopicHandler")}, 585492653);
    mgr.regist_struct_meta(StructMeta{name:String::from("js_call::DBIter")}, 517878327);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<Fn>")}, 676023733);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<Vec<u8>>")}, 2886438122);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::mgr::Tr")}, 1754972364);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::memery_db::MemeryDB")}, 3176709138);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<pi_lib::sinfo::StructInfo>")}, 1721307497);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::db::TabKV")}, 4000136370);
    mgr.regist_struct_meta(StructMeta{name:String::from("net::api::NetManager")}, 2761082466);
    mgr.regist_struct_meta(StructMeta{name:String::from("mqtt::server::ServerNode")}, 1751456239);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<mqtt::session::Session>")}, 226971089);
    mgr.regist_struct_meta(StructMeta{name:String::from("rpc::server::RPCServer")}, 1285687456);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<handler::TopicHandler>")}, 471985604);
    mgr.regist_struct_meta(StructMeta{name:String::from("depend::Depend")}, 1797798710);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<String>")}, 1542823015);
    mgr.regist_struct_meta(StructMeta{name:String::from("Box<FnBox>")}, 1255307008);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_2239806005), 2239806005);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3133367430), 3133367430);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3825824874_sync), 3825824874);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_158707721_sync), 158707721);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3284237535), 3284237535);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_1995451612), 1995451612);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3189416152), 3189416152);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_1338391149), 1338391149);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_2340393156), 2340393156);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_1209559845), 1209559845);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_56622988), 56622988);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3661222231), 3661222231);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_1204956194), 1204956194);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_690562975), 690562975);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_1613784573), 1613784573);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_1394145511), 1394145511);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2244240226_sync), 2244240226);
}