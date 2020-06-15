use std::sync::{Arc, RwLock};

use atom::Atom;
use gray::{GrayTab, GrayVersion};
use handler::{Args, GenType, Handler};
use crate::js_lib::JSGray;
use pi_vm::adapter::{JSType, JS};
use pi_vm::bonmgr::ptr_jstype;
use pi_vm::bonmgr::{jstype_ptr, BonMgr, CallResult, FnMeta};
use pi_vm::channel_map::VMChannel;
use pi_vm::pi_vm_impl::async_request;
use pi_vm::proc_pool::{close_process, name_send, name_to_pid, pid_send, set_catcher, set_factory, set_receiver,spawn_process};

fn async_request_hash(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in async_request";
    let jst0 = &v[0];
    if !jst0.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.get_str();

    let jst1 = &v[1];
    if !jst1.is_uint8_array() && !jst1.is_array_buffer() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.into_vec();

    let jst2 = &v[2];
    if !jst2.is_array() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let len = jst2.get_array_length();
    let mut arr = Vec::with_capacity(len);
    for i in 0..len {
        arr.push(jst2.get_index(i as u32).get_native_object());
    }
    let jst3 = &v[3];
    if jst3.is_undefined() || jst3.is_null() {
        js.new_boolean(async_request(
            js.clone(),
            Atom::from(jst0),
            Arc::new(jst1),
            arr,
            None,
        ));
        return None;
    } else if jst3.is_number() {
        js.new_boolean(async_request(
            js.clone(),
            Atom::from(jst0),
            Arc::new(jst1),
            arr,
            Some(jst3.get_u32()),
        ));
        return Some(CallResult::Ok);
    } else {
        return Some(CallResult::Err(String::from(param_error)));
    }
}

fn async_response_hash(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in async_response_hash";
    //VMChannel
    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3366364668, true, param_error).expect("");
    let jst0 = *unsafe { Box::from_raw(ptr as *mut Arc<VMChannel>) };
    //args
    let jst1 = &v[1];
    if !jst1.is_uint8_array() && !jst1.is_array_buffer() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.into_vec();
    //&[nativObject]
    let jst2 = &v[2];
    if !jst2.is_array() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let len = jst2.get_array_length();
    let mut arr = Vec::with_capacity(len);
    for i in 0..len {
        arr.push(jst2.get_index(i as u32).get_native_object());
    }

    //callbackIndex
    let jst3 = &v[3];
    if jst3.is_undefined() || jst3.is_null() {
        js.new_boolean(jst0.response(None, Arc::new(jst1), arr));
    } else if jst3.is_number() {
        js.new_boolean(jst0.response(Some(jst3.get_u32()), Arc::new(jst1), arr));
    } else {
        return Some(CallResult::Err(String::from(param_error)));
    }
    Some(CallResult::Ok)
}

/**
* 异步请求处理器
*/
pub struct AsyncRequestHandler {
    gray_tab: Arc<RwLock<GrayTab<JSGray>>>, //灰度表
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

    fn handle(
        &self,
        env: Arc<dyn GrayVersion>,
        name: Atom,
        args: Args<Self::A, Self::B, Self::C, Self::D, Self::E, Self::F, Self::G, Self::H>,
    ) -> Self::HandleResult {
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
                }
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
                }
                _ => panic!("invalid async call handler args"),
            }
            let ptr = Box::into_raw(Box::new(mgr.clone())) as usize;
            ptr_jstype(vm.get_objs(), vm.clone(), ptr, 2976191628);
            let ptr = Box::into_raw(Box::new(env.clone())) as usize;
            ptr_jstype(vm.get_objs(), vm.clone(), ptr, 3366364668);
            6
        });
        gray.factory.call(
            None,
            Atom::from("_$async"),
            real_args,
            Atom::from((*name).to_string() + " async task"),
        );
    }
}

impl AsyncRequestHandler {
    /**
    	* 构建异步请求处理器
    	* @param gray 灰度对象
    	* @returns 返回异步请求处理器
    	*/
    pub fn new(gray: JSGray) -> Self {
        AsyncRequestHandler {
            gray_tab: Arc::new(RwLock::new(GrayTab::new(gray))),
        }
    }
}

fn pid_set_receiver_hash(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in pid_set_receiver_hash";
    // pid
    let jst0 = &v[0];
    if !jst0.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.get_u32();

    // callback
    let jst1 = &v[1];
    if !jst1.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_u32();
    //注册receiver
    if let Err(e) = set_receiver(jst0 as u64, GenType::U32(jst1)) {
        return Some(CallResult::Err(e.to_string()));
    }
    js.new_undefined();
    Some(CallResult::Ok)
}

fn pid_send_hash(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in pid_send_hash";

    // 源pid
    let src = &v[0];
    if !src.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let src = src.get_u32();

    // 目标pid或名称
    let dst = &v[1];
    let mut number_pid = None;
    let mut str_pid = None;
    if dst.is_number() {
        number_pid = Some(dst.get_u32());
    } else if dst.is_string() {
        str_pid = Some(dst.get_str());
    } else {
        return Some(CallResult::Err(String::from(param_error)));
    }

    let args = match get_args(&v, &js, 2, v.len()) {
        Some(r) => r,
        None => return Some(CallResult::Err(String::from(param_error))),
    };

    let r = if let Some(pid) = number_pid {
        pid_send(src as u64, pid as u64, args)
    } else if let Some(pid) = str_pid {
        name_send(src as u64, pid, args)
    } else {
        return Some(CallResult::Err(param_error.to_string()));
    };
    match r {
        Ok(_) => {
            js.new_undefined();
            Some(CallResult::Ok)
        }
        Err(e) => return Some(CallResult::Err(e.to_string())),
    }
}

fn pid_spawn_hash(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in pid_spawn_hash";
    if !v[0].is_string()
        || !v[1].is_string()
        || !v[2].is_string()
        || !v[3].is_string()
    {
        return Some(CallResult::Err(param_error.to_string()));
    }
    let factory_name = v[0].get_str();
    let process_name = v[1].get_str();
    let module = v[2].get_str();
    let function = v[3].get_str();

    let args = match get_args(&v, &js, 2, v.len()) {
        Some(r) => r,
        None => return Some(CallResult::Err(String::from(param_error))),
    };

    match spawn_process(
        Some(process_name),
        Atom::from(factory_name),
        module,
        function,
        "onspawn".to_string(),
        args,
    ) {
        Err(e) => Some(CallResult::Err(e.to_string())),
        Ok(pid) => {
            js.new_u32(pid as u32);
            Some(CallResult::Ok)
        }
    }
}

fn pid_close_hash(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in pid_close_hash";
    let pid = &v[0];
    if !pid.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let pid = pid.get_u32();

    if let Err(e) = close_process(pid as u64, "normal".to_string()) {
        return Some(CallResult::Err(e.to_string()));
    }

    js.new_undefined();
    Some(CallResult::Ok)
}

fn pid_set_catcher_hash(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in pid_set_catcher_hash";
    // pid
    let jst0 = &v[0];
    if !jst0.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.get_u32();

    // callback
    let jst1 = &v[1];
    if !jst1.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_u32();

    //注册receiver
    if let Err(e) = set_catcher(jst0 as u64, GenType::U32(jst1)) {
        return Some(CallResult::Err(e.to_string()));
    }
    js.new_undefined();
    Some(CallResult::Ok)
}

fn get_args(v: &Vec<JSType>, js: &Arc<JS>, start: usize, end: usize) -> Option<GenType> {
    let mut args = Vec::new();
    // 数据
    for i in start..end {
        let arg = &v[i];
        if arg.is_uint8_array() || arg.is_array_buffer() {
            args.push(GenType::Bin(arg.into_vec()));
        } else if arg.is_number() {
            args.push(GenType::F64(arg.get_f64()));
        } else if arg.is_string() {
            args.push(GenType::Str(arg.get_str()))
        } else if arg.is_boolean() {
            args.push(GenType::Bool(arg.get_boolean()))
        } else if arg.is_array() && arg.get_array_length() == 2 {
            let objs = js.get_objs();
            let mut objs = objs.borrow_mut();
            let ptr = arg.get_index(0 as u32).get_native_object();
            let meta_hash = match objs.remove(&(ptr as usize)) {
                Some(v) => v.meta_hash,
                None => return None,
            };
            args.push(GenType::Array(vec![
                GenType::USize(ptr),
                GenType::USize(arg.get_index(1 as u32).get_u32() as usize),
                GenType::USize(meta_hash as usize),
			]));
        } else {
            return None;
        }
    }
    Some(GenType::Array(args))
}

pub fn register(mgr: &BonMgr) {
    mgr.regist_fun_meta(FnMeta::CallArg(async_request_hash), 1);
    mgr.regist_fun_meta(FnMeta::CallArg(async_response_hash), 2);
    mgr.regist_fun_meta(FnMeta::CallArg(pid_set_receiver_hash), 3);
    mgr.regist_fun_meta(FnMeta::CallArg(pid_send_hash), 4);
    mgr.regist_fun_meta(FnMeta::CallArg(pid_spawn_hash), 5);
    mgr.regist_fun_meta(FnMeta::CallArg(pid_set_catcher_hash), 6);
    mgr.regist_fun_meta(FnMeta::CallArg(pid_close_hash), 7);
}
