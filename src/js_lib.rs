use std::sync::{Arc, RwLock};

use pi_vm::adapter::{JS, JSType};
use pi_vm::pi_vm_impl::VMFactory;
use pi_vm::bonmgr::{ptr_jstype, BON_MGR};
use pi_db::mgr::Mgr;
use atom::Atom;
use guid::GuidGen;
use gray::{Gray, GrayTab};
use ordmap::sbtree::{Tree};
use ordmap::ordmap::{Entry, ImOrdMap, Iter};
use bon::{partial_cmp, ReadBuffer};
use std::cmp::Ordering;

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

//本地对象
#[derive(Clone)]
pub struct Nobjs {
    pub nobjs: Tree<Atom, Arc<Nobj>> //本地对象
}


impl Nobjs {
    pub fn new() -> Self{
        Nobjs{
            nobjs: None
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

		match self.nobjs.insert(Atom::from(key), Arc::new(Nobj::new(ptr, hash, Atom::from(path) , Atom::from(name)))) {
			Some(root) => {
				self.nobjs = root;
				Ok(true)
			},
			_ => Ok(false),
		}
    }

    pub fn get_depend(&self) -> Vec<String>{
        let mut arr = Vec::new();
        for Entry(_, obj) in Iter::iter(&self.nobjs, None, false){
            let name = obj.path.as_str();
            let index = match name.find("."){
                Some(v) => v,
                None => panic!("illegal module name, lack '.', modName: {}", name),
            };
            let r = obj.path.split_at(index).0.to_string() + ".js";// r.0为模块名， r.1为类型名称;
            arr.push( r );
        }
        arr
    }

    pub fn to_map(&self, vm: &Arc<JS>) -> JSType {
        vm.get_type("Map".to_string());
        let temp = vm.new_array();
        let mut i = 0;
        for Entry(k, obj) in Iter::iter(&self.nobjs, None, false){
            let mut arr = vm.new_array();
            vm.set_index(&arr, 0, &mut vm.new_str(k.as_str().to_string()).unwrap());
            let name = obj.path.as_str();
            let index = match name.find("."){
                Some(v) => v,
                None => panic!("illegal module name, lack '.', modName: {}", name),
            };
            let r = obj.path.split_at(index);// r.0为模块名， r.1为类型名称;
            let type_name = String::from("pi_modules['") + r.0 + "']" + ".exports" + r.1;

            vm.get_type(type_name.clone());
            ptr_jstype(vm.get_objs_ref(), vm.clone(), obj.ptr, obj.hash);
            let mut obj = vm.new_type(type_name.clone(), 1);

            if obj.is_undefined(){
                panic!("module is not exist, please make sure the module has been loaded, modName:{}", type_name);
            }

            vm.set_index(&arr, 1, &mut obj);
            vm.set_index(&temp, i, &mut arr);
            i += 1;
        }
        let objs = vm.new_type("Map".to_string(), 1);
        objs
    }
}
//灰度
#[derive(Clone)]
pub struct JSGray {
    pub mgr: Mgr, //数据库管理器
    pub factory: Arc<VMFactory>, //虚拟机工厂
    pub nobjs: Nobjs, //本地对象
    pub name: Atom,//为灰度取一个名称， 所有灰度不能重复重复
}

impl JSGray {
    pub fn new(mgr: &Mgr, factory: VMFactory, name: &str, nobjs: &Nobjs) -> Self{
        JSGray{
            mgr: mgr.clone(),
            factory: Arc::new(factory),
            nobjs: nobjs.clone(),
            name: Atom::from(name),
        }
    }

    //设置NativeObject， obj应该是本地对象的所有权, 如果灰度表中存在名为key的对象， 将会覆盖
    pub fn set_obj(&mut self, key: String, obj: &JSType, path: String, name: String, js: &Arc<JS>) -> Result<bool, String> {
        self.nobjs.set_obj(key, obj, path, name, js)
    }
}

impl Gray for JSGray {}

pub fn create_gray_tab(gray: JSGray) -> Arc<RwLock<GrayTab<JSGray>>>{
    Arc::new(RwLock::new(GrayTab::new(gray)))
}

pub fn guid_gen(guid: &GuidGen, ctrl_id: u16) -> u128 {
    guid.gen(ctrl_id).0
} 

pub fn bonbuf_cmp(b1: &[u8], b2: &[u8]) -> Option<i32> {
    let mut b1= ReadBuffer::new(b1, 0);
    let mut b2 = ReadBuffer::new(b2, 0);

    match partial_cmp(&mut b1, &mut b2) {
        Some(Ordering::Less) => Some(-1),
        Some(Ordering::Equal) => Some(0),
        Some(Ordering::Greater) => Some(1),
        None => None
    }
}
