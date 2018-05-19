use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use pi_vm::pi_vm_impl::{ block_reply};
use pi_vm::task::TaskType;
use pi_test;



fn call_4128819759(js: Arc<JS>) -> Option<CallResult>{

	let result = pi_test::tsvm::Position::get();let result = js.new_u8(result);

    Some(CallResult::Ok)
}


fn call_1030029618(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 4142560269, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_test::tsvm::Position) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, mgr, 4142560269, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_test::tsvm::Position) };


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, mgr, 1918243293, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut Arc<pi_test::tsvm::Position>)}.clone();


	let result = pi_test::tsvm::test_nobj_param(jst0,jst1,jst2);let result = js.new_str(result);

    Some(CallResult::Ok)
}


fn call_2422638428(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_str();


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32();


	let result = pi_test::tsvm::test_nobj_result(jst0,jst1);
	let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr, js.clone(), ptr,4142560269);

    Some(CallResult::Ok)
}


fn call_3957523241(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u8();


	let result = pi_test::tsvm::test_u8(jst0);let result = js.new_u8(result);

    Some(CallResult::Ok)
}


fn call_3150791809(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u16();


	let result = pi_test::tsvm::test_u16(jst0);let result = js.new_u16(result);

    Some(CallResult::Ok)
}


fn call_490420216(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32();


	let result = pi_test::tsvm::test_u32(jst0);let result = js.new_u32(result);

    Some(CallResult::Ok)
}


fn call_2515372555(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u64();


	let result = pi_test::tsvm::test_u64(jst0);let result = js.new_u64(result);

    Some(CallResult::Ok)
}


fn call_4256005366(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


	let result = pi_test::tsvm::test_usize(jst0);let result = js.new_u32(result as u32);

    Some(CallResult::Ok)
}


fn call_3062287738(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_i8();


	let result = pi_test::tsvm::test_i8(jst0);let result = js.new_i8(result);

    Some(CallResult::Ok)
}


fn call_1310922606(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_i16();


	let result = pi_test::tsvm::test_i16(jst0);let result = js.new_i16(result);

    Some(CallResult::Ok)
}


fn call_1682298782(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_i32();


	let result = pi_test::tsvm::test_i32(jst0);let result = js.new_i32(result);

    Some(CallResult::Ok)
}


fn call_256815972(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_i64();


	let result = pi_test::tsvm::test_i64(jst0);let result = js.new_i64(result);

    Some(CallResult::Ok)
}


fn call_1073100712(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_i32() as isize;


	let result = pi_test::tsvm::test_isize(jst0);let result = js.new_i32(result as i32);

    Some(CallResult::Ok)
}


fn call_512624395(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
	let jst0 = jst0.get_boolean();


	let result = pi_test::tsvm::test_bool(jst0);let result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_3800674060(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_str();


	let result = pi_test::tsvm::test_string(jst0);let result = js.new_str(result);

    Some(CallResult::Ok)
}


fn call_3082418051(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = &jst0.get_str();


	let result = pi_test::tsvm::test_str(jst0);let result = js.new_str(String::from(result));

    Some(CallResult::Ok)
}


fn call_157062202(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_f32();


	let result = pi_test::tsvm::test_f32(jst0);let result = js.new_f32(result);

    Some(CallResult::Ok)
}


fn call_221802902(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_f64();


	let result = pi_test::tsvm::test_f64(jst0);let result = js.new_f64(result);

    Some(CallResult::Ok)
}


fn call_2141422(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0_ = jst0.to_bytes();
	if jst0_.len() != 2{return Some(CallResult::Err(String::from(param_error))); }
    let mut jst0 = [0u8; 2];jst0.copy_from_slice(jst0_);



	let result = pi_test::tsvm::test_u8arr1(jst0);let result = js.new_array_buffer(result.len() as u32).from_bytes(&result);

    Some(CallResult::Ok)
}


fn call_2808517302(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();
    if jst0.len() != 2{return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = unsafe{*(jst0.as_ptr() as * const [u8; 2])};
    let jst0 = &jst0;



	let result = pi_test::tsvm::test_u8arr2(jst0);let result = js.new_array_buffer(result.len() as u32).from_bytes(result);

    Some(CallResult::Ok)
}


fn call_3244427237(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pi_test::tsvm";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



	let result = pi_test::tsvm::test_u8arr3(jst0);let result = js.new_array_buffer(result.len() as u32).from_bytes(result);

    Some(CallResult::Ok)
}
pub fn register(mgr: &mut BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_test::tsvm::Position")}, 4142560269);
    mgr.regist_fun_meta(FnMeta::Call(call_4128819759), 4128819759);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_1030029618), 1030029618);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_2422638428), 2422638428);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3957523241), 3957523241);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3150791809), 3150791809);
    mgr.regist_fun_meta(FnMeta::CallArg(call_490420216), 490420216);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2515372555), 2515372555);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4256005366), 4256005366);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3062287738), 3062287738);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1310922606), 1310922606);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1682298782), 1682298782);
    mgr.regist_fun_meta(FnMeta::CallArg(call_256815972), 256815972);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1073100712), 1073100712);
    mgr.regist_fun_meta(FnMeta::CallArg(call_512624395), 512624395);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3800674060), 3800674060);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3082418051), 3082418051);
    mgr.regist_fun_meta(FnMeta::CallArg(call_157062202), 157062202);
    mgr.regist_fun_meta(FnMeta::CallArg(call_221802902), 221802902);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2141422), 2141422);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2808517302), 2808517302);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3244427237), 3244427237);
}