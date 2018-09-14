use std::sync::{Arc};

use pi_vm::adapter::{JS, JSType};
use pi_vm::pi_vm_impl::VMFactory;
use pi_vm::bonmgr::{ptr_jstype, BON_MGR};
use pi_db::mgr::Mgr;
use pi_lib::atom::Atom;
use pi_lib::gray::{Gray};
use pi_lib::sbtree::{Tree};
use pi_lib::ordmap::{Entry, ImOrdMap, Iter};

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

    pub fn to_json(&self, vm: &Arc<JS>) -> JSType {
        let objs = vm.new_object();
        for Entry(k, obj) in Iter::iter(&self.nobjs, None, false){
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

            vm.set_field(&objs, String::from(k.as_str()), &mut obj);
        }
        objs
    }
}
//灰度
#[derive(Clone)]
pub struct JSGray {
    pub mgr: Mgr, //数据库管理器
    pub factory: Arc<VMFactory>, //虚拟机工厂
    pub nobjs: Nobjs //本地对象
}

impl JSGray {
    pub fn new(mgr: &Mgr, factory: VMFactory, nobjs: &Nobjs) -> Self{
        JSGray{
            mgr: mgr.clone(),
            factory: Arc::new(factory),
            nobjs: nobjs.clone()
        }
    }

    //设置NativeObject， obj应该是本地对象的所有权, 如果灰度表中存在名为key的对象， 将会覆盖
    pub fn set_obj(&mut self, key: String, obj: &JSType, path: String, name: String, js: &Arc<JS>) -> Result<bool, String> {
        self.nobjs.set_obj(key, obj, path, name, js)
    }
}

impl Gray for JSGray {}