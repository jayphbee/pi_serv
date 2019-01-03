use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use worker::task::TaskType;
use pi_vm::pi_vm_impl::{block_reply, block_throw};
use std::sync::Arc;
use atom::Atom;
use guid;
use atom;
use pi_db;



fn call_3651801454(js: Arc<JS>) -> Option<CallResult>{

    let result = pi_db::memery_db::DB::new();
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1237457629);


    Some(CallResult::Ok)
}


fn call_4081023775(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1736136244, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut guid::GuidGen) };


    let result = pi_db::mgr::Mgr::new(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,2976191628);


    Some(CallResult::Ok)
}


fn call_258785726(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in tab_info";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 913748025, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const atom::Atom) };


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 913748025, false, param_error).expect("");
	let jst2 = unsafe { &*(ptr as *const atom::Atom) };


    let result = pi_db::mgr::Mgr::tab_info(jst0,jst1,jst2);let mut result = match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let mut v = ptr_jstype(js.get_objs(), js.clone(), ptr,4164638564);

 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_951191934(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in transaction";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };


	let jst1 = &v[1];
	if !jst1.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst1 = jst1.get_boolean();
    

    let result = pi_db::mgr::Mgr::transaction(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let mut result = ptr_jstype(js.get_objs(), js.clone(), ptr,1754972364);


    Some(CallResult::Ok)
}


fn call_3803008464_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in prepare";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };

    let jscopy = js.clone();
	let call_back = move |r: Result<(),String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
	let array = js.new_array();    let mut r = array;

            } ), Atom::from(""));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    let r = pi_db::mgr::Tr::prepare(jst0,Arc::new(call_back));
	if r.is_some(){
        let r = r.unwrap();let mut r = match r{
        Ok(r) => { 
	let array = js.new_array();    let mut r = array;
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

        return Some(CallResult::Ok);
    }
	None
}


fn call_1346774966_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in commit";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };

    let jscopy = js.clone();
	let call_back = move |r: Result<(),String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
	let array = js.new_array();    let mut r = array;

            } ), Atom::from(""));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    let r = pi_db::mgr::Tr::commit(jst0,Arc::new(call_back));
	if r.is_some(){
        let r = r.unwrap();let mut r = match r{
        Ok(r) => { 
	let array = js.new_array();    let mut r = array;
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

        return Some(CallResult::Ok);
    }
	None
}


fn call_977907218_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in rollback";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };

    let jscopy = js.clone();
	let call_back = move |r: Result<(),String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
	let array = js.new_array();    let mut r = array;

            } ), Atom::from(""));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    let r = pi_db::mgr::Tr::rollback(jst0,Arc::new(call_back));
	if r.is_some(){
        let r = r.unwrap();let mut r = match r{
        Ok(r) => { 
	let array = js.new_array();    let mut r = array;
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

        return Some(CallResult::Ok);
    }
	None
}


fn call_1841891766_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in query";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 2202214327, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut Vec<pi_db::db::TabKV>) };


	let jst2 = &v[2];
    let jst2 = if jst2.is_undefined() || jst2.is_null(){
        None
    }else{
	if !jst2.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_u32() as usize;

        Some(jst2)
    };


	let jst3 = &v[3];
	if !jst3.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst3 = jst3.get_boolean();
    
    let jscopy = js.clone();
	let call_back = move |r: Result<Vec<pi_db::db::TabKV>,String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,2202214327);


            } ), Atom::from(""));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    let r = pi_db::mgr::Tr::query(jst0,jst1,jst2,jst3,Arc::new(call_back));
	if r.is_some(){
        let r = r.unwrap();let mut r = match r{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,2202214327);

 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

        return Some(CallResult::Ok);
    }
	None
}


fn call_685881041_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in modify";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 2202214327, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut Vec<pi_db::db::TabKV>) };


	let jst2 = &v[2];
    let jst2 = if jst2.is_undefined() || jst2.is_null(){
        None
    }else{
	if !jst2.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_u32() as usize;

        Some(jst2)
    };


	let jst3 = &v[3];
	if !jst3.is_boolean(){ return Some(CallResult::Err(String::from(param_error))); }
    let jst3 = jst3.get_boolean();
    
    let jscopy = js.clone();
	let call_back = move |r: Result<(),String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
	let array = js.new_array();    let mut r = array;

            } ), Atom::from(""));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    let r = pi_db::mgr::Tr::modify(jst0,jst1,jst2,jst3,Arc::new(call_back));
	if r.is_some(){
        let r = r.unwrap();let mut r = match r{
        Ok(r) => { 
	let array = js.new_array();    let mut r = array;
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

        return Some(CallResult::Ok);
    }
	None
}


fn call_3786000589_sync( js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{

	let param_error = "param error in alter";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 913748025, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const atom::Atom) };


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 913748025, false, param_error).expect("");
	let jst2 = unsafe { &*(ptr as *const atom::Atom) };


	let jst3 = &v[3];
    let jst3 = if jst3.is_undefined() || jst3.is_null(){
        None
    }else{
    let ptr = jstype_ptr(&jst3, js.clone(), 4164638564, true, param_error).expect("");
	let jst3 = *unsafe { Box::from_raw(ptr as *mut Arc<pi_db::db::TabMeta>)}.clone();

        Some(jst3)
    };

    let jscopy = js.clone();
	let call_back = move |r: Result<(),String>| {let mut r = match r{
        Ok(r) => {
            block_reply(jscopy.clone(), Box::new(move |js: Arc<JS>| {
	let array = js.new_array();    let mut r = array;

            } ), Atom::from(""));
        }
        Err(v) => { 
            block_throw(jscopy.clone(), v + ", Result is Err", Atom::from("block throw task"));
            return;
        }
    };

    };
    let r = pi_db::mgr::Tr::alter(jst0,jst1,jst2,jst3,Arc::new(call_back));
	if r.is_some(){
        let r = r.unwrap();let mut r = match r{
        Ok(r) => { 
	let array = js.new_array();    let mut r = array;
 r }
        Err(v) => { 
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

        return Some(CallResult::Ok);
    }
	None
}

fn drop_1237457629(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_db::memery_db::DB) };
}

fn drop_1736136244(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut guid::GuidGen) };
}

fn drop_2976191628(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_db::mgr::Mgr) };
}

fn drop_913748025(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut atom::Atom) };
}

fn drop_4164638564(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<pi_db::db::TabMeta>) };
}

fn drop_1754972364(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_db::mgr::Tr) };
}

fn drop_2202214327(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Vec<pi_db::db::TabKV>) };
}

fn drop_18210791(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_db::db::TabMeta) };
}

fn drop_4000136370(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut pi_db::db::TabKV) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::memery_db::DB"), drop_fn: drop_1237457629}, 1237457629);
    mgr.regist_struct_meta(StructMeta{name:String::from("guid::GuidGen"), drop_fn: drop_1736136244}, 1736136244);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::mgr::Mgr"), drop_fn: drop_2976191628}, 2976191628);
    mgr.regist_struct_meta(StructMeta{name:String::from("atom::Atom"), drop_fn: drop_913748025}, 913748025);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<pi_db::db::TabMeta>"), drop_fn: drop_4164638564}, 4164638564);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::mgr::Tr"), drop_fn: drop_1754972364}, 1754972364);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<pi_db::db::TabKV>"), drop_fn: drop_2202214327}, 2202214327);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::db::TabMeta"), drop_fn: drop_18210791}, 18210791);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_db::db::TabKV"), drop_fn: drop_4000136370}, 4000136370);
    mgr.regist_fun_meta(FnMeta::Call(call_3651801454), 3651801454);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4081023775), 4081023775);
    mgr.regist_fun_meta(FnMeta::CallArg(call_258785726), 258785726);
    mgr.regist_fun_meta(FnMeta::CallArg(call_951191934), 951191934);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3803008464_sync), 3803008464);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1346774966_sync), 1346774966);
    mgr.regist_fun_meta(FnMeta::CallArg(call_977907218_sync), 977907218);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1841891766_sync), 1841891766);
    mgr.regist_fun_meta(FnMeta::CallArg(call_685881041_sync), 685881041);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3786000589_sync), 3786000589);
}