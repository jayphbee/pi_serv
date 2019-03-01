use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use atom::Atom;
use pi_p2p;



fn call_2634481422(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in connect";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3035778520, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_p2p::manage::P2PManage) };


    pi_p2p::manage::P2PManage::connect(jst0);
    Some(CallResult::Ok)
}


fn call_338675993(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in connect_addr";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3035778520, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_p2p::manage::P2PManage) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = &jst1.get_str();


    pi_p2p::manage::P2PManage::connect_addr(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_696058749(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in broadcast_addr";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3035778520, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_p2p::manage::P2PManage) };


    pi_p2p::manage::P2PManage::broadcast_addr(jst0);
    Some(CallResult::Ok)
}

fn drop_3035778520(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_p2p::manage::P2PManage) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_p2p::manage::P2PManage"), drop_fn: drop_3035778520}, 3035778520);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2634481422), 2634481422);
    mgr.regist_fun_meta(FnMeta::CallArg(call_338675993), 338675993);
    mgr.regist_fun_meta(FnMeta::CallArg(call_696058749), 696058749);
}