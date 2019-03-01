use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use std::mem::transmute;
use pi_db;



fn call_278583573(js: Arc<JS>) -> Option<CallResult>{

    let result:Vec<u8> = Vec::<u8>::new();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,104530634);


    Some(CallResult::Ok)
}


fn call_605387716(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in with_capacity";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result:Vec<u8> = Vec::<u8>::with_capacity(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,104530634);


    Some(CallResult::Ok)
}


fn call_3865263801(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in capacity";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Vec<u8>) };


    let result = Vec::<u8>::capacity(jst0);let mut result = js.new_u32(result as u32);

    Some(CallResult::Ok)
}


fn call_2115662480(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in as_slice";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Vec<u8>) };


    let result = Vec::<u8>::as_slice(jst0);
    let result_jstype = js.new_uint8_array(result.len() as u32);result_jstype.from_bytes(result);let mut result = result_jstype;

    Some(CallResult::Ok)
}


fn call_645064753(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in swap_remove";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u8>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::<u8>::swap_remove(jst0,jst1);let mut result = js.new_u8(result);

    Some(CallResult::Ok)
}


fn call_3352453288(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in insert";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u8>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


	let jst2 = &v[2];
	if !jst2.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_u8();


    Vec::<u8>::insert(jst0,jst1,jst2);
    Some(CallResult::Ok)
}


fn call_2151809700(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in remove";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u8>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::<u8>::remove(jst0,jst1);let mut result = js.new_u8(result);

    Some(CallResult::Ok)
}


fn call_107439253(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in push";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u8>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u8();


    Vec::<u8>::push(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_2913114375(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pop";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u8>) };


    let result = Vec::<u8>::pop(jst0);let mut result = match result{
        Some(v) => { let mut v = js.new_u8(v);
 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_4154086477(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in clear";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u8>) };


    Vec::<u8>::clear(jst0);
    Some(CallResult::Ok)
}


fn call_1534577376(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in len";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Vec<u8>) };


    let result = Vec::<u8>::len(jst0);let mut result = js.new_u32(result as u32);

    Some(CallResult::Ok)
}


fn call_3787109479(js: Arc<JS>) -> Option<CallResult>{

    let result:Vec<pi_db::db::TabKV> = Vec::<pi_db::db::TabKV>::new();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2202214327);


    Some(CallResult::Ok)
}


fn call_3760459365(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in with_capacity";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result:Vec<pi_db::db::TabKV> = Vec::<pi_db::db::TabKV>::with_capacity(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2202214327);


    Some(CallResult::Ok)
}


fn call_580562131(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in as_slice";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2202214327, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Vec<pi_db::db::TabKV>) };


    let result = Vec::<pi_db::db::TabKV>::as_slice(jst0);
	let mut result_array = js.new_array();
	for result_index in 0..result.len(){
		let mut result_elem = &result[result_index];
    let ptr = result_elem as *const pi_db::db::TabKV as usize;let mut result_elem = ptr_jstype(js.get_objs_ref(), js.clone(), ptr,4000136370);

js.set_index(&result_array, result_index as u32, &mut result_elem);
    }

    Some(CallResult::Ok)
}


fn call_3697063043(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in swap_remove";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2202214327, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<pi_db::db::TabKV>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::<pi_db::db::TabKV>::swap_remove(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,4000136370);


    Some(CallResult::Ok)
}


fn call_952027254(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in insert";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2202214327, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<pi_db::db::TabKV>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 4000136370, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut pi_db::db::TabKV) };


    Vec::<pi_db::db::TabKV>::insert(jst0,jst1,jst2);
    Some(CallResult::Ok)
}


fn call_482264970(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in remove";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2202214327, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<pi_db::db::TabKV>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::<pi_db::db::TabKV>::remove(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,4000136370);


    Some(CallResult::Ok)
}


fn call_393347340(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in push";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2202214327, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<pi_db::db::TabKV>) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 4000136370, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut pi_db::db::TabKV) };


    Vec::<pi_db::db::TabKV>::push(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_3897029640(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pop";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2202214327, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<pi_db::db::TabKV>) };


    let result = Vec::<pi_db::db::TabKV>::pop(jst0);let mut result = match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let mut v = ptr_jstype(js.get_objs(), js.clone(), ptr,4000136370);

 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_1982375693(js: Arc<JS>) -> Option<CallResult>{

    let result:Vec<i64> = Vec::<i64>::new();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2289656978);


    Some(CallResult::Ok)
}


fn call_3601066191(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in with_capacity";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result:Vec<i64> = Vec::<i64>::with_capacity(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2289656978);


    Some(CallResult::Ok)
}


fn call_1239372537(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in as_slice";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2289656978, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Vec<i64>) };


    let result = Vec::<i64>::as_slice(jst0);
	let mut result_array = js.new_array();
	for result_index in 0..result.len(){
		let mut result_elem = &result[result_index];let mut result_elem = js.new_i64(result_elem.clone());
js.set_index(&result_array, result_index as u32, &mut result_elem);
    }

    Some(CallResult::Ok)
}


fn call_859758326(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in swap_remove";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2289656978, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<i64>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::<i64>::swap_remove(jst0,jst1);let mut result = js.new_i64(result);

    Some(CallResult::Ok)
}


fn call_498200772(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in insert";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2289656978, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<i64>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


	let jst2 = &v[2];
	if !jst2.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_i64();


    Vec::<i64>::insert(jst0,jst1,jst2);
    Some(CallResult::Ok)
}


fn call_2071154981(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in remove";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2289656978, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<i64>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::<i64>::remove(jst0,jst1);let mut result = js.new_i64(result);

    Some(CallResult::Ok)
}


fn call_2957693395(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in push";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2289656978, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<i64>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_i64();


    Vec::<i64>::push(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_802425326(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pop";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2289656978, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<i64>) };


    let result = Vec::<i64>::pop(jst0);let mut result = match result{
        Some(v) => { let mut v = js.new_i64(v);
 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_2399706024(js: Arc<JS>) -> Option<CallResult>{

    let result:Vec<String> = Vec::<String>::new();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1542823015);


    Some(CallResult::Ok)
}


fn call_3498998071(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in with_capacity";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result:Vec<String> = Vec::<String>::with_capacity(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1542823015);


    Some(CallResult::Ok)
}


fn call_3093995464(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in as_slice";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1542823015, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Vec<String>) };


    let result = Vec::<String>::as_slice(jst0);
	let mut result_array = js.new_array();
	for result_index in 0..result.len(){
		let mut result_elem = &result[result_index];let mut result_elem = js.new_str(String::from(result_elem.as_str()));
js.set_index(&result_array, result_index as u32, &mut result_elem);
    }

    Some(CallResult::Ok)
}


fn call_3156648318(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in swap_remove";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1542823015, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<String>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::<String>::swap_remove(jst0,jst1);let mut result = js.new_str(result);

    Some(CallResult::Ok)
}


fn call_1978728938(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in insert";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1542823015, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<String>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


	let jst2 = &v[2];
	if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = jst2.get_str();


    Vec::<String>::insert(jst0,jst1,jst2);
    Some(CallResult::Ok)
}


fn call_1210159287(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in remove";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1542823015, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<String>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::<String>::remove(jst0,jst1);let mut result = js.new_str(result);

    Some(CallResult::Ok)
}


fn call_3803919743(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in push";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1542823015, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<String>) };


	let jst1 = &v[1];
	if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = jst1.get_str();


    Vec::<String>::push(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_3830052262(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pop";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1542823015, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<String>) };


    let result = Vec::<String>::pop(jst0);let mut result = match result{
        Some(v) => { let mut v = js.new_str(v);
 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_1828679694(js: Arc<JS>) -> Option<CallResult>{

    let result:Vec<Arc<Vec<u8>>> = Vec::<Arc<Vec<u8>>>::new();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2962204509);


    Some(CallResult::Ok)
}


fn call_2496158841(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in with_capacity";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result:Vec<Arc<Vec<u8>>> = Vec::<Arc<Vec<u8>>>::with_capacity(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2962204509);


    Some(CallResult::Ok)
}


fn call_2606142630(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in as_slice";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2962204509, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Vec<Arc<Vec<u8>>>) };


    let result = Vec::<Arc<Vec<u8>>>::as_slice(jst0);
	let mut result_array = js.new_array();
	for result_index in 0..result.len(){
		let mut result_elem = &result[result_index];
    let ptr = result_elem as *const Arc<Vec<u8>> as usize;let mut result_elem = ptr_jstype(js.get_objs_ref(), js.clone(), ptr,2886438122);

js.set_index(&result_array, result_index as u32, &mut result_elem);
    }

    Some(CallResult::Ok)
}


fn call_12783470(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in swap_remove";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2962204509, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<Arc<Vec<u8>>>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::<Arc<Vec<u8>>>::swap_remove(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2886438122);


    Some(CallResult::Ok)
}


fn call_1981878306(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in insert";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2962204509, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<Arc<Vec<u8>>>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 2886438122, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut Arc<Vec<u8>>)}.clone();


    Vec::<Arc<Vec<u8>>>::insert(jst0,jst1,jst2);
    Some(CallResult::Ok)
}


fn call_3566885191(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in remove";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2962204509, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<Arc<Vec<u8>>>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::<Arc<Vec<u8>>>::remove(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2886438122);


    Some(CallResult::Ok)
}


fn call_1441496172(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in push";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2962204509, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<Arc<Vec<u8>>>) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 2886438122, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut Arc<Vec<u8>>)}.clone();


    Vec::<Arc<Vec<u8>>>::push(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_2704292785(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pop";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2962204509, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<Arc<Vec<u8>>>) };


    let result = Vec::<Arc<Vec<u8>>>::pop(jst0);let mut result = match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let mut v = ptr_jstype(js.get_objs(), js.clone(), ptr,2886438122);

 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_1708919049(js: Arc<JS>) -> Option<CallResult>{

    let result:Vec<u32> = Vec::<u32>::new();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1662444400);


    Some(CallResult::Ok)
}


fn call_1994007224(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in with_capacity";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result:Vec<u32> = Vec::<u32>::with_capacity(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1662444400);


    Some(CallResult::Ok)
}


fn call_2842251538(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in as_slice";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1662444400, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Vec<u32>) };


    let result = Vec::<u32>::as_slice(jst0);
	let mut result_array = js.new_array();
	for result_index in 0..result.len(){
		let mut result_elem = &result[result_index];let mut result_elem = js.new_u32(result_elem.clone());
js.set_index(&result_array, result_index as u32, &mut result_elem);
    }

    Some(CallResult::Ok)
}


fn call_1587209337(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in swap_remove";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1662444400, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u32>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::<u32>::swap_remove(jst0,jst1);let mut result = js.new_u32(result);

    Some(CallResult::Ok)
}


fn call_3744706321(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in insert";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1662444400, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u32>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


	let jst2 = &v[2];
	if !jst2.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_u32();


    Vec::<u32>::insert(jst0,jst1,jst2);
    Some(CallResult::Ok)
}


fn call_2103362090(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in remove";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1662444400, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u32>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::<u32>::remove(jst0,jst1);let mut result = js.new_u32(result);

    Some(CallResult::Ok)
}


fn call_1420742667(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in push";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1662444400, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u32>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32();


    Vec::<u32>::push(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_3987113084(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in pop";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1662444400, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u32>) };


    let result = Vec::<u32>::pop(jst0);let mut result = match result{
        Some(v) => { let mut v = js.new_u32(v);
 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}

fn drop_104530634(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Vec<u8>) };
}

fn drop_2202214327(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Vec<pi_db::db::TabKV>) };
}

fn drop_4000136370(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_db::db::TabKV) };
}

fn drop_2289656978(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Vec<i64>) };
}

fn drop_1542823015(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Vec<String>) };
}

fn drop_2962204509(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Vec<Arc<Vec<u8>>>) };
}

fn drop_2886438122(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<Vec<u8>>) };
}

fn drop_1662444400(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Vec<u32>) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<u8>"), drop_fn: drop_104530634}, 104530634);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<pi_db::db::TabKV>"), drop_fn: drop_2202214327}, 2202214327);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::db::TabKV"), drop_fn: drop_4000136370}, 4000136370);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<i64>"), drop_fn: drop_2289656978}, 2289656978);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<String>"), drop_fn: drop_1542823015}, 1542823015);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<Arc<Vec<u8>>>"), drop_fn: drop_2962204509}, 2962204509);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<Vec<u8>>"), drop_fn: drop_2886438122}, 2886438122);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<u32>"), drop_fn: drop_1662444400}, 1662444400);
    mgr.regist_fun_meta(FnMeta::Call(call_278583573), 278583573);
    mgr.regist_fun_meta(FnMeta::CallArg(call_605387716), 605387716);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3865263801), 3865263801);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2115662480), 2115662480);
    mgr.regist_fun_meta(FnMeta::CallArg(call_645064753), 645064753);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3352453288), 3352453288);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2151809700), 2151809700);
    mgr.regist_fun_meta(FnMeta::CallArg(call_107439253), 107439253);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2913114375), 2913114375);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4154086477), 4154086477);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1534577376), 1534577376);
    mgr.regist_fun_meta(FnMeta::Call(call_3787109479), 3787109479);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3760459365), 3760459365);
    mgr.regist_fun_meta(FnMeta::CallArg(call_580562131), 580562131);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3697063043), 3697063043);
    mgr.regist_fun_meta(FnMeta::CallArg(call_952027254), 952027254);
    mgr.regist_fun_meta(FnMeta::CallArg(call_482264970), 482264970);
    mgr.regist_fun_meta(FnMeta::CallArg(call_393347340), 393347340);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3897029640), 3897029640);
    mgr.regist_fun_meta(FnMeta::Call(call_1982375693), 1982375693);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3601066191), 3601066191);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1239372537), 1239372537);
    mgr.regist_fun_meta(FnMeta::CallArg(call_859758326), 859758326);
    mgr.regist_fun_meta(FnMeta::CallArg(call_498200772), 498200772);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2071154981), 2071154981);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2957693395), 2957693395);
    mgr.regist_fun_meta(FnMeta::CallArg(call_802425326), 802425326);
    mgr.regist_fun_meta(FnMeta::Call(call_2399706024), 2399706024);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3498998071), 3498998071);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3093995464), 3093995464);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3156648318), 3156648318);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1978728938), 1978728938);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1210159287), 1210159287);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3803919743), 3803919743);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3830052262), 3830052262);
    mgr.regist_fun_meta(FnMeta::Call(call_1828679694), 1828679694);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2496158841), 2496158841);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2606142630), 2606142630);
    mgr.regist_fun_meta(FnMeta::CallArg(call_12783470), 12783470);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1981878306), 1981878306);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3566885191), 3566885191);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1441496172), 1441496172);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2704292785), 2704292785);
    mgr.regist_fun_meta(FnMeta::Call(call_1708919049), 1708919049);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1994007224), 1994007224);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2842251538), 2842251538);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1587209337), 1587209337);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3744706321), 3744706321);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2103362090), 2103362090);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1420742667), 1420742667);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3987113084), 3987113084);
}