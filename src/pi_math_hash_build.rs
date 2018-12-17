use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use hash_value;



fn call_3776200155(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in take";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3383104515, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut hash_value::H32) };


    let result = hash_value::H32::take(jst0);
    let result_jstype = js.new_uint8_array(result.len() as u32);result_jstype.from_bytes(&result);let mut result = result_jstype;

    Some(CallResult::Ok)
}


fn call_2655400174(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tohex";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3383104515, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const hash_value::H32) };


    let result = hash_value::H32::tohex(jst0);let mut result = js.new_str(result);
    

    Some(CallResult::Ok)
}


fn call_444306144(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in from_buf";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = hash_value::H32::from_buf(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3383104515);


    Some(CallResult::Ok)
}


fn call_640799706(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in fromhex";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = &jst0.get_str();


    let result = hash_value::H32::fromhex(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3383104515);


    Some(CallResult::Ok)
}


fn call_4080635904(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in cmp";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3383104515, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const hash_value::H32) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 3383104515, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const hash_value::H32) };


    let result = hash_value::H32::cmp(jst0,jst1);let mut result = js.new_i8(result);

    Some(CallResult::Ok)
}


fn call_702574989(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in take";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3370757073, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut hash_value::H48) };


    let result = hash_value::H48::take(jst0);
    let result_jstype = js.new_uint8_array(result.len() as u32);result_jstype.from_bytes(&result);let mut result = result_jstype;

    Some(CallResult::Ok)
}


fn call_3679689648(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tohex";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3370757073, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const hash_value::H48) };


    let result = hash_value::H48::tohex(jst0);let mut result = js.new_str(result);
    

    Some(CallResult::Ok)
}


fn call_1365606331(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in from_buf";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = hash_value::H48::from_buf(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3370757073);


    Some(CallResult::Ok)
}


fn call_3968952281(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in fromhex";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = &jst0.get_str();


    let result = hash_value::H48::fromhex(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3370757073);


    Some(CallResult::Ok)
}


fn call_3601455854(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in cmp";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3370757073, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const hash_value::H48) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 3370757073, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const hash_value::H48) };


    let result = hash_value::H48::cmp(jst0,jst1);let mut result = js.new_i8(result);

    Some(CallResult::Ok)
}


fn call_3927864906(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in take";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1176882760, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut hash_value::H160) };


    let result = hash_value::H160::take(jst0);
    let result_jstype = js.new_uint8_array(result.len() as u32);result_jstype.from_bytes(&result);let mut result = result_jstype;

    Some(CallResult::Ok)
}


fn call_1290828860(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tohex";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1176882760, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const hash_value::H160) };


    let result = hash_value::H160::tohex(jst0);let mut result = js.new_str(result);
    

    Some(CallResult::Ok)
}


fn call_3024861218(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in from_buf";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = hash_value::H160::from_buf(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1176882760);


    Some(CallResult::Ok)
}


fn call_3933772904(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in fromhex";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = &jst0.get_str();


    let result = hash_value::H160::fromhex(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1176882760);


    Some(CallResult::Ok)
}


fn call_1395856100(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in cmp";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1176882760, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const hash_value::H160) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1176882760, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const hash_value::H160) };


    let result = hash_value::H160::cmp(jst0,jst1);let mut result = js.new_i8(result);

    Some(CallResult::Ok)
}


fn call_4065712353(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in take";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1035403249, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut hash_value::H256) };


    let result = hash_value::H256::take(jst0);
    let result_jstype = js.new_uint8_array(result.len() as u32);result_jstype.from_bytes(&result);let mut result = result_jstype;

    Some(CallResult::Ok)
}


fn call_4173242765(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tohex";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1035403249, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const hash_value::H256) };


    let result = hash_value::H256::tohex(jst0);let mut result = js.new_str(result);
    

    Some(CallResult::Ok)
}


fn call_155840530(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in from_buf";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = hash_value::H256::from_buf(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1035403249);


    Some(CallResult::Ok)
}


fn call_1103597055(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in fromhex";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = &jst0.get_str();


    let result = hash_value::H256::fromhex(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1035403249);


    Some(CallResult::Ok)
}


fn call_234413511(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in cmp";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1035403249, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const hash_value::H256) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1035403249, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const hash_value::H256) };


    let result = hash_value::H256::cmp(jst0,jst1);let mut result = js.new_i8(result);

    Some(CallResult::Ok)
}


fn call_3659472819(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in take";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3223866506, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut hash_value::H512) };


    let result = hash_value::H512::take(jst0);
    let result_jstype = js.new_uint8_array(result.len() as u32);result_jstype.from_bytes(&result);let mut result = result_jstype;

    Some(CallResult::Ok)
}


fn call_768602447(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tohex";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3223866506, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const hash_value::H512) };


    let result = hash_value::H512::tohex(jst0);let mut result = js.new_str(result);
    

    Some(CallResult::Ok)
}


fn call_3857186173(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in from_buf";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = hash_value::H512::from_buf(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3223866506);


    Some(CallResult::Ok)
}


fn call_2370856657(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in fromhex";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = &jst0.get_str();


    let result = hash_value::H512::fromhex(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3223866506);


    Some(CallResult::Ok)
}


fn call_3293705496(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in cmp";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3223866506, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const hash_value::H512) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 3223866506, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const hash_value::H512) };


    let result = hash_value::H512::cmp(jst0,jst1);let mut result = js.new_i8(result);

    Some(CallResult::Ok)
}


fn call_3987195607(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in take";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2313040707, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut hash_value::H520) };


    let result = hash_value::H520::take(jst0);
    let result_jstype = js.new_uint8_array(result.len() as u32);result_jstype.from_bytes(&result);let mut result = result_jstype;

    Some(CallResult::Ok)
}


fn call_1178325458(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tohex";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2313040707, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const hash_value::H520) };


    let result = hash_value::H520::tohex(jst0);let mut result = js.new_str(result);
    

    Some(CallResult::Ok)
}


fn call_57500088(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in from_buf";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = hash_value::H520::from_buf(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2313040707);


    Some(CallResult::Ok)
}


fn call_1046314649(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in fromhex";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = &jst0.get_str();


    let result = hash_value::H520::fromhex(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2313040707);


    Some(CallResult::Ok)
}


fn call_1817272802(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in cmp";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2313040707, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const hash_value::H520) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 2313040707, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const hash_value::H520) };


    let result = hash_value::H520::cmp(jst0,jst1);let mut result = js.new_i8(result);

    Some(CallResult::Ok)
}

fn drop_3383104515(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut hash_value::H32) };
}

fn drop_3370757073(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut hash_value::H48) };
}

fn drop_1176882760(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut hash_value::H160) };
}

fn drop_1035403249(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut hash_value::H256) };
}

fn drop_3223866506(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut hash_value::H512) };
}

fn drop_2313040707(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut hash_value::H520) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("hash_value::H32"), drop_fn: drop_3383104515}, 3383104515);
    mgr.regist_struct_meta(StructMeta{name:String::from("hash_value::H48"), drop_fn: drop_3370757073}, 3370757073);
    mgr.regist_struct_meta(StructMeta{name:String::from("hash_value::H160"), drop_fn: drop_1176882760}, 1176882760);
    mgr.regist_struct_meta(StructMeta{name:String::from("hash_value::H256"), drop_fn: drop_1035403249}, 1035403249);
    mgr.regist_struct_meta(StructMeta{name:String::from("hash_value::H512"), drop_fn: drop_3223866506}, 3223866506);
    mgr.regist_struct_meta(StructMeta{name:String::from("hash_value::H520"), drop_fn: drop_2313040707}, 2313040707);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3776200155), 3776200155);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2655400174), 2655400174);
    mgr.regist_fun_meta(FnMeta::CallArg(call_444306144), 444306144);
    mgr.regist_fun_meta(FnMeta::CallArg(call_640799706), 640799706);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4080635904), 4080635904);
    mgr.regist_fun_meta(FnMeta::CallArg(call_702574989), 702574989);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3679689648), 3679689648);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1365606331), 1365606331);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3968952281), 3968952281);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3601455854), 3601455854);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3927864906), 3927864906);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1290828860), 1290828860);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3024861218), 3024861218);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3933772904), 3933772904);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1395856100), 1395856100);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4065712353), 4065712353);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4173242765), 4173242765);
    mgr.regist_fun_meta(FnMeta::CallArg(call_155840530), 155840530);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1103597055), 1103597055);
    mgr.regist_fun_meta(FnMeta::CallArg(call_234413511), 234413511);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3659472819), 3659472819);
    mgr.regist_fun_meta(FnMeta::CallArg(call_768602447), 768602447);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3857186173), 3857186173);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2370856657), 2370856657);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3293705496), 3293705496);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3987195607), 3987195607);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1178325458), 1178325458);
    mgr.regist_fun_meta(FnMeta::CallArg(call_57500088), 57500088);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1046314649), 1046314649);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1817272802), 1817272802);
}