use std::sync::{Arc, RwLock};

use pi_vm::adapter::{JS, JSType, dukc_pop};
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
use js_env::{env_var};
use std::str::FromStr;
use std::cell::RefCell;
use json;

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

/**
* 本地对象表
*/
#[derive(Clone)]
pub struct Nobjs {
    pub nobjs: Tree<Atom, Arc<Nobj>> //本地对象
}


impl Nobjs {
    /**
    * 构建本地对象表
    * @returns 返回本地对象表
    */
    pub fn new() -> Self{
        Nobjs{
            nobjs: None
        }
    }

    /**
    * 设置NativeObject， obj应该是本地对象的所有权, 如果灰度表中存在名为key的对象， 将会覆盖
    */
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
            let type_name = format!("Module.modules['{}.js'].exports{}", r.0, r.1);
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

/**
* 灰度对象
*/
#[derive(Clone)]
pub struct JSGray {
    pub mgr: Mgr, //数据库管理器
    pub factory: Arc<VMFactory>, //虚拟机工厂
    pub nobjs: Nobjs, //本地对象
    pub name: Atom,//为灰度取一个名称， 所有灰度不能重复
}

impl JSGray {
    /**
    * 构建灰度对象
    * @param mgr 表库及事务管理器
    * @param name 灰度对象名
    * @param nobjs 本地对象表
    * @returns 返回灰度对象
    */
    pub fn new(mgr: &Mgr, factory: VMFactory, name: &str, nobjs: &Nobjs) -> Self{
        JSGray{
            mgr: mgr.clone(),
            factory: Arc::new(factory),
            nobjs: nobjs.clone(),
            name: Atom::from(name),
        }
    }

    /**
    * 设置NativeObject， obj应该是本地对象的所有权, 如果灰度表中存在名为key的对象， 将会覆盖
    */
    pub fn set_obj(&mut self, key: String, obj: &JSType, path: String, name: String, js: &Arc<JS>) -> Result<bool, String> {
        self.nobjs.set_obj(key, obj, path, name, js)
    }
}

impl Gray for JSGray {}

/**
* 创建灰度表
*/
pub fn create_gray_tab(gray: JSGray) -> Arc<RwLock<GrayTab<JSGray>>>{
    Arc::new(RwLock::new(GrayTab::new(gray)))
}

/**
* 获取全局唯一id
* @param guid 全局唯一id生成器
* @param ctrl_id 本地节点编号
* @returns 返回全局唯一id
*/
pub fn guid_gen(guid: &GuidGen, ctrl_id: u16) -> u128 {
    guid.gen(ctrl_id).0
} 

/**
* 比较两个BonBuffer
* @param b1 BonBuffer
* @param b2 BonBuffer
* @returns 返回比较结果，-1表示小于，0表示相同，1表示大于
*/
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

pub struct Json (json::JsonValue);

/**
 * 使用key获取值
 * 可以使用形如“yyyy.zzzz.xxxx”结构的字符串作为key，来获取子结构中的值
 * 如果key = “”， 表示获取整个json
 */
pub fn get_json_value(j: &Json, key: String, js: &Arc<JS>) -> Result<JSType, String> {
	let arr: Vec<&str> = key.split(".").collect();
	let mut v = &j.0;

	for i in 0..arr.len() {
		v = match get_value(v, &arr[i]) {
			Some(r) => r,
			None => return Err(format!("get_json err, key: {}", key)),
		}
	}
	json_to_js_type(v, js)
}

pub fn create_share_json() -> Arc<RefCell<Json>> {
	Arc::new(RefCell::new(Json(json::JsonValue::new_object())))
}

pub fn set_json_value(j: &Arc<RefCell<Json>>, key: String, value: String) -> Result<(), String> {
	let arr: Vec<&str> = key.split(".").collect();
	let mut r = j.borrow_mut();
	let mut v = &mut r.0;
	if arr.len() == 0 {
		return Err(format!("set_json_value fail, key: {}", key));
	}

	if arr.len() > 1 {
		for i in 0..arr.len() {
			v = match get_value_mut(v, &arr[i]) {
				Some(r) => r,
				None => return Err(format!("set_json_value fail, key: {}", key)),
			}
		}
	}
	
	let value = match json::parse(value.as_str()) {
		Ok(r) => r,
		Err(s) => return Err(s.to_string()),
	};
	match v {
		json::JsonValue::Object(o) => {o.insert(arr[arr.len() - 1], value)},
		json::JsonValue::Array(o) => {
			if let Ok(i) = usize::from_str(arr[arr.len() - 1]) {
				o.insert(i, value);
			} else {
				return Err(format!("set_json_value fail, key: {}", key))
			}
		},
		_ => ()
	};
	Ok(())
}

fn get_value<'a, 'b>(j: &'a json::JsonValue, key:&'b str) -> Option<&'a json::JsonValue> {
	match j {
		json::JsonValue::Object(obj) => obj.get(key),
		json::JsonValue::Array(arr) => {
			if let Ok(k) = usize::from_str(key) {
				if arr.len() > k {
					return Some(&arr[k]);
				}
			}
			None
		},
		_ => None,
	}
}

fn get_value_mut<'a, 'b>(j: &'a mut json::JsonValue, key:&'b str) -> Option<&'a mut json::JsonValue> {
	match j {
		json::JsonValue::Object(obj) => obj.get_mut(key),
		json::JsonValue::Array(arr) => {
			if let Ok(k) = usize::from_str(key) {
				if arr.len() > k {
					return Some(&mut arr[k]);
				}
			}
			None
		},
		_ => None,
	}
}

fn json_to_js_type<'a, 'b>(j: &'a json::JsonValue, js: &Arc<JS>) -> Result<JSType, String> {
	match j {
		json::JsonValue::Object(json_obj) => {
			let obj = js.new_object();
			for v in json_obj.iter() {
				let mut value = match json_to_js_type(&v.1, js) {
					Ok(r) => r,
					Err(s) => {
						unsafe { dukc_pop(js.get_vm()) };
                        return Err(s);
					},
				};
				js.set_field(&obj, String::from(v.0.to_string()), &mut value);
			}
			Ok(obj)
		},
		json::JsonValue::Array(json_obj) => {
			let obj = js.new_array();
			let mut i: u32 = 0;
			for v in json_obj.iter() {
				let mut value = match json_to_js_type(v, js) {
					Ok(r) => r,
					Err(s) => {
						unsafe { dukc_pop(js.get_vm()) };
                        return Err(s);
					},
				};
				js.set_index(&obj, i, &mut value);
				i += 1;
			}
			Ok(obj)
		},
		json::JsonValue::Boolean(b) => Ok(js.new_boolean(*b)),
		json::JsonValue::Short(s) => js.new_str(s.as_str().to_string()),
		json::JsonValue::String(s) => js.new_str(s.clone()),
		json::JsonValue::Number(n) => Ok(js.new_f64(n.clone().into())),
		json::JsonValue::Null => Ok(js.new_undefined()),
	}
}
