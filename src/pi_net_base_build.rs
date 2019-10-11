use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use atom::Atom;
use base;


fn drop_2024423271(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut base::service::BaseListener) };
}

fn drop_4212103719(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut base::service::BaseService) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("base::service::BaseListener"), drop_fn: drop_2024423271}, 2024423271);
    mgr.regist_struct_meta(StructMeta{name:String::from("base::service::BaseService"), drop_fn: drop_4212103719}, 4212103719);
}