use std::sync::{Arc};
use std::collections::HashMap;
use std::boxed::FnBox;
use std::path::Path;
use std::fs;

use mqtt3::QoS;

use pi_db::memery_db::{DB};
use pi_db::db::{TabKV, Iter, Ware, Bin, TabMeta};
use pi_db::util::{dump as db_dump, restore as db_restore};
use pi_db::mgr::{Monitor, Event, EventType, Mgr, Tr};
use pi_lib::bon::{Decode, Encode, ReadBuffer, WriteBuffer};
use pi_lib::atom::Atom;
use pi_math::hex::ToHex;
use pi_vm::adapter::{JSType, JS};
use pi_vm::pi_vm_impl::VMFactory;
use pi_vm::bonmgr::{ptr_jstype};
//use pi_store::db::{DB as FileDB};
use mqtt::server::ServerNode;
use mqtt::data::Server;

//use pi_base::util::now_millisecond;
//use pi_vm::adapter::dukc_top;

use js_util::{decode_by_type, decode_by_tabkv};


type DBIterTrait = Box<Iter<Item=(Bin, Bin)>>;
/**
 * 封装类db迭代器， 是其由traiobj转化为具体类型（构建工具暂时不支持traitobj的构建）
 * */
pub struct DBIter(DBIterTrait, Arc<TabMeta>);

impl DBIter{
    pub fn next(&mut self, cb: Arc<Fn(Result<Option<(Arc<Vec<u8>>, Arc<Vec<u8>>)>, String>)>) -> Option<Result<Option<(Arc<Vec<u8>>, Arc<Vec<u8>>)>, String>>{
        match self.0.next(cb.clone()) {
            Some(v) => {cb(v); None},
            None => None,
        }
    }

    pub fn next_elem(&mut self, cb: Arc<Fn(Result<Option<JSType>, String>)>, js: &Arc<JS>) -> Option<Result<Option<JSType>, String>>{
        let js = js.clone();
        let js1 = js.clone();
        let meta = self.1.clone();
        let meta1 = self.1.clone();
        let call_back = move|r: Result<Option<(Arc<Vec<u8>>, Arc<Vec<u8>>)>, String>|{
            match r {
                Ok(v) => {
                    match v {
                        Some(value) => {
                            let m = meta.clone();
                            let arr = js.new_array();
                            let mut k = match decode_by_type(&js, &mut ReadBuffer::new(&value.0, 0) , &m.k) {
                                Ok(v) => v,
                                Err(s) => {cb(Err(s)); return;},
                            };
                            js.set_index(&arr, 0, &mut k);
                            let mut v = match decode_by_type(&js, &mut ReadBuffer::new(&value.1, 0) ,  &m.v) {
                                Ok(v) => v,
                                Err(s) => {cb(Err(s)); return;},
                            };
                            js.set_index(&arr, 1, &mut v);
                            js.set_global_var("_$rust_r".to_string(), arr);
                            cb(Ok(Some(js.new_undefined())));
                        },
                        None => cb(Ok(None)),
                    };
                },
                Err(s) => cb(Err(s)),
            }
        };

        match self.0.next(Arc::new(call_back)) {
            Some(v) => {
                match v {
                    Ok(v) => {
                        match v {
                            Some(value) => {
                                let arr = js1.new_array();
                                let mut k = match decode_by_type(&js1, &mut ReadBuffer::new(&value.0, 0) , &meta1.k) {
                                    Ok(v) => v,
                                    Err(s) => return Some(Err(s)),
                                };
                                js1.set_index(&arr, 0, &mut k);
                                let mut v = match decode_by_type(&js1, &mut ReadBuffer::new(&value.1, 0) ,  &meta1.v) {
                                    Ok(v) => v,
                                    Err(s) => return Some(Err(s)),
                                };
                                js1.set_index(&arr, 1, &mut v);
                                Some(Ok(Some(arr)))
                            },
                            None => Some(Ok(None)),
                        }
                    },
                    Err(s) => Some(Err(s)),
                }
            },
            None => None,
        }
    }

}

pub struct DBWare(Arc<Ware>);

// 取到数据库的迭代器
pub fn iter_db(tr: &Tr, ware: String, tab: String, key: Option<&[u8]>, descending: bool, _filter: Option<String>, cb: Arc<Fn(Result<DBIter, String>)>) -> Option<Result<DBIter, String>> {
    let ware = Atom::from(ware);
    let tab = Atom::from(tab);
    let key = match key {
        Some(v) => Some(Arc::new(Vec::from(v))),
        None => None,
    };

    //取元信息
    let meta = match tr.tab_info(&ware, &tab){
        Some(v) => v,
        None => {return Some(Err(String::from("meta is not exist")))}, //元信息不存在，不可能生成迭代器， 因此直接返回None
    };
    let meta1 = meta.clone();

    let cb1 = move |r:Result<DBIterTrait, String>|{
        match r {
            Ok(v) => cb(Ok(DBIter(v, meta.clone()))),
            Err(s) => cb(Err(s)),
        }
    };
    let cb1 =  Arc::new(cb1);
    match tr.iter(&ware, &tab, key, descending, None, cb1.clone()) {
        Some(v) => match v {
            Ok(v) => Some(Ok(DBIter(v, meta1))),
            Err(s) => Some(Err(s)),
        },
        None => None,
    } 
}

pub fn clone_db_mgr(mgr: &Mgr) -> Mgr{
    mgr.clone()
}

// 注册内存数据库
pub fn register_memery_db(mgr: &Mgr, prefix: String, ware: DB) -> bool {
	mgr.register(Atom::from(prefix), Arc::new(ware))
}

// 注册文件数据库
// pub fn register_file_db(mgr: &Mgr, prefix: String, ware: FileDB) -> bool {
// 	mgr.register(Atom::from(prefix), Arc::new(ware))
// }

//new TabKV
pub fn tabkv_with_value(ware: &str, tab: &str, key: &[u8], value: &[u8]) -> TabKV {
    TabKV{
        ware: Atom::from(ware),
        tab: Atom::from(tab),
        key: Arc::new(Vec::from(key)),
        index: 0,
        value: Some(Arc::new(Vec::from(value))),
    }
}

//new TabKV
pub fn tabkv_new(ware: &str, tab: &str, key: &[u8]) -> TabKV {
    TabKV{
        ware: Atom::from(ware),
        tab: Atom::from(tab),
        key: Arc::new(Vec::from(key)),
        index: 0,
        value: None,
    }
}

//TabKV get_value
pub fn tabkv_get_value(tabkv: &TabKV) -> Option<Arc<Vec<u8>>> {
    tabkv.value.clone()
}

//插入元信息
pub fn alter(tr: &Tr, ware: String, tab: String, meta_buf: Option<&[u8]>, cb: Arc<Fn(Result<(), String>)>) -> Option<Result<(), String>>{
    let meta = match meta_buf {
        Some(buf) => Some(Arc::new(TabMeta::decode(&mut ReadBuffer::new(buf, 0)))),
        None => None,
    };
    let r = tr.alter(&Atom::from(ware), &Atom::from(tab), meta, cb);
    r
}

//修改数据库数据
pub fn modify(tr: &Tr, items: &JSType, lock_time: Option<usize>, read_lock: bool, cb: Arc<Fn(Result<(), String>)>) -> Option<Result<(), String>>{
    let param_error = String::from("param error in modify");
    if !items.is_array() {
        return Some(Err(param_error));
    }
    let a_len = items.get_array_length();
    let mut arr = Vec::new();
    for i in 0..a_len {

        let elem_e = items.get_index(i as u32);
        if !elem_e.is_array() {
            return Some(Err(param_error));
        }
        let elem_e_e = elem_e.get_index(0);
        if !elem_e_e.is_string() {
            return Some(Err(param_error));
        }
        let elem_e_0 = elem_e_e.get_str();
        let elem_e_e = elem_e.get_index(1);
        if !elem_e_e.is_string() {
            return Some(Err(param_error));
        }
        let elem_e_1 = elem_e_e.get_str();

        let elem_e_e = elem_e.get_index(2);
        if !elem_e_e.is_uint8_array() && !elem_e_e.is_array_buffer() {
            return Some(Err(param_error));
        }
        let elem_e_2 = elem_e_e.to_bytes();

        let elem_e_e = elem_e.get_index(3);
        let elem_e_3 = if elem_e_e.is_null() || elem_e_e.is_undefined(){
            None
        }else if !elem_e_e.is_uint8_array() && !elem_e_e.is_array_buffer() {
            return Some(Err(param_error));
        }else{
            Some(Arc::new(elem_e_e.into_vec()))
        };

        arr.push(TabKV{
            ware: Atom::from(elem_e_0.clone()),
            tab: Atom::from(elem_e_1.clone()),
            key: Arc::new(Vec::from(elem_e_2)),
            value: elem_e_3,
            index:0
        });
    }

    tr.modify(arr, lock_time, read_lock, cb)
}

//查询数据库
pub fn query (tr: &Tr, items: &JSType, lock_time: Option<usize>, read_lock: bool, cb: Arc<Fn(Result<JSType, String>)>, js: &Arc<JS>) -> Option<Result<JSType, String>>{
    let param_error = String::from("param error in query");
    if !items.is_array() {
        return Some(Err(param_error));
    }
    let a_len = items.get_array_length();
    let mut arr = Vec::new();
    for i in 0..a_len {
        let elem_e = items.get_index(i as u32);
        if !elem_e.is_array() {
            return Some(Err(param_error));
        }
        let elem_e_e = elem_e.get_index(0);
        if !elem_e_e.is_string() {
            return Some(Err(param_error));
        }
        let elem_e_0 = elem_e_e.get_str();

        let elem_e_e = elem_e.get_index(1);
        if !elem_e_e.is_string() {
            return Some(Err(param_error));
        }
        let elem_e_1 = elem_e_e.get_str();

        let elem_e_e = elem_e.get_index(2);
        if !elem_e_e.is_uint8_array() && !elem_e_e.is_array_buffer() {
            return Some(Err(param_error));
        }
        let elem_e_2 = Arc::new(elem_e_e.into_vec());

        arr.push(TabKV{
            ware: Atom::from(elem_e_0.clone()),
            tab: Atom::from(elem_e_1.clone()),
            key: elem_e_2,
            value: None,
            index:0
        });
    }

    let js1 = js.clone();
    let tr1 = tr.clone();
    let call_back = move|r: Result<Vec<TabKV>, String>|{
        match r {
            Ok(v) => {
                let arr = js1.new_array();
                for i in 0..v.len(){
                    let elem = &v[i];
                    let mut r = match decode_by_tabkv(&js1, elem, &tr1.tab_info(&elem.ware, &elem.tab).unwrap()) {
                        Ok(v) => v,
                        Err(s) => {cb(Err(s)); return;},
                    };
                    js1.set_index(&arr, i as u32, &mut r);
                }
                js1.set_global_var("_$rust_r".to_string(), arr);
                cb(Ok(js1.new_undefined()));
            },
            Err(s) => cb(Err(s)),
        }
    };
    match tr.query(arr, lock_time, read_lock, Arc::new(call_back)) {
        Some(r) => {
            match r {
                Ok(v) => {
                    let arr = js.new_array();
                    for i in 0..v.len(){
                        let elem = &v[i];
                        let mut r = match decode_by_tabkv(&js, elem, &tr.tab_info(&elem.ware, &elem.tab).unwrap()) {
                            Ok(v) => v,
                            Err(s) => return Some(Err(s)),
                        };
                        js.set_index(&arr, i as u32, &mut r);
                    }
                    Some(Ok(arr))
                },
                Err(s) => Some(Err(s)),
            }
        },
        None => None,
    }
}

//数据库监听器
pub struct DBToMqttMonitor{
    cfg: HashMap<Atom, HashMap<Atom, bool>>,
    mqtt_server: ServerNode
}

impl DBToMqttMonitor{
    pub fn new(mqtt_server: &ServerNode, cfg: &[u8]) -> DBToMqttMonitor{
        let r = HashMap::decode(&mut ReadBuffer::new(cfg, 0));
        println!("new DBToMqttMonitor----------------{:?}", &r);
        DBToMqttMonitor{
            cfg:r,
            mqtt_server: mqtt_server.clone()
        }
    }
}

pub fn register_db_to_mqtt_monitor(mgr: &Mgr, monitor: DBToMqttMonitor){
    println!("register_db_to_mqtt_monitor-------------------------------------------------------------");
    mgr.listen(Arc::new(monitor));
}

impl Monitor for DBToMqttMonitor{
    fn notify(&self, e: Event, _mgr: Mgr){
        //如果名单中没有对应的库和表， 忽略该事件
        match self.cfg.get(&e.ware) {
            Some(tabs) => {
                match tabs.get(&e.tab){
                    Some(_) => (),
                    None => return,
                }
            },
            None => return,
        }

        println!("db listen1-------------------------------------------ware{:?}, tab:{:?}", &e.ware, &e.tab);

        //否则，将该事件投递到mqtt TODO
        match &e.other {
            &EventType::Tab{key: ref k, value: ref v} => {
                let topic = String::from(*&e.ware.as_str()) + "." + &*e.tab.as_str() + "." + k.to_hex().as_str();
                let value = match v {
                    Some(v) => Vec::from(v.as_slice()),
                    None => {
                        let mut wb = WriteBuffer::with_capacity(1);
                        wb.write_nil();
                        wb.unwrap()
                    },
                };
                println!("db listen-------------------------------------------{:?}, topic:{}", value, &topic);
                match self.mqtt_server.publish(false, QoS::AtMostOnce, Atom::from(topic), value) {
                    Ok(_) => (),
                    Err(r) => println!("db listen reponse fail:{}", r),
                } ;
            },
            &EventType::Meta(ref info) => {
                let topic = String::from(*&e.ware.as_str()) + "." + &*e.tab.as_str();
                let value = match info {
                    Some(v) => {
                        let mut wb = WriteBuffer::with_capacity(1);
                        v.encode(&mut wb);
                        wb.unwrap()
                    },
                    None => {
                        let mut wb = WriteBuffer::with_capacity(1);
                        wb.write_nil();
                        wb.unwrap()
                    },
                };
                match self.mqtt_server.publish(false, QoS::AtMostOnce, Atom::from(topic), value) {
                    Ok(_) => (),
                    Err(r) => println!("{}", r),
                } ;
            },
        }
    }
}

pub fn dump(mgr: &Mgr, ware: String, tab: String, file: String, cb: Arc<Fn(Result<(), String>)>) {
    let dir = match file.as_str().rfind("/") {
        Some(v) => &file[0..v],
        None => {panic!("restore file Invalid:{}", file);},
    };

    if !Path::new(&dir).exists(){
        fs::DirBuilder::new().recursive(true).create(dir).unwrap();
    }
    db_dump(mgr, Atom::from(ware), Atom::from(tab), file.clone(), cb);
}

pub fn restore(mgr: &Mgr, ware: String, tab: String, file: String, cb: Box<FnBox(Result<(), String>)>){
    let dir = match file.as_str().rfind("/") {
        Some(v) => &file[0..v],
        None => {panic!("restore file Invalid:{}", file);},
    };
    if !Path::new(&dir).exists(){
        fs::DirBuilder::new().recursive(true).create(dir).unwrap();
    }

    if !Path::new(&file).exists(){
        fs::File::create(&file).expect("");
    }
    db_restore(mgr, Atom::from(ware), Atom::from(tab), Atom::from(file.clone()), cb);
}

// 表的元信息
// pub fn tab_info(mgr: &Mgr, ware_name:String, tab_name: String) -> Option<Arc<TabMeta>> {
//     match mgr.tab_info(&Atom::from(ware_name), &Atom::from(tab_name)) {
//         Some(b) => b.tab_info(tab_name),
//         _ => None
//     }
// }

/*
* 数据库监听器
*/
pub struct JSDBMonitor {
    handler: Atom, //处理函数名称（js函数）
    factory:Arc<VMFactory>,
}

pub fn register_db_js_db_monitor(mgr: &Mgr, monitor: JSDBMonitor){
    mgr.listen(Arc::new(monitor));
}

impl Monitor for JSDBMonitor{
    fn notify(&self, e: Event, mgr: Mgr){
        //否则，将该事件投递到mqtt TODO
        if e.ware.as_str() != "file" {
            return;
        }
        match &e.other {
            &EventType::Tab{key: ref k, value: ref v} => {
                let k = k.clone();
                let v = v.clone();
                let ware = e.ware.clone();
                let tab = e.tab.clone();
                let real_args = Box::new(move |vm: Arc<JS>| -> usize {
                    let event = vm.new_object();
                    vm.set_field(&event, String::from("event_name"), &mut vm.new_str("db_change".to_string()));
                    vm.set_field(&event, String::from("ware"), &mut vm.new_str(ware.as_str().to_string()));// ware
                    vm.set_field(&event, String::from("tab"), &mut vm.new_str(tab.as_str().to_string()));// tab
                    vm.set_field(&event, String::from("key"), &mut ptr_jstype(vm.get_objs(), vm.clone(), Box::into_raw(Box::new(k)) as usize, 2886438122));//key
                    match v {
                        Some(v) => {vm.set_field(&event, String::from("value"), &mut ptr_jstype(vm.get_objs(), vm.clone(), Box::into_raw(Box::new(v)) as usize, 2886438122));},//value,
                        None => (),
                    };
                    //mgr
                    ptr_jstype(vm.get_objs(), vm.clone(), Box::into_raw(Box::new(mgr.clone())) as usize, 2976191628);
                    vm.new_undefined();
                    vm.new_object();
                    4
                });
                self.factory.call(0, self.handler.clone(), real_args, Atom::from("db_change".to_string() + " rpc task"));
            },
            &EventType::Meta(ref _info) => (),
        }
    }
}

impl JSDBMonitor {
	//构建一个监听器
	pub fn new(handler: String, factory: VMFactory) -> JSDBMonitor {
		JSDBMonitor {
            handler: Atom::from(handler),
            factory: Arc::new(factory)
        }
	}
}