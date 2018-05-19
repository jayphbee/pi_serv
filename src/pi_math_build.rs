use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use pi_vm::pi_vm_impl::{ block_reply};
use pi_vm::task::TaskType;
use pi_math;



fn call_2798870758(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_math::hash::H32";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 3974239134, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_math::hash::H32) };


	let result = pi_math::hash::H32::take(jst0);let result = js.new_array_buffer(result.len() as u32).from_bytes(&result);

    Some(CallResult::Ok)
}


fn call_767388297(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_math::hash::H32";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 3974239134, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H32) };


	let result = pi_math::hash::H32::tohex(jst0);let result = js.new_str(result);

    Some(CallResult::Ok)
}


fn call_3292766157(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_math::hash::H160";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 3995272273, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_math::hash::H160) };


	let result = pi_math::hash::H160::take(jst0);let result = js.new_array_buffer(result.len() as u32).from_bytes(&result);

    Some(CallResult::Ok)
}


fn call_1334624721(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_math::hash::H160";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 3995272273, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H160) };


	let result = pi_math::hash::H160::tohex(jst0);let result = js.new_str(result);

    Some(CallResult::Ok)
}


fn call_2454669575(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_math::hash::H256";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 526967798, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_math::hash::H256) };


	let result = pi_math::hash::H256::take(jst0);let result = js.new_array_buffer(result.len() as u32).from_bytes(&result);

    Some(CallResult::Ok)
}


fn call_3197660783(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_math::hash::H256";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 526967798, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H256) };


	let result = pi_math::hash::H256::tohex(jst0);let result = js.new_str(result);

    Some(CallResult::Ok)
}


fn call_3783275301(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_math::hash::H512";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 2521161042, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_math::hash::H512) };


	let result = pi_math::hash::H512::take(jst0);let result = js.new_array_buffer(result.len() as u32).from_bytes(&result);

    Some(CallResult::Ok)
}


fn call_3697048694(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_math::hash::H512";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 2521161042, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H512) };


	let result = pi_math::hash::H512::tohex(jst0);let result = js.new_str(result);

    Some(CallResult::Ok)
}
pub fn register(mgr: &mut BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_math::hash::H32")}, 3974239134);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_math::hash::H160")}, 3995272273);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_math::hash::H256")}, 526967798);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_math::hash::H512")}, 2521161042);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_2798870758), 2798870758);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_767388297), 767388297);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3292766157), 3292766157);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_1334624721), 1334624721);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_2454669575), 2454669575);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3197660783), 3197660783);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3783275301), 3783275301);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3697048694), 3697048694);
}