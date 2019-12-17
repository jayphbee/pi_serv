use std::sync::{Arc, Mutex, RwLock, MutexGuard};
use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::ops::Deref;
use std::mem::forget;

use fnv::FnvHashMap;

use pi_vm::pi_vm_impl::{VMFactory};
use pi_vm::adapter::{JS, JSType};
use pi_vm::bonmgr::{ptr_jstype, NativeObjsAuth};
use atom::Atom;

use bon::{WriteBuffer, Encode};
use file::fs_monitor::{FSMonitorOptions, FSListener, FSMonitor, FSChangeEvent};
use pi_db::mgr::Mgr;
use pi_db::db::{TabKV, SResult};
use gray::GrayTab;
//use pi_base::util::now_millisecond;

use init_js::compeil_global;
use js_lib::Nobjs;
use js_lib::JSGray;

//灰度管理器
pub struct GrayMgr{
    pub last_mgr: Mgr,
    map: FnvHashMap<Atom, Arc<RwLock<GrayTab<JSGray>>>>, //所有灰度表的汇总
    pub dependent: FnvHashMap<Atom, Vec<Atom>>, //逆向的依赖表， 描述文件的被依赖关系
    nobjs: Nobjs,
}

impl GrayMgr {
    pub fn new(mgr: &Mgr, nobjs: &Nobjs) -> GrayMgr {
        //创建GrayMgr
        GrayMgr{
            last_mgr: mgr.clone(),
            map: FnvHashMap::default(),
            dependent: FnvHashMap::default(),
            nobjs: nobjs.clone()
        }
        
    }

    pub fn update_gray(&mut self, key: &str, mgr: &Mgr, factor: VMFactory) -> bool{
        match self.map.get(&Atom::from(key)) {
            Some(v) => {
                let mut v = v.write().unwrap();
                let (name, nobjs) = {
                    let last = v.get_last();
                    (last.name.clone(), last.nobjs.clone())
                };
                v.add(JSGray::new(&mgr, factor, name.as_str(), &nobjs));
                true
            },
            None => false,
        }
    }

    pub fn has_gray_tab(&self, key: &str) -> bool{
        match self.map.get(&Atom::from(key)) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_gray_tab(&self, key: &str) -> Option<Arc<RwLock<GrayTab<JSGray>>>> {
        match self.map.get(&Atom::from(key)) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    pub fn add_gray_tab(&mut self, gray_tab: &Arc<RwLock<GrayTab<JSGray>>>) -> Result<(), String>{
        let gray_tab1 = gray_tab.read().unwrap();
        let js_gray_name = gray_tab1.get_last().name.clone();
        match self.map.get(&js_gray_name) {
            Some(_) => Err(format!("gray_tab is exist, name:{}", js_gray_name.as_str())),
            None => {
                self.map.insert(js_gray_name, gray_tab.clone());
                Ok(())
            }
        }
    }

    pub fn remove_gray(&self, _version: usize) -> bool {
        true
    }

    //移除被依赖关系， 如果dst为None，表示移除src的所有被依赖关系
    pub fn remove_dependent(&mut self, src: &Atom, dst: Option<Atom>){
        match dst {
            Some(v) => {
                match self.dependent.get_mut(src) {
                    Some(list) => {
                        list.remove_item(&v);
                    },
                    None => (),
                };
            },
            None => {self.dependent.remove(src);},
        };
    }

    //添加被依赖关系
    pub fn add_dependent(&mut self, src: &Atom, dst: Atom){
        let r = self.dependent.entry(src.clone()).or_insert(Vec::new());
        r.push(dst);
    }

    pub fn set_obj(&mut self, key: String, obj: &JSType, path: String, name: String, js: &Arc<JS>) -> Result<bool, String>{
        self.nobjs.set_obj(key, obj, path, name, js)
    }
}

//
pub fn graymgr_to_arc(gray_mgr: GrayMgr) -> Arc<Mutex<GrayMgr>>{
    Arc::new(Mutex::new(gray_mgr))
}

//监听depned的变化， 根据depend文件的变化修改灰度
pub fn hotfix_listen(gray_mgr: Arc<Mutex<GrayMgr>>, path: String) {
    let list_u = {
        let mut arr = Vec::new();
        let gray_mgr_lock = gray_mgr.lock().unwrap();
        for (k, _) in &gray_mgr_lock.dependent{
            if k.as_str().ends_with(".u.js"){
                arr.push(String::from(k.as_str()));
            }
        }
        arr
    };
    //println!("list_u---------------------------------{:?}", list_u);
    let listener = FSListener(Arc::new(move |event: FSChangeEvent| {
        match event {
            FSChangeEvent::Create(path) => (),
            FSChangeEvent::Write(path) => (),
            FSChangeEvent::Remove(_) => (), //删除depend什么也不做
            FSChangeEvent::Rename(_, _) => (), //重命名depend什么也不做
        };
    }));
    let mut monitor = FSMonitor::new(FSMonitorOptions::File(Atom::from(path), 1000), listener);
    monitor.run().expect("");
    forget(monitor);
}

fn read_code(mgr: &Mgr, files: &Vec<String>) -> HashMap<Atom, Arc<Vec<u8>>>{
    let ware = Atom::from("memory");
    let tab = Atom::from("_$code");
    let mut file_map = HashMap::new();
    let mut arr = Vec::new();
    for v in files{
        let mut bb = WriteBuffer::new();
        v.encode(&mut bb);
        arr.push(TabKV{ware: ware.clone(), tab: tab.clone(), key: Arc::new(bb.unwrap()), value: None, index: 0});
    }
    let tr = mgr.transaction(false);
    let r = tr.query(arr, None, false, Arc::new(|_r: SResult<Vec<TabKV>>|{})).unwrap();
    match r {
        Ok(r) => {
            let mut i = 0;
            for v in r.into_iter(){
                file_map.insert(Atom::from(files[i].as_str()),v.value.unwrap());
                i += 1;
            }
        },
        Err(s) => {panic!(s);},
    }
    file_map
}

#[derive(Clone, Debug)]
pub enum FileEvent{
    Create,
    Modify,
    Remove,
}

pub struct GrayMgrMutax(usize);

impl GrayMgrMutax {
    pub fn new (mg: MutexGuard<GrayMgr>) -> GrayMgrMutax {
        GrayMgrMutax(Box::into_raw(Box::new(mg)) as usize)
    }
}

impl Drop for GrayMgrMutax {
    fn drop(&mut self){
        unsafe {Box::from_raw(self.0 as *mut MutexGuard<GrayMgr>)};
    }
}


