use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use pi_math;



fn call_2798870758(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in take";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3974239134, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_math::hash::H32) };


    let result = pi_math::hash::H32::take(jst0);
    let result_jstype = js.new_uint8_array(result.len() as u32);result_jstype.from_bytes(&result);let mut result = result_jstype;

    Some(CallResult::Ok)
}


fn call_767388297(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tohex";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3974239134, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H32) };


    let result = pi_math::hash::H32::tohex(jst0);let mut result = js.new_str(result);
    

    Some(CallResult::Ok)
}


fn call_1420780752(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in from_buf";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = pi_math::hash::H32::from_buf(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3974239134);


    Some(CallResult::Ok)
}


fn call_4151244803(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in fromhex";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = &jst0.get_str();


    let result = pi_math::hash::H32::fromhex(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3974239134);


    Some(CallResult::Ok)
}


fn call_2263528600(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in cmp";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3974239134, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H32) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 3974239134, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_math::hash::H32) };


    let result = pi_math::hash::H32::cmp(jst0,jst1);let mut result = js.new_i8(result);

    Some(CallResult::Ok)
}


fn call_385903992(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in take";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 788004774, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_math::hash::H48) };


    let result = pi_math::hash::H48::take(jst0);
    let result_jstype = js.new_uint8_array(result.len() as u32);result_jstype.from_bytes(&result);let mut result = result_jstype;

    Some(CallResult::Ok)
}


fn call_1426274161(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tohex";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 788004774, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H48) };


    let result = pi_math::hash::H48::tohex(jst0);let mut result = js.new_str(result);
    

    Some(CallResult::Ok)
}


fn call_3783527665(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in from_buf";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = pi_math::hash::H48::from_buf(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,788004774);


    Some(CallResult::Ok)
}


fn call_4206140082(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in fromhex";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = &jst0.get_str();


    let result = pi_math::hash::H48::fromhex(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,788004774);


    Some(CallResult::Ok)
}


fn call_1194676335(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in cmp";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 788004774, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H48) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 788004774, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_math::hash::H48) };


    let result = pi_math::hash::H48::cmp(jst0,jst1);let mut result = js.new_i8(result);

    Some(CallResult::Ok)
}


fn call_3292766157(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in take";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3995272273, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_math::hash::H160) };


    let result = pi_math::hash::H160::take(jst0);
    let result_jstype = js.new_uint8_array(result.len() as u32);result_jstype.from_bytes(&result);let mut result = result_jstype;

    Some(CallResult::Ok)
}


fn call_1334624721(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tohex";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3995272273, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H160) };


    let result = pi_math::hash::H160::tohex(jst0);let mut result = js.new_str(result);
    

    Some(CallResult::Ok)
}


fn call_831073469(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in from_buf";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = pi_math::hash::H160::from_buf(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3995272273);


    Some(CallResult::Ok)
}


fn call_2102156475(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in fromhex";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = &jst0.get_str();


    let result = pi_math::hash::H160::fromhex(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3995272273);


    Some(CallResult::Ok)
}


fn call_1173820933(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in cmp";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3995272273, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H160) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 3995272273, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_math::hash::H160) };


    let result = pi_math::hash::H160::cmp(jst0,jst1);let mut result = js.new_i8(result);

    Some(CallResult::Ok)
}


fn call_2454669575(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in take";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 526967798, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_math::hash::H256) };


    let result = pi_math::hash::H256::take(jst0);
    let result_jstype = js.new_uint8_array(result.len() as u32);result_jstype.from_bytes(&result);let mut result = result_jstype;

    Some(CallResult::Ok)
}


fn call_3197660783(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tohex";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 526967798, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H256) };


    let result = pi_math::hash::H256::tohex(jst0);let mut result = js.new_str(result);
    

    Some(CallResult::Ok)
}


fn call_3458762269(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in from_buf";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = pi_math::hash::H256::from_buf(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,526967798);


    Some(CallResult::Ok)
}


fn call_3903164331(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in fromhex";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = &jst0.get_str();


    let result = pi_math::hash::H256::fromhex(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,526967798);


    Some(CallResult::Ok)
}


fn call_1683207497(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in cmp";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 526967798, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H256) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 526967798, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_math::hash::H256) };


    let result = pi_math::hash::H256::cmp(jst0,jst1);let mut result = js.new_i8(result);

    Some(CallResult::Ok)
}


fn call_3783275301(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in take";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2521161042, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_math::hash::H512) };


    let result = pi_math::hash::H512::take(jst0);
    let result_jstype = js.new_uint8_array(result.len() as u32);result_jstype.from_bytes(&result);let mut result = result_jstype;

    Some(CallResult::Ok)
}


fn call_3697048694(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tohex";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2521161042, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H512) };


    let result = pi_math::hash::H512::tohex(jst0);let mut result = js.new_str(result);
    

    Some(CallResult::Ok)
}


fn call_1727973064(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in from_buf";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = pi_math::hash::H512::from_buf(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2521161042);


    Some(CallResult::Ok)
}


fn call_4289491258(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in fromhex";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = &jst0.get_str();


    let result = pi_math::hash::H512::fromhex(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2521161042);


    Some(CallResult::Ok)
}


fn call_1422643842(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in cmp";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2521161042, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H512) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 2521161042, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_math::hash::H512) };


    let result = pi_math::hash::H512::cmp(jst0,jst1);let mut result = js.new_i8(result);

    Some(CallResult::Ok)
}


fn call_4027378382(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in take";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3787131431, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_math::hash::H520) };


    let result = pi_math::hash::H520::take(jst0);
    let result_jstype = js.new_uint8_array(result.len() as u32);result_jstype.from_bytes(&result);let mut result = result_jstype;

    Some(CallResult::Ok)
}


fn call_2639379268(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tohex";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3787131431, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H520) };


    let result = pi_math::hash::H520::tohex(jst0);let mut result = js.new_str(result);
    

    Some(CallResult::Ok)
}


fn call_4109608036(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in from_buf";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = pi_math::hash::H520::from_buf(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3787131431);


    Some(CallResult::Ok)
}


fn call_1287124001(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in fromhex";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = &jst0.get_str();


    let result = pi_math::hash::H520::fromhex(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,3787131431);


    Some(CallResult::Ok)
}


fn call_3340863876(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in cmp";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3787131431, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_math::hash::H520) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 3787131431, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_math::hash::H520) };


    let result = pi_math::hash::H520::cmp(jst0,jst1);let mut result = js.new_i8(result);

    Some(CallResult::Ok)
}

fn drop_3974239134(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_math::hash::H32) };
}

fn drop_788004774(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_math::hash::H48) };
}

fn drop_3995272273(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_math::hash::H160) };
}

fn drop_526967798(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_math::hash::H256) };
}

fn drop_2521161042(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_math::hash::H512) };
}

fn drop_3787131431(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_math::hash::H520) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_math::hash::H32"), drop_fn: drop_3974239134}, 3974239134);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_math::hash::H48"), drop_fn: drop_788004774}, 788004774);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_math::hash::H160"), drop_fn: drop_3995272273}, 3995272273);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_math::hash::H256"), drop_fn: drop_526967798}, 526967798);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_math::hash::H512"), drop_fn: drop_2521161042}, 2521161042);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_math::hash::H520"), drop_fn: drop_3787131431}, 3787131431);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2798870758), 2798870758);
    mgr.regist_fun_meta(FnMeta::CallArg(call_767388297), 767388297);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1420780752), 1420780752);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4151244803), 4151244803);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2263528600), 2263528600);
    mgr.regist_fun_meta(FnMeta::CallArg(call_385903992), 385903992);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1426274161), 1426274161);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3783527665), 3783527665);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4206140082), 4206140082);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1194676335), 1194676335);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3292766157), 3292766157);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1334624721), 1334624721);
    mgr.regist_fun_meta(FnMeta::CallArg(call_831073469), 831073469);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2102156475), 2102156475);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1173820933), 1173820933);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2454669575), 2454669575);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3197660783), 3197660783);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3458762269), 3458762269);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3903164331), 3903164331);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1683207497), 1683207497);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3783275301), 3783275301);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3697048694), 3697048694);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1727973064), 1727973064);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4289491258), 4289491258);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1422643842), 1422643842);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4027378382), 4027378382);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2639379268), 2639379268);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4109608036), 4109608036);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1287124001), 1287124001);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3340863876), 3340863876);
}