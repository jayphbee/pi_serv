use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicUsize;

use fnv::FnvHashMap;

use pi_vm::adapter::{JS, JSType};
use pi_vm::pi_vm_impl::VMFactory;
use pi_vm::bonmgr::{ptr_jstype, BON_MGR};
use pi_db::mgr::Mgr;
use pi_lib::atom::Atom;
use pi_lib::handler::{Env, GenType, Handler, Args};

/*
* 灰度表
*/
struct GrayTab {
	tab: Arc<RwLock<FnvHashMap<usize, (Arc<VMFactory>, Mgr)>>>,
}

impl GrayTab {
	//构建一个灰度表
	pub fn new() -> Self {
		GrayTab {
			tab: Arc::new(RwLock::new(FnvHashMap::default())),
		}
	}

	//获取指定灰度的虚拟机工厂和事务管理器
	pub fn get(&self, gray: usize) -> Option<(Arc<VMFactory>, Mgr)> {
		match self.tab.read().unwrap().get(&gray) {
			None => None,
			Some((factory, mgr)) => Some((factory.clone(), mgr.clone())),
		}
	}

	//设置指定灰度的虚拟机工厂和事务管理器
	pub fn set(&self, gray: usize, factory: VMFactory, mgr: Mgr) {
		self.tab.write().unwrap().insert(gray, (Arc::new(factory), mgr));
	}

	//移除指定灰度的虚拟机工厂和事务管理器
	pub fn remove(&self, gray: usize) -> Option<(Arc<VMFactory>, Mgr)> {
		self.tab.write().unwrap().remove(&gray)
	}
}

/*
* Topic处理器
*/
pub struct TopicHandler {
	len: 		AtomicUsize,	//处理器消息队列最大长度
	factory: 	Arc<VMFactory>,	//默认虚拟机工厂
	mgr: 		Mgr,			//默认事务管理器
	gray_tab: 	GrayTab,		//灰度表
}

unsafe impl Send for TopicHandler {}
unsafe impl Sync for TopicHandler {}

impl Handler for TopicHandler {
	type A = u8;
    type B = Arc<Vec<u8>>;
    type C = ();
    type D = ();
    type E = ();
    type F = ();
    type G = ();
    type H = ();
	type HandleResult = ();

	fn handle(&self, env: Arc<dyn Env>, topic: Atom, args: Args<Self::A, Self::B, Self::C, Self::D, Self::E, Self::F, Self::G, Self::H>) -> Self::HandleResult {
		let (factory, mgr) = self.get(env.clone());
        let topic_name = topic.clone();
		let real_args = Box::new(move |vm: Arc<JS>| {
			vm.new_str((*topic_name).to_string());
			match args {
				Args::TwoArgs(_, bin) => {
					let buffer = vm.new_uint8_array(bin.len() as u32);
					buffer.from_bytes(bin.as_slice());
				},
				_ => panic!("invalid topic handler args"),
			}
			let ptr = Box::into_raw(Box::new(mgr.clone())) as usize;
			ptr_jstype(vm.get_objs(), vm.clone(), ptr, 2976191628);
			let ptr = Box::into_raw(Box::new(env.clone())) as usize;
			ptr_jstype(vm.get_objs(), vm.clone(), ptr, 2256377725);
		});
		factory.call(0, Atom::from("_$rpc"), real_args, Atom::from((*topic).to_string() + " rpc task"));
	}
}

impl TopicHandler {
	//构建一个处理器
	pub fn new(len: usize, factory: VMFactory, mgr: Mgr) -> Self {
		TopicHandler {
			len: AtomicUsize::new(len),
			factory: Arc::new(factory),
			mgr: mgr,
			gray_tab: GrayTab::new(),
		}
	}

	//获取默认虚拟机工厂和事务管理器
	pub fn get_default(&self) -> (Arc<VMFactory>, Mgr) {
		(self.factory.clone(), self.mgr.clone())
	}

	//设置指定灰度为默认版本
	pub fn set_default(&mut self, gray: usize) {
		match self.gray_tab.remove(gray) {
			None => return,
			Some((f, m)) => {
				self.factory = f;
				self.mgr = m;
			},
		}
	}

	//获取指定的虚拟机工厂和事务管理器
	fn get(&self, session: Arc<dyn Env>) -> (Arc<VMFactory>, Mgr) {
		match session.get_attr(Atom::from("_$gray")) {
			Some(val) => {
				match val {
					GenType::Bin(bin) => {
						let gray = usize::from_le(unsafe { *(bin[..].as_ptr() as *mut usize) });
						match self.gray_tab.get(gray) {
							None => self.get_default(),
							Some(r) => r,
						}
					},
					_ => self.get_default(),
				}
			},
			_ => self.get_default(),
		}
	}
}

/*
* 异步请求处理器
*/
pub struct AsyncRequestHandler {
	factory: 	Arc<VMFactory>,	//默认虚拟机工厂
	mgr: 		Mgr,			//默认事务管理器
	gray_tab: 	GrayTab,		//灰度表
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

	fn handle(&self, env: Arc<dyn Env>, name: Atom, args: Args<Self::A, Self::B, Self::C, Self::D, Self::E, Self::F, Self::G, Self::H>) -> Self::HandleResult {
		let (factory, mgr) = self.get(env.clone());
        let copy_name = name.clone();
		let real_args = Box::new(move |vm: Arc<JS>| {
			vm.new_str((*copy_name).to_string());
			match args {
				Args::ThreeArgs(bin, objs, Some(index)) => {
					let buffer = vm.new_uint8_array(bin.len() as u32);
					buffer.from_bytes(bin.as_slice());
					let mut value: JSType;
					let array = vm.new_array();
					for i in 0..objs.len() {
						value = vm.new_native_object(objs[i].get_native_object());
						vm.set_index(&array, i as u32, &value);
					}
					vm.new_u32(index);
				},
				_ => panic!("invalid async call handler args"),
			}
			let ptr = Box::into_raw(Box::new(mgr.clone())) as usize;
			ptr_jstype(vm.get_objs(), vm.clone(), ptr, 2976191628);
			let ptr = Box::into_raw(Box::new(env.clone())) as usize;
			ptr_jstype(vm.get_objs(), vm.clone(), ptr, 2256377725);
		});
		factory.call(0, Atom::from("_$async"), real_args, Atom::from((*name).to_string() + " rpc task"));
	}
}

impl AsyncRequestHandler {
	//构建一个处理器
	pub fn new(len: usize, factory: VMFactory, mgr: Mgr) -> Self {
		AsyncRequestHandler {
			factory: Arc::new(factory),
			mgr: mgr,
			gray_tab: GrayTab::new(),
		}
	}

	//获取默认虚拟机工厂和事务管理器
	pub fn get_default(&self) -> (Arc<VMFactory>, Mgr) {
		(self.factory.clone(), self.mgr.clone())
	}

	//设置指定灰度为默认版本
	pub fn set_default(&mut self, gray: usize) {
		match self.gray_tab.remove(gray) {
			None => return,
			Some((f, m)) => {
				self.factory = f;
				self.mgr = m;
			},
		}
	}

	//获取指定的虚拟机工厂和事务管理器
	fn get(&self, session: Arc<dyn Env>) -> (Arc<VMFactory>, Mgr) {
		match session.get_attr(Atom::from("_$gray")) {
			Some(val) => {
				match val {
					GenType::Bin(bin) => {
						let gray = usize::from_le(unsafe { *(bin[..].as_ptr() as *mut usize) });
						match self.gray_tab.get(gray) {
							None => self.get_default(),
							Some(r) => r,
						}
					},
					_ => self.get_default(),
				}
			},
			_ => self.get_default(),
		}
	}
}

/*
* 异步阻塞请求处理器
*/
pub struct AsyncBlockRequestHandler {
	factory: 	Arc<VMFactory>,	//默认虚拟机工厂
	mgr: 		Mgr,			//默认事务管理器
	gray_tab: 	GrayTab,		//灰度表
}

unsafe impl Send for AsyncBlockRequestHandler {}
unsafe impl Sync for AsyncBlockRequestHandler {}

impl Handler for AsyncBlockRequestHandler {
	type A = Arc<Vec<u8>>;
	type B = Vec<JSType>;
	type C = Option<u32>;
	type D = ();
	type E = ();
	type F = ();
	type G = ();
	type H = ();
	type HandleResult = ();

	fn handle(&self, env: Arc<dyn Env>, name: Atom, args: Args<Self::A, Self::B, Self::C, Self::D, Self::E, Self::F, Self::G, Self::H>) -> Self::HandleResult {
		let (factory, mgr) = self.get(env.clone());
        let copy_name = name.clone();
		let real_args = Box::new(move |vm: Arc<JS>| {
			vm.new_str((*copy_name).to_string());
			match args {
				Args::ThreeArgs(bin, objs, None) => {
					let buffer = vm.new_uint8_array(bin.len() as u32);
					buffer.from_bytes(bin.as_slice());
					let mut value: JSType;
					let array = vm.new_array();
					for i in 0..objs.len() {
						value = vm.new_native_object(objs[i].get_native_object());
						vm.set_index(&array, i as u32, &value);
					}
				},
				_ => panic!("invalid async block call handler args"),
			}
			let ptr = Box::into_raw(Box::new(mgr.clone())) as usize;
			ptr_jstype(vm.get_objs(), vm.clone(), ptr, 2976191628);
			let ptr = Box::into_raw(Box::new(env.clone())) as usize;
			ptr_jstype(vm.get_objs(), vm.clone(), ptr, 2256377725);
		});
		factory.call(0, Atom::from("_$sync"), real_args, Atom::from((*name).to_string() + " rpc task"));
	}
}

impl AsyncBlockRequestHandler {
	//构建一个处理器
	pub fn new(len: usize, factory: VMFactory, mgr: Mgr) -> Self {
		AsyncBlockRequestHandler {
			factory: Arc::new(factory),
			mgr: mgr,
			gray_tab: GrayTab::new(),
		}
	}

	//获取默认虚拟机工厂和事务管理器
	pub fn get_default(&self) -> (Arc<VMFactory>, Mgr) {
		(self.factory.clone(), self.mgr.clone())
	}

	//设置指定灰度为默认版本
	pub fn set_default(&mut self, gray: usize) {
		match self.gray_tab.remove(gray) {
			None => return,
			Some((f, m)) => {
				self.factory = f;
				self.mgr = m;
			},
		}
	}

	//获取指定的虚拟机工厂和事务管理器
	fn get(&self, session: Arc<dyn Env>) -> (Arc<VMFactory>, Mgr) {
		match session.get_attr(Atom::from("_$gray")) {
			Some(val) => {
				match val {
					GenType::Bin(bin) => {
						let gray = usize::from_le(unsafe { *(bin[..].as_ptr() as *mut usize) });
						match self.gray_tab.get(gray) {
							None => self.get_default(),
							Some(r) => r,
						}
					},
					_ => self.get_default(),
				}
			},
			_ => self.get_default(),
		}
	}
}