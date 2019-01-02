use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use pi_vm::pi_vm_impl::{ block_reply, block_throw, push_callback};
use worker::task::TaskType;
use atom::Atom;
use std::mem::{transmute, uninitialized};
use atom;
use pi_store;



fn call_4027749383(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 913748025, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut atom::Atom) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = pi_store::lmdb_file::DB::new(jst0,jst1);let mut result = match result{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,568147534);

 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}

fn drop_913748025(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut atom::Atom) };
}

fn drop_568147534(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_store::lmdb_file::DB) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("atom::Atom"), drop_fn: drop_913748025}, 913748025);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_store::lmdb_file::DB"), drop_fn: drop_568147534}, 568147534);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4027749383), 4027749383);
}