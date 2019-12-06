use std::sync::{Arc, Mutex};
use std::cell::RefCell;

use atom::Atom;
use fnv::FnvHashMap as HashMap;
use pi_vm::adapter::JS;
use pi_vm::bonmgr::{NativeObjsAuth};

// 全局字节码缓冲
lazy_static! {
	pub static ref BYTE_CODE_CATCH: Arc<Mutex<RefCell<HashMap<String, Arc<Vec<u8>>>>>> = Arc::new(Mutex::new(RefCell::new(HashMap::default())));
}

/**
 * 取缓存字节码
 * @name： 字节码的key（通常是模块名称）
 */
pub fn get_byte_code(name: String) -> Option<Arc<Vec<u8>>> {
	let lock = BYTE_CODE_CATCH.lock().unwrap();
	let b = lock.borrow();
	match b.get(&name) {
		Some(r) => Some(unsafe { r.clone() }),
		None => None
	}
}

/**
 * 异步编译， 从源码编译为二进制码
 */
pub fn compile(name: String, source_code: String, call_back: Box<dyn FnOnce(Result<Arc<Vec<u8>>, String>)>) {
	let opts = JS::new(1, Atom::from("compile"), Arc::new(NativeObjsAuth::new(None, None)), None).unwrap();
	//为了保证模块的封装函数，可以是匿名的，且不绑定到全局环境中，需要用括号将封装函数括起来
	match opts.compile(name.clone(), source_code) {
		Some(r) => {
			let lock = BYTE_CODE_CATCH.lock().unwrap();
			let mut b = lock.borrow_mut();
			let byte_code = b.entry(name).or_insert(Arc::new(r));
			// 字节码被全局缓冲，
			call_back(Ok(byte_code.clone()));
		},
		None => call_back(Err(format!("compile err: {}", name))),
	};
}

/**
 * 从源码编译为二进制码
 */
pub fn compile_sync(name: String, source_code: String) -> Option<Arc< Vec<u8>>> {
	let opts = JS::new(1, Atom::from("compile"), Arc::new(NativeObjsAuth::new(None, None)), None).unwrap();
	//为了保证模块的封装函数，可以是匿名的，且不绑定到全局环境中，需要用括号将封装函数括起来
	match opts.compile(name.clone(), source_code) {
		Some(r) => {
			let lock = BYTE_CODE_CATCH.lock().unwrap();
			let mut b = lock.borrow_mut();
			let byte_code = b.entry(name).or_insert(Arc::new(r));
			// 字节码被全局缓冲， 
			Some(byte_code.clone())
		},
		None => None,
	}
}

/**
 * 执行二进制字节码， 返回函数
 */
pub fn load_module(byte_code: Arc<Vec<u8>>, js: &Arc<JS>) {
	if !js.load_module(byte_code.as_slice()) {
		//加载失败，则返回undefined
		js.new_undefined();
	}
}