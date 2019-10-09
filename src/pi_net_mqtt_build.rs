use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use atom::Atom;
use atom;
use mqtt_tmp;



fn call_3560614167(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in respond";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2735033865, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const mqtt_tmp::session::Session) };


	let jst1 = &v[1];
    if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = Atom::from(jst1.get_str());


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 104530634, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut Vec<u8>) };


    mqtt_tmp::session::Session::respond(jst0,jst1,jst2);
    Some(CallResult::Ok)
}

fn drop_2484911420(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut mqtt_tmp::server::ServerNode) };
}

fn drop_2735033865(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut mqtt_tmp::session::Session) };
}

fn drop_104530634(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Vec<u8>) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("mqtt_tmp::server::ServerNode"), drop_fn: drop_2484911420}, 2484911420);
    mgr.regist_struct_meta(StructMeta{name:String::from("mqtt_tmp::session::Session"), drop_fn: drop_2735033865}, 2735033865);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<u8>"), drop_fn: drop_104530634}, 104530634);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3560614167), 3560614167);
}