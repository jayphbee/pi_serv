use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicUsize;

use fnv::FnvHashMap;
use mqtt::handler::TopicHandle;
use mqtt::session::Session;

use pi_vm::adapter::JS;
use pi_vm::pi_vm_impl::VMFactory;
use pi_vm::bonmgr::{ptr_jstype, BON_MGR};
use pi_db::mgr::Mgr;
use pi_lib::atom::Atom;

/*
* Topic处理器
*/
pub struct TopicHandler {
	len: 		AtomicUsize,										    //处理器消息队列最大长度
	factory: 	Arc<VMFactory>,										    //默认虚拟机工厂
	mgr: 		Mgr,												    //默认事务管理器
	gray_tab: 	Arc<RwLock<FnvHashMap<usize, (Arc<VMFactory>, Mgr)>>>,	//灰度表
}

impl TopicHandle for TopicHandler {
	fn handle(&self, topic: Atom, _version: u8, session: Arc<Session>, bin: Arc<Vec<u8>>) {
		let (factory, mgr) = self.get(session.clone());
        let topic_name = topic.clone();
		let args = Box::new(move |vm: Arc<JS>| {
			vm.new_str((*topic_name).to_string());
			let array = vm.new_uint8_array(bin.len() as u32);
			array.from_bytes(bin.as_slice());
			let ptr = Box::into_raw(Box::new(mgr.clone())) as usize;
			ptr_jstype(BON_MGR.objs.clone(), vm.clone(), ptr, 2976191628);
			let ptr = Box::into_raw(Box::new(session.clone())) as usize;
			ptr_jstype(BON_MGR.objs.clone(), vm.clone(), ptr, 2256377725);
		});
		factory.call(0, args, Atom::from((*topic).to_string() + " rpc task"));
	}
}

impl TopicHandler {
	//构建一个处理器
	pub fn new(len: usize, factory: VMFactory, default: Mgr) -> Self {
		TopicHandler {
			len: AtomicUsize::new(len),
			factory: Arc::new(factory),
			mgr: default,
			gray_tab: Arc::new(RwLock::new(FnvHashMap::default())),
		}
	}

	//获取默认虚拟机工厂和事务管理器
	pub fn get_default(&self) -> (Arc<VMFactory>, Mgr) {
		(self.factory.clone(), self.mgr.clone())
	}

	//设置指定灰度为默认版本
	pub fn set_default(&mut self, gray: usize) {
		match self.gray_tab.write().unwrap().remove(&gray) {
			None => return,
			Some((f, m)) => {
				self.factory = f;
				self.mgr = m;
			},
		}
	}

	//获取指定灰度的虚拟机工厂和事务管理器
	pub fn get_gray(&self, gray: usize) -> Option<(Arc<VMFactory>, Mgr)> {
		match self.gray_tab.read().unwrap().get(&gray) {
			_ => None,
			Some((factory, mgr)) => Some((factory.clone(), mgr.clone())),
		}
	}

	//设置指定灰度的虚拟机工厂和事务管理器
	pub fn set_gray(&self, gray: usize, factory: VMFactory, mgr: Mgr) {
		self.gray_tab.write().unwrap().insert(gray, (Arc::new(factory), mgr));
	}

	//移除指定灰度的虚拟机工厂和事务管理器
	pub fn remove_gray(&self, gray: usize) {
		self.gray_tab.write().unwrap().remove(&gray);
	}

	//获取指定的虚拟机工厂和事务管理器
	fn get(&self, session: Arc<Session>) -> (Arc<VMFactory>, Mgr) {
		match session.get_attr(Atom::from("_$gray")) {
			Some(vec) => {
                let gray = usize::from_le(unsafe { *(vec[..].as_ptr() as *mut usize) });
				match self.get_gray(gray) {
					None => self.get_default(),
					Some(r) => r,
				}
			},
			_ => self.get_default(),
		}
	}
}