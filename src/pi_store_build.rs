use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use pi_base::task::TaskType;
use pi_vm::pi_vm_impl::{block_reply, block_throw};
use std::sync::Arc;
use pi_lib::atom::Atom;
use pi_lib;
use pi_store;



fn call_1140526407(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1411051473, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_lib::atom::Atom) };


    let result = pi_store::db::DB::new(jst0);let result = match result{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let r = ptr_jstype(js.get_objs(), js.clone(), ptr,4204700632);

 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}

fn drop_1411051473(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_lib::atom::Atom) };
}

fn drop_4204700632(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_store::db::DB) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_lib::atom::Atom"), drop_fn: drop_1411051473}, 1411051473);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_store::db::DB"), drop_fn: drop_4204700632}, 4204700632);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1140526407), 1140526407);
}