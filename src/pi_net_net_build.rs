use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use net;



fn call_1569890377(js: Arc<JS>) -> Option<CallResult>{

    let result = net::api::NetManager::new();
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,2761082466);


    Some(CallResult::Ok)
}

fn drop_2761082466(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut net::api::NetManager) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("net::api::NetManager"), drop_fn: drop_2761082466}, 2761082466);
    mgr.regist_fun_meta(FnMeta::Call(call_1569890377), 1569890377);
}