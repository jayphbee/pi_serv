use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use pi_vm::pi_vm_impl::{ block_reply, block_throw};
use pi_vm::task::TaskType;
use pi_db;



fn call_278583573(js: Arc<JS>, mgr: &BonMgr) -> Option<CallResult>{

    let result:Vec<u8> = Vec::new();
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,104530634);


    Some(CallResult::Ok)
}


fn call_605387716(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result:Vec<u8> = Vec::with_capacity(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,104530634);


    Some(CallResult::Ok)
}


fn call_3865263801(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 104530634, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Vec<u8>) };


    let result = Vec::capacity(jst0);let result = js.new_u32(result as u32);

    Some(CallResult::Ok)
}


fn call_1832001267(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 104530634, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Vec<u8>) };


    let result = Vec::as_slice(jst0);
    let result_jstype = js.new_uint8_array(result.len() as u32);result_jstype.from_bytes(result);let result = result_jstype;

    Some(CallResult::Ok)
}


fn call_645064753(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 104530634, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u8>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::swap_remove(jst0,jst1);let result = js.new_u8(result);

    Some(CallResult::Ok)
}


fn call_3352453288(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 104530634, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u8>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


	let jst2 = &v[2];
	if !jst2.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_u8();


    Vec::insert(jst0,jst1,jst2);
    Some(CallResult::Ok)
}


fn call_2151809700(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 104530634, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u8>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::remove(jst0,jst1);let result = js.new_u8(result);

    Some(CallResult::Ok)
}


fn call_107439253(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 104530634, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u8>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u8();


    Vec::push(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_2913114375(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 104530634, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u8>) };


    let result = Vec::pop(jst0);
    match result{
        Some(v) => { let v = js.new_u8(v);
 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_4154086477(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 104530634, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<u8>) };


    Vec::clear(jst0);
    Some(CallResult::Ok)
}


fn call_1534577376(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 104530634, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const Vec<u8>) };


    let result = Vec::len(jst0);let result = js.new_u32(result as u32);

    Some(CallResult::Ok)
}


fn call_3787109479(js: Arc<JS>, mgr: &BonMgr) -> Option<CallResult>{

    let result:Vec<pi_db::db::TabKV> = Vec::new();
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,2202214327);


    Some(CallResult::Ok)
}


fn call_3760459365(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result:Vec<pi_db::db::TabKV> = Vec::with_capacity(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,2202214327);


    Some(CallResult::Ok)
}


fn call_3697063043(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 2202214327, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<pi_db::db::TabKV>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::swap_remove(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,4000136370);


    Some(CallResult::Ok)
}


fn call_952027254(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 2202214327, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<pi_db::db::TabKV>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, mgr, 4000136370, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut pi_db::db::TabKV) };


    Vec::insert(jst0,jst1,jst2);
    Some(CallResult::Ok)
}


fn call_482264970(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 2202214327, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<pi_db::db::TabKV>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::remove(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,4000136370);


    Some(CallResult::Ok)
}


fn call_393347340(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 2202214327, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<pi_db::db::TabKV>) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, mgr, 4000136370, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut pi_db::db::TabKV) };


    Vec::push(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_3897029640(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 2202214327, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<pi_db::db::TabKV>) };


    let result = Vec::pop(jst0);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,4000136370);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_1982375693(js: Arc<JS>, mgr: &BonMgr) -> Option<CallResult>{

    let result:Vec<i64> = Vec::new();
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,2289656978);


    Some(CallResult::Ok)
}


fn call_3601066191(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result:Vec<i64> = Vec::with_capacity(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(mgr.objs.clone(), js.clone(), ptr,2289656978);


    Some(CallResult::Ok)
}


fn call_859758326(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 2289656978, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<i64>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::swap_remove(jst0,jst1);let result = js.new_i64(result);

    Some(CallResult::Ok)
}


fn call_498200772(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 2289656978, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<i64>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


	let jst2 = &v[2];
	if !jst2.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_i64();


    Vec::insert(jst0,jst1,jst2);
    Some(CallResult::Ok)
}


fn call_2071154981(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 2289656978, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<i64>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


    let result = Vec::remove(jst0,jst1);let result = js.new_i64(result);

    Some(CallResult::Ok)
}


fn call_2957693395(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 2289656978, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<i64>) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_i64();


    Vec::push(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_802425326(js: Arc<JS>, mgr: &BonMgr, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in Vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, mgr, 2289656978, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Vec<i64>) };


    let result = Vec::pop(jst0);
    match result{
        Some(v) => { let v = js.new_i64(v);
 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<u8>")}, 104530634);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<pi_db::db::TabKV>")}, 2202214327);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::db::TabKV")}, 4000136370);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<i64>")}, 2289656978);
    mgr.regist_fun_meta(FnMeta::CallNobj(call_278583573), 278583573);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_605387716), 605387716);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3865263801), 3865263801);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_1832001267), 1832001267);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_645064753), 645064753);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3352453288), 3352453288);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_2151809700), 2151809700);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_107439253), 107439253);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_2913114375), 2913114375);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_4154086477), 4154086477);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_1534577376), 1534577376);
    mgr.regist_fun_meta(FnMeta::CallNobj(call_3787109479), 3787109479);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3760459365), 3760459365);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3697063043), 3697063043);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_952027254), 952027254);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_482264970), 482264970);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_393347340), 393347340);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3897029640), 3897029640);
    mgr.regist_fun_meta(FnMeta::CallNobj(call_1982375693), 1982375693);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_3601066191), 3601066191);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_859758326), 859758326);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_498200772), 498200772);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_2071154981), 2071154981);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_2957693395), 2957693395);
    mgr.regist_fun_meta(FnMeta::CallArgNobj(call_802425326), 802425326);
}