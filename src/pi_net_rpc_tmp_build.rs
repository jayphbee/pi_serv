use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use atom::Atom;
use mqtt_tmp;
use rpc_tmp;



fn call_193751450(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2484911420, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut mqtt_tmp::server::ServerNode) };


    let result = rpc_tmp::server::RPCServer::new(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3913457295);


    Some(CallResult::Ok)
}

fn drop_2484911420(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut mqtt_tmp::server::ServerNode) };
}

fn drop_3913457295(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut rpc_tmp::server::RPCServer) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("mqtt_tmp::server::ServerNode"), drop_fn: drop_2484911420}, 2484911420);
    mgr.regist_struct_meta(StructMeta{name:String::from("rpc_tmp::server::RPCServer"), drop_fn: drop_3913457295}, 3913457295);
    mgr.regist_fun_meta(FnMeta::CallArg(call_193751450), 193751450);
}