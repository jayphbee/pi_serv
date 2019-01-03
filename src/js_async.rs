use std::sync::{Arc, RwLock};

use atom::Atom;
use handler::{Handler, Args};
use gray::{ GrayVersion, GrayTab};
use pi_vm::bonmgr::{BonMgr, FnMeta, jstype_ptr, CallResult};
use pi_vm::channel_map::VMChannel;
use pi_vm::pi_vm_impl::async_request;
use pi_vm::adapter::{JS, JSType};
use pi_vm::bonmgr::{ptr_jstype};
use js_lib::JSGray;

fn async_request_hash(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in async_request";
	let jst0 = &v[0];
	if !jst0.is_string(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.get_str();

    let jst1 = &v[1];
	if !jst1.is_uint8_array() && !jst1.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst1 = jst1.into_vec();

	let jst2 = &v[2];
	if !jst2.is_array(){return Some(CallResult::Err(String::from(param_error)));}
    let len = jst2.get_array_length();
    let mut arr = Vec::with_capacity(len);
    for i in 0..len{
        arr.push(jst2.get_index(i as u32).get_native_object());
    }
    let jst3 = &v[3];
    if jst3.is_undefined() || jst3.is_null(){
        js.new_boolean(async_request(js.clone(), Atom::from(jst0), Arc::new(jst1), arr, None));
        return None;
    }else if jst3.is_number(){
        js.new_boolean(async_request(js.clone(), Atom::from(jst0), Arc::new(jst1), arr, Some(jst3.get_u32())));
        return Some(CallResult::Ok);
    }else{
        return Some(CallResult::Err(String::from(param_error)));
    }
}

fn async_response_hash(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in async_response_hash";
    //VMChannel
    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3366364668, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut Arc<VMChannel>) };
    //args
    let jst1 = &v[1];
	if !jst1.is_uint8_array() && !jst1.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst1 = jst1.into_vec();
    //&[nativObject]
	let jst2 = &v[2];
	if !jst2.is_array(){return Some(CallResult::Err(String::from(param_error)));}
    let len = jst2.get_array_length();
    let mut arr = Vec::with_capacity(len);
    for i in 0..len{
        arr.push(jst2.get_index(i as u32).get_native_object());
    }

    //callbackIndex
    let jst3 = &v[3];
    if jst3.is_undefined() || jst3.is_null(){
        js.new_boolean(jst0.response(None, Arc::new(jst1), arr));
    }else if jst3.is_number(){
        js.new_boolean(jst0.response(Some(jst3.get_u32()), Arc::new(jst1), arr));
    }else{
        return Some(CallResult::Err(String::from(param_error)));
    }
    Some(CallResult::Ok)
}

pub fn register(mgr: &BonMgr){
    mgr.regist_fun_meta(FnMeta::CallArg(async_request_hash), 1);
    mgr.regist_fun_meta(FnMeta::CallArg(async_response_hash), 2);
}


/*
* 异步请求处理器
*/
pub struct AsyncRequestHandler {
	gray_tab: 	Arc<RwLock<GrayTab<JSGray>>>,	//灰度表
}

unsafe impl Send for AsyncRequestHandler {}
unsafe impl Sync for AsyncRequestHandler {}

impl Handler for AsyncRequestHandler {
	type A = Arc<Vec<u8>>;
	type B = Vec<JSType>;
	type C = Option<u32>;
	type D = ();
	type E = ();
	type F = ();
	type G = ();
	type H = ();
	type HandleResult = ();

	fn handle(&self, env: Arc<dyn GrayVersion>, name: Atom, args: Args<Self::A, Self::B, Self::C, Self::D, Self::E, Self::F, Self::G, Self::H>) -> Self::HandleResult {
		let gray_tab = self.gray_tab.read().unwrap();
        let gray = match env.get_gray() {
            Some(v) => match gray_tab.get(v) {
                Some(g) => g,
                None => panic!("gray is not exist, version:{}", v),
            },
            None => gray_tab.get_last(),
        };
        let mgr = gray.mgr.clone();
        let copy_name = name.clone();
		let real_args = Box::new(move |vm: Arc<JS>| -> usize {
			vm.new_str((*copy_name).to_string());
			match args {
				Args::ThreeArgs(bin, objs, None) => {
					//处理异步阻塞调用
					let buffer = vm.new_uint8_array(bin.len() as u32);
					buffer.from_bytes(bin.as_slice());
					let mut value: JSType;
					let array = vm.new_array();
					for i in 0..objs.len() {
						value = vm.new_native_object(objs[i].get_native_object());
						vm.set_index(&array, i as u32, &mut value);
					}
					vm.new_null();
				},
				Args::ThreeArgs(bin, objs, Some(index)) => {
					//处理异步调用
					let buffer = vm.new_uint8_array(bin.len() as u32);
					buffer.from_bytes(bin.as_slice());
					let mut value: JSType;
					let array = vm.new_array();
					for i in 0..objs.len() {
						value = vm.new_native_object(objs[i].get_native_object());
						vm.set_index(&array, i as u32, &mut value);
					}
					vm.new_u32(index);
				},
				_ => panic!("invalid async call handler args"),
			}
			let ptr = Box::into_raw(Box::new(mgr.clone())) as usize;
			ptr_jstype(vm.get_objs(), vm.clone(), ptr, 2976191628);
			let ptr = Box::into_raw(Box::new(env.clone())) as usize;
			ptr_jstype(vm.get_objs(), vm.clone(), ptr, 3366364668);
			6
		});
		gray.factory.call(None, Atom::from("_$async"), real_args, Atom::from((*name).to_string() + " rpc task"));
	}
}

impl AsyncRequestHandler {
	//构建一个处理器
	pub fn new(gray: JSGray) -> Self {
		AsyncRequestHandler {
			gray_tab: Arc::new(RwLock::new(GrayTab::new(gray))),
		}
	}
}
