use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use pi_vm::pi_vm_impl::{ block_reply, block_throw};
use pi_vm::task::TaskType;
use core;
use core::convert::From;
use pi_lib;



fn call_1549520222(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_lib::atom::Atom";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_str();


    let result = pi_lib::atom::Atom::from(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,1411051473);


    Some(CallResult::Ok)
}


fn call_1469354144(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_lib::guid::GuidGen";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u64();


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u16();


    let result = pi_lib::guid::GuidGen::new(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,1706731228);


    Some(CallResult::Ok)
}


fn call_748243846(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_lib::sinfo::StructInfo";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 1411051473, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_lib::atom::Atom) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32();


    let result = pi_lib::sinfo::StructInfo::new(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,3099464219);


    Some(CallResult::Ok)
}


fn call_3869000731(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_lib::bon::ReadBuffer";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = pi_lib::bon::ReadBuffer::new(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,560651107);


    Some(CallResult::Ok)
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_lib::atom::Atom")}, 1411051473);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_lib::guid::GuidGen")}, 1706731228);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_lib::sinfo::StructInfo")}, 3099464219);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_lib::bon::ReadBuffer")}, 560651107);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_1549520222), 1549520222);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_1469354144), 1469354144);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_748243846), 748243846);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3869000731), 3869000731);
}