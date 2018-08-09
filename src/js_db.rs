use std::sync::Arc;
use std::collections::HashMap;

use pi_db::memery_db::{DB};
use pi_db::db::{TabKV, Iter, Ware, Bin};
use pi_db::mgr::{Monitor, Event, EventType, Mgr, Tr};
use pi_store::db::{DB as FileDB};
use pi_lib::bon::{Decode, Encode, ReadBuffer, WriteBuffer};
use pi_lib::atom::Atom;
use pi_math::hex::ToHex;
use mqtt::server::ServerNode;
use mqtt::data::Server;
use mqtt3::QoS;


type DBIterTrait = Box<Iter<Item=(Bin, Bin)>>;
/**
 * 封装类db迭代器， 是其由traiobj转化为具体类型（构建工具暂时不支持traitobj的构建）
 * */
pub struct DBIter(DBIterTrait);

impl DBIter{
    pub fn next(&mut self, cb: Arc<Fn(Result<Option<(Arc<Vec<u8>>, Arc<Vec<u8>>)>, String>)>) -> Option<Result<Option<(Arc<Vec<u8>>, Arc<Vec<u8>>)>, String>>{
        match self.0.next(cb.clone()) {
            Some(v) => {cb(v); None},
            None => None,
        }
    }
}

pub struct DBWare(Arc<Ware>);

pub fn clone_db_mgr(mgr: &Mgr) -> Mgr{
    mgr.clone()
}

// 取到数据库的迭代器
pub fn iter_db(tr: &Tr, ware: String, tab: String, key: Option<&[u8]>, descending: bool, _filter: Option<String>, cb: Arc<Fn(Result<DBIter, String>)>) -> Option<Result<DBIter, String>> {
    let key = match key {
        Some(v) => Some(Arc::new(Vec::from(v))),
        None => None,
    };

    let cb1 = move |r:Result<DBIterTrait, String>|{
        match r {
            Ok(v) => cb(Ok(DBIter(v))),
            Err(s) => cb(Err(s)),
        }
    };
    let cb1 =  Arc::new(cb1);
    let ware = Atom::from(ware);
    let tab = Atom::from(tab);
    match tr.iter(&ware, &tab, key, descending, None, cb1.clone()) {
        Some(v) => match v {
            Ok(v) => Some(Ok(DBIter(v))),
            Err(s) => Some(Err(s)),
        },
        None => None,
    } 
}

// 注册内存数据库
pub fn register_memery_db(mgr: &Mgr, prefix: String, ware: DB) -> bool {
	mgr.register(Atom::from(prefix), Arc::new(ware))
}

// 注册文件数据库
pub fn register_file_db(mgr: &Mgr, prefix: String, ware: FileDB) -> bool {
	mgr.register(Atom::from(prefix), Arc::new(ware))
}

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

//数据库监听器
pub struct DBToMqttMonitor{
    cfg: HashMap<Atom, HashMap<Atom, bool>>,
    mqtt_server: ServerNode
}

impl DBToMqttMonitor{
    pub fn new(mqtt_server: ServerNode, cfg: &[u8]) -> DBToMqttMonitor{
        DBToMqttMonitor{
            cfg: HashMap::decode(&mut ReadBuffer::new(cfg, 0)),
            mqtt_server: mqtt_server
        }
    }
}

pub fn register_db_to_mqtt_monitor(mgr: &Mgr, monitor: DBToMqttMonitor){
    mgr.listen(Arc::new(monitor));
}

impl Monitor for DBToMqttMonitor{
    fn notify(&self, e: Event, _mgr: Mgr){
        //如果表中没有对应的库和表， 忽略该事件
        match self.cfg.get(&e.ware) {
            Some(tabs) => {
                match tabs.get(&e.tab){
                    Some(_) => (),
                    None => return,
                }
            },
            None => return,
        }

        //否则，将该事件投递到mqtt TODO
        match &e.other {
            &EventType::Tab{key: ref k, value: ref v} => {
                let topic = String::from(*&e.ware.as_str()) + "." + &*e.tab.as_str() + k.to_hex().as_str();
                let value = match v {
                    Some(v) => Vec::from(v.as_slice()),
                    None => {
                        let mut wb = WriteBuffer::with_capacity(1);
                        wb.write_nil();
                        wb.unwrap()
                    },
                };
                //println!("db listen-------------------------------------------{:?}", value);
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
