use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use atom::Atom;
use rpc;



fn call_4030884866(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in get_local_ip";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2855847321, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const rpc::connect::RpcConnect) };


    let result = rpc::connect::RpcConnect::get_local_ip(jst0);let mut result = match result{
        Some(v) => { let mut v = js.new_str(v).unwrap();
 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_711780732(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in get_local_port";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2855847321, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const rpc::connect::RpcConnect) };


    let result = rpc::connect::RpcConnect::get_local_port(jst0);let mut result = match result{
        Some(v) => { let mut v = js.new_u16(v);
 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_747133793(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in get_remote_ip";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2855847321, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const rpc::connect::RpcConnect) };


    let result = rpc::connect::RpcConnect::get_remote_ip(jst0);let mut result = match result{
        Some(v) => { let mut v = js.new_str(v).unwrap();
 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_1246857552(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in get_remote_port";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2855847321, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const rpc::connect::RpcConnect) };


    let result = rpc::connect::RpcConnect::get_remote_port(jst0);let mut result = match result{
        Some(v) => { let mut v = js.new_u16(v);
 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}

fn drop_101304093(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut rpc::service::RpcListener) };
}

fn drop_3036387747(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut rpc::service::RpcService) };
}

fn drop_2855847321(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut rpc::connect::RpcConnect) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("rpc::service::RpcListener"), drop_fn: drop_101304093}, 101304093);
    mgr.regist_struct_meta(StructMeta{name:String::from("rpc::service::RpcService"), drop_fn: drop_3036387747}, 3036387747);
    mgr.regist_struct_meta(StructMeta{name:String::from("rpc::connect::RpcConnect"), drop_fn: drop_2855847321}, 2855847321);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4030884866), 4030884866);
    mgr.regist_fun_meta(FnMeta::CallArg(call_711780732), 711780732);
    mgr.regist_fun_meta(FnMeta::CallArg(call_747133793), 747133793);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1246857552), 1246857552);
}