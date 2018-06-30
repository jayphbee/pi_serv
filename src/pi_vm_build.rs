use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use pi_vm;



fn call_2222376158(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 510245560, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut Arc<pi_vm::bonmgr::NativeObjsAuth>)}.clone();


    let result = pi_vm::pi_vm_impl::VMFactory::new(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,730519735);


    Some(CallResult::Ok)
}


fn call_1487978276(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in append";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 730519735, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_vm::pi_vm_impl::VMFactory) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 2886438122, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut Arc<Vec<u8>>)}.clone();


    let result = pi_vm::pi_vm_impl::VMFactory::append(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,730519735);


    Some(CallResult::Ok)
}


fn call_1350440529(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in response";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3720506907, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_vm::channel_map::VMChannel) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32();


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 2886438122, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut Arc<Vec<u8>>)}.clone();


	let jst3 = &v[3];
    let ptr = jstype_ptr(&jst3, js.clone(), 3703958710, true, param_error).expect("");
	let jst3 = *unsafe { Box::from_raw(ptr as *mut Vec<pi_vm::adapter::JSType>) };


    let result = pi_vm::channel_map::VMChannel::response(jst0,jst1,jst2,jst3);let result = js.new_boolean(result);

    Some(CallResult::Ok)
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<pi_vm::bonmgr::NativeObjsAuth>")}, 510245560);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_vm::pi_vm_impl::VMFactory")}, 730519735);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<Vec<u8>>")}, 2886438122);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_vm::channel_map::VMChannel")}, 3720506907);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<pi_vm::adapter::JSType>")}, 3703958710);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2222376158), 2222376158);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1487978276), 1487978276);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1350440529), 1350440529);
}