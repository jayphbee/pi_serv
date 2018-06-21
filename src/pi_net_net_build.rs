use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use net;



fn call_1569890377(js: Arc<JS>, mgr: &BonMgr) -> Option<CallResult>{

    let result = net::api::NetManager::new();
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,2761082466);


    Some(CallResult::Ok)
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("net::api::NetManager")}, 2761082466);
    mgr.regist_fun_meta(FnMeta::CallNobj(call_1569890377), 1569890377);
}