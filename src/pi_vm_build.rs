use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use atom::Atom;
use pi_vm;



fn call_2222376158(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = &jst0.get_str();


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


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
    let ptr = jstype_ptr(&jst5, js.clone(), 510245560, true, param_error).expect("");
	let jst5 = *unsafe { Box::from_raw(ptr as *mut Arc<pi_vm::bonmgr::NativeObjsAuth>)}.clone();


    let result = pi_vm::pi_vm_impl::VMFactory::new(jst0,jst1,jst2,jst3,jst4,jst5);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,730519735);


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
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,730519735);


    Some(CallResult::Ok)
}


fn call_2773712761(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in append_depend";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 730519735, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_vm::pi_vm_impl::VMFactory) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


    let result = pi_vm::pi_vm_impl::VMFactory::append_depend(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,730519735);


    Some(CallResult::Ok)
}


fn call_647083293(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in is_depend";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 730519735, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_vm::pi_vm_impl::VMFactory) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = &jst1.get_str();


    let result = pi_vm::pi_vm_impl::VMFactory::is_depend(jst0,jst1);let mut result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_1393151886(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in produce";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 730519735, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_vm::pi_vm_impl::VMFactory) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = pi_vm::pi_vm_impl::VMFactory::produce(jst0,jst1);let mut result = match result{
        Ok(r) => { let mut r = js.new_u32(r as u32);
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_54848988(js: Arc<JS>) -> Option<CallResult>{

    let result = pi_vm::bonmgr::NativeObjsAuth::with_none();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,510245560);


    Some(CallResult::Ok)
}

fn drop_510245560(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<pi_vm::bonmgr::NativeObjsAuth>) };
}

fn drop_730519735(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_vm::pi_vm_impl::VMFactory) };
}

fn drop_2886438122(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<Vec<u8>>) };
}

fn drop_3720506907(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_vm::channel_map::VMChannel) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<pi_vm::bonmgr::NativeObjsAuth>"), drop_fn: drop_510245560}, 510245560);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_vm::pi_vm_impl::VMFactory"), drop_fn: drop_730519735}, 730519735);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<Vec<u8>>"), drop_fn: drop_2886438122}, 2886438122);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_vm::channel_map::VMChannel"), drop_fn: drop_3720506907}, 3720506907);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2222376158), 2222376158);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1487978276), 1487978276);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2773712761), 2773712761);
    mgr.regist_fun_meta(FnMeta::CallArg(call_647083293), 647083293);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1393151886), 1393151886);
    mgr.regist_fun_meta(FnMeta::Call(call_54848988), 54848988);
}