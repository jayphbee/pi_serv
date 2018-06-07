use std::sync::{Arc, RwLock};
use std::sync::atomic::{Ordering, AtomicUsize};
use std::io::{Read, Write, Result};

use fnv::FnvHashMap;
use mqtt::handler::TopicHandle;
use mqtt::session::Session;

use pi_vm::adapter::JS;
use pi_vm::pi_vm_impl::VMFactory;
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
	fn handle(&self, topic: Atom, version: u8, session: Arc<Session>, bin: Arc<Vec<u8>>) {
		let (factory, mgr) = self.get(session.clone());
        let topic_name = topic.clone();
		let args = Box::new(move |vm: JS| -> JS {
			vm.new_str((*topic_name).to_string());
			let array = vm.new_uint8_array(bin.len() as u32);
			array.from_bytes(bin.as_slice());
			vm.new_native_object(Arc::into_raw(Arc::new(mgr)) as usize);
			vm.new_native_object(Arc::into_raw(session.clone()) as usize);
			vm
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

	//获取指定灰度的事务管理器
	pub fn get_gray(&self, gray: usize) -> Option<(Arc<VMFactory>, Mgr)> {
		match self.gray_tab.read().unwrap().get(&gray) {
			_ => None,
			Some((factory, mgr)) => Some((factory.clone(), mgr.clone())),
		}
	}

	//设置指定灰度的管理器
	pub fn set_gray(&self, gray: usize, factory: VMFactory, mgr: Mgr) {
		self.gray_tab.write().unwrap().insert(gray, (Arc::new(factory), mgr));
	}

	//移除指定灰度的管理器
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

// /*
// * 消息处理
// */
// pub trait MsgHandle {
//     type PrevResult;
//     type NextResult;

//     fn handle(&self, Self::PrevResult) -> Self::NextResult;
// }

// struct MsgHandler {

// }

// impl MsgHandle for MsgHandler {
//     type PrevResult = Result<(Arc<ClientStub>, Arc<[u8]>)>;
//     type NextResult = Result<(Arc<ClientStub>, Atom, BonBuffer)>;

//     fn handle(&self, data: Self::PrevResult) -> Self::NextResult {
// 		data.and_then(|(stub, bin)| {
// 			let tail = bin.len();
// 			Ok((stub, Atom::from(""), BonBuffer::with_bytes(bin.to_vec(), Some(0), Some(tail))))
// 		})
//     }
// }

// impl MsgProtocolHandler {
// 	//构建一个消息协议处理器
// 	pub fn new(msg: Arc<[u8]>) -> Self {
// 		MsgProtocolHandler {
// 			msg: msg,
// 		}
// 	}
// }

// /*
// * 消息事务处理器
// */
// pub struct MsgTxHandler {

// }

// impl MsgHandle for MsgTxHandler {
// 	type PrevResult = <MsgProtocolHandler as MsgHandle>::NextResult;
// 	type NextResult = bool;

// 	fn handle(&self, data: Self::PrevResult) -> Self::NextResult {
// 		true
// 	}
// }

// /*
// * 消息处理链
// */
// pub trait MsgHandleChain {
//     //创建一个没有队列的链，消息的所有处理都交由当前线程完成
// 	fn new() -> Self;

// 	//设置指定消息队列长度的链，当前线程只负责将消息队列头中的消息投递到任务池中执行
// 	fn with_queue(usize) -> Self;

// 	//为链尾部增加处理器，当前链上的所有处理器，都会在一个线程中同步执行完成
// 	fn link<T: MsgHandle>(self, Arc<T>) -> Self;

//     //获取当前链的消息队列长度
// 	fn len(&self) -> usize;

// 	//获取当前链的消息数量
// 	fn size(&self) -> usize;
// }