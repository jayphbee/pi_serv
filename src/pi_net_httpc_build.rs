use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use atom::Atom;
use atom;
use std::io::Result;
use httpc::SharedHttpc;
use httpc;



fn call_3526501959(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in add_header";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1107924793, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Arc<httpc::HttpClient>) };


	let jst1 = &v[1];
    if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = Atom::from(jst1.get_str());


	let jst2 = &v[2];
    if !jst2.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst2 = Atom::from(jst2.get_str());


    let result = httpc::HttpClient::add_header(jst0,jst1,jst2);let mut result = js.new_u32(result as u32);

    Some(CallResult::Ok)
}


fn call_2025875773(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in remove_header";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1107924793, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Arc<httpc::HttpClient>) };


	let jst1 = &v[1];
    if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = Atom::from(jst1.get_str());


    let result = httpc::HttpClient::remove_header(jst0,jst1);let mut result = js.new_u32(result as u32);

    Some(CallResult::Ok)
}


fn call_2970107566(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in clear_headers";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1107924793, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut Arc<httpc::HttpClient>) };


    httpc::HttpClient::clear_headers(jst0);
    Some(CallResult::Ok)
}


fn call_29226352(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in headers_size";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1867977966, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const httpc::HttpClient) };


    let result = httpc::HttpClient::headers_size(jst0);let mut result = js.new_u32(result as u32);

    Some(CallResult::Ok)
}


fn call_3576683825(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in headers_keys";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1867977966, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const httpc::HttpClient) };


    let result = httpc::HttpClient::headers_keys(jst0);let mut result = match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let mut v = ptr_jstype(js.get_objs(), js.clone(), ptr,2108096905);

 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_2476662030(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in get_header";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1867977966, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const httpc::HttpClient) };


	let jst1 = &v[1];
    if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = Atom::from(jst1.get_str());


    let result = httpc::HttpClient::get_header(jst0,jst1);let mut result = match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let mut v = ptr_jstype(js.get_objs(), js.clone(), ptr,2108096905);

 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_3825034130(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in url";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 606449873, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const httpc::HttpClientResponse) };


    let result = httpc::HttpClientResponse::url(jst0);let mut result = js.new_str((*result).clone());


    Some(CallResult::Ok)
}


fn call_986662685(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in is_info";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 606449873, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const httpc::HttpClientResponse) };


    let result = httpc::HttpClientResponse::is_info(jst0);let mut result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_4079869020(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in is_ok";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 606449873, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const httpc::HttpClientResponse) };


    let result = httpc::HttpClientResponse::is_ok(jst0);let mut result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_2008399665(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in is_redirect";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 606449873, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut httpc::HttpClientResponse) };


    let result = httpc::HttpClientResponse::is_redirect(jst0);let mut result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_208103417(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in is_client_error";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 606449873, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut httpc::HttpClientResponse) };


    let result = httpc::HttpClientResponse::is_client_error(jst0);let mut result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_1117881293(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in is_server_error";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 606449873, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut httpc::HttpClientResponse) };


    let result = httpc::HttpClientResponse::is_server_error(jst0);let mut result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_45575971(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in is_undefined";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 606449873, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut httpc::HttpClientResponse) };


    let result = httpc::HttpClientResponse::is_undefined(jst0);let mut result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_3889629654(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in status";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 606449873, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const httpc::HttpClientResponse) };


    let result = httpc::HttpClientResponse::status(jst0);let mut result = js.new_u16(result);

    Some(CallResult::Ok)
}


fn call_484341674(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in status_info";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 606449873, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const httpc::HttpClientResponse) };


    let result = httpc::HttpClientResponse::status_info(jst0);let mut result = match result{
        Some(v) => { let mut v = js.new_str((*v).clone());

 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_677141052(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in headers_size";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 606449873, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const httpc::HttpClientResponse) };


    let result = httpc::HttpClientResponse::headers_size(jst0);let mut result = js.new_u32(result as u32);

    Some(CallResult::Ok)
}


fn call_1258454971(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in headers_keys";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 606449873, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const httpc::HttpClientResponse) };


    let result = httpc::HttpClientResponse::headers_keys(jst0);let mut result = match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let mut v = ptr_jstype(js.get_objs(), js.clone(), ptr,2108096905);

 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_2914686338(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in get_header";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 606449873, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const httpc::HttpClientResponse) };


	let jst1 = &v[1];
    if !jst1.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
    let jst1 = Atom::from(jst1.get_str());


    let result = httpc::HttpClientResponse::get_header(jst0,jst1);let mut result = match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let mut v = ptr_jstype(js.get_objs(), js.clone(), ptr,2108096905);

 v}
        None => js.new_null()
    };

    Some(CallResult::Ok)
}


fn call_2925270627(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in text";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 606449873, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut httpc::HttpClientResponse) };


    let result = httpc::HttpClientResponse::text(jst0);let mut result = match result{
        Ok(r) => { let mut r = js.new_str(r);
 r }
        Err(v) => { 
            return Some(CallResult::Err(v.to_string() + "Result is Err"));
        }
    };

    Some(CallResult::Ok)
}


fn call_2657372573(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bin";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 606449873, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut httpc::HttpClientResponse) };


    let result = httpc::HttpClientResponse::bin(jst0);let mut result = match result{
        Ok(r) => { 
    let ptr = Box::into_raw(Box::new(r)) as usize;let mut r = ptr_jstype(js.get_objs(), js.clone(), ptr,104530634);

 r }
        Err(v) => { 
            return Some(CallResult::Err(v.to_string() + "Result is Err"));
        }
    };

    Some(CallResult::Ok)
}

fn drop_1107924793(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Arc<httpc::HttpClient>) };
}

fn drop_1867977966(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut httpc::HttpClient) };
}

fn drop_2108096905(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Vec<atom::Atom>) };
}

fn drop_606449873(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut httpc::HttpClientResponse) };
}

fn drop_104530634(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Vec<u8>) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<httpc::HttpClient>"), drop_fn: drop_1107924793}, 1107924793);
    mgr.regist_struct_meta(StructMeta{name:String::from("httpc::HttpClient"), drop_fn: drop_1867977966}, 1867977966);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<atom::Atom>"), drop_fn: drop_2108096905}, 2108096905);
    mgr.regist_struct_meta(StructMeta{name:String::from("httpc::HttpClientResponse"), drop_fn: drop_606449873}, 606449873);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<u8>"), drop_fn: drop_104530634}, 104530634);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3526501959), 3526501959);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2025875773), 2025875773);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2970107566), 2970107566);
    mgr.regist_fun_meta(FnMeta::CallArg(call_29226352), 29226352);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3576683825), 3576683825);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2476662030), 2476662030);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3825034130), 3825034130);
    mgr.regist_fun_meta(FnMeta::CallArg(call_986662685), 986662685);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4079869020), 4079869020);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2008399665), 2008399665);
    mgr.regist_fun_meta(FnMeta::CallArg(call_208103417), 208103417);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1117881293), 1117881293);
    mgr.regist_fun_meta(FnMeta::CallArg(call_45575971), 45575971);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3889629654), 3889629654);
    mgr.regist_fun_meta(FnMeta::CallArg(call_484341674), 484341674);
    mgr.regist_fun_meta(FnMeta::CallArg(call_677141052), 677141052);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1258454971), 1258454971);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2914686338), 2914686338);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2925270627), 2925270627);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2657372573), 2657372573);
}