use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use pi_vm;



fn call_2222376158(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_vm::pi_vm_impl::VMFactory";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result = pi_vm::pi_vm_impl::VMFactory::new(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,730519735);


    Some(CallResult::Ok)
}


fn call_1487978276(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_vm::pi_vm_impl::VMFactory";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 730519735, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_vm::pi_vm_impl::VMFactory) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, mgr, 2886438122, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut Arc<Vec<u8>>)}.clone();


    let result = pi_vm::pi_vm_impl::VMFactory::append(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,730519735);


    Some(CallResult::Ok)
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_vm::pi_vm_impl::VMFactory")}, 730519735);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<Vec<u8>>")}, 2886438122);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_2222376158), 2222376158);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_1487978276), 1487978276);
}