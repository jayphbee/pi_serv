use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicUsize;

use pi_vm::adapter::{JS, JSType};
use pi_vm::pi_vm_impl::VMFactory;
use pi_vm::bonmgr::{ptr_jstype, BON_MGR};
use pi_db::mgr::Mgr;
use pi_lib::atom::Atom;
use pi_lib::handler::{Handler, Args};
use pi_lib::gray::{Gray, GrayVersion, GrayTab};
use pi_lib::sbtree::{Tree};
use pi_lib::ordmap::OrdMap;

//NativeObject, 灰度系统需要使用
#[derive(Clone, Debug)]
pub struct Nobj {
    ptr: usize, //指针
    hash: u32, //hash
    path: Atom, //模块路径
    name: Atom, //Object名称
}

impl Drop for Nobj{
    fn drop(&mut self){
        println!("drop Nobj!");
        let struct_metas = BON_MGR.struct_metas.lock().unwrap();
        let meta = struct_metas.get(&self.hash).unwrap();
        (meta.drop_fn)(self.ptr);
    }
}

impl Nobj {
    pub fn new(ptr: usize, hash: u32, path: Atom, name: Atom) -> Self {
        Nobj{
            ptr,
            hash,
            path,
            name
        }
    }
}

//灰度
#[derive(Clone)]
pub struct JSGray {
    mgr: Mgr, //数据库管理器
    factory: Arc<VMFactory>, //虚拟机工厂
    nobj_metas: OrdMap<Tree<Atom, Nobj>> //本地对象
}

impl JSGray {
    pub fn new(mgr: Mgr, factory: VMFactory) -> Self{
        JSGray{
            mgr,
            factory: Arc::new(factory),
            nobj_metas: OrdMap::new(None)
        }
    }

    //设置NativeObject， obj应该是本地对象的所有权, 如果灰度表中存在名为key的对象， 将会覆盖
    pub fn set_obj(&mut self, key: String, obj: &JSType, path: String, name: String, js: &Arc<JS>) -> Result<bool, String> {
        if !obj.is_native_object(){
            return Err(String::from("obj is not NativeObject"));
        }
        let ptr = obj.get_native_object();
        let objs = js.get_objs();
        let hash = match objs.borrow_mut().remove(&ptr) {
            Some(v) => v.meta_hash,
            None => return Err(String::from("NativeObj is not exist, key:") + key.as_str()),
        };
        self.nobj_metas.insert(Atom::from(key), Nobj::new(ptr, hash, Atom::from(path) , Atom::from(name)));
        Ok(true)
    }
}

impl Gray for JSGray {}
/*
* Topic处理器
*/
pub struct TopicHandler {
	//len: 		AtomicUsize,	//处理器消息队列最大长度
	gray_tab: 	Arc<RwLock<GrayTab<JSGray>>>, //灰度表
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

	fn handle(&self, env: Arc<dyn GrayVersion>, topic: Atom, args: Args<Self::A, Self::B, Self::C, Self::D, Self::E, Self::F, Self::G, Self::H>) -> Self::HandleResult {
        let gray_tab = self.gray_tab.read().unwrap();
        let gray = match env.get_gray() {
            Some(v) => match gray_tab.get(v) {
                Some(g) => g,
                None => panic!("gray is not exist, version:{}", v),
            },
            None => gray_tab.get_last(),
        };
        let mgr = gray.mgr.clone();
        let topic_name = topic.clone();
		let real_args = Box::new(move |vm: Arc<JS>| -> usize {
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
			ptr_jstype(vm.get_objs(), vm.clone(), ptr, 226971089);
			4
		});
		gray.factory.call(0, Atom::from("_$rpc"), real_args, Atom::from((*topic).to_string() + " rpc task"));
	}
}

impl TopicHandler {
	//构建一个处理器
	pub fn new(len: usize, gray: JSGray) -> Self {
		TopicHandler {
			//len: AtomicUsize::new(len),
			gray_tab: Arc::new(RwLock::new(GrayTab::new(gray))) ,
		}
	}
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
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!async call start, copy_name: {:?}", copy_name);
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
						vm.set_index(&array, i as u32, &value);
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
						vm.set_index(&array, i as u32, &value);
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
		gray.factory.call(0, Atom::from("_$async"), real_args, Atom::from((*name).to_string() + " rpc task"));
	}
}

impl AsyncRequestHandler {
	//构建一个处理器
	pub fn new(len: usize, gray: JSGray) -> Self {
		AsyncRequestHandler {
			gray_tab: Arc::new(RwLock::new(GrayTab::new(gray))),
		}
	}
}

