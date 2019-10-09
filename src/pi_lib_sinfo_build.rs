use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use std::mem::transmute;
use atom::Atom;
use atom;
use sinfo;



fn call_1549381160(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
    if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = Atom::from(jst0.get_str());


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32();


    let result = sinfo::StructInfo::new(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2453199836);


    Some(CallResult::Ok)
}

fn drop_2453199836(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut sinfo::StructInfo) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("sinfo::StructInfo"), drop_fn: drop_2453199836}, 2453199836);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1549381160), 1549381160);
}