use std::sync::{Arc, Mutex, RwLock, MutexGuard};
use std::mem::forget;
use std::env;
use std::path::PathBuf;

use fnv::FnvHashMap;

use pi_vm::pi_vm_impl::{VMFactory};
use pi_vm::adapter::{JS, JSType};
use pi_vm::bonmgr::{NativeObjsAuth};
use atom::Atom;

use file::fs_monitor::{FSMonitorOptions, FSListener, FSMonitor, FSChangeEvent};
use pi_db::mgr::Mgr;
use gray::GrayTab;

use js_lib::JSGray;
use js_vm::{ remove_byte_code_cache, rename_byte_code_cache, compile_sync, load_module };

use js_env::env_var;
use init_js::{read_code, load_core_env};


lazy_static! {
	pub static ref GRAY_TABLE: Arc<RwLock<GrayTable>> = Arc::new(RwLock::new(GrayTable::new()));
}

pub fn get_gray_table() -> Arc<RwLock<GrayTable>> {
    GRAY_TABLE.clone()
}

pub fn register_jsgray(gray_tab: Arc<RwLock<GrayTable>>, version: Option<usize>, jsgray: JSGray) {
    let mut gray_tab = gray_tab.write().unwrap();
    let name = jsgray.name.clone();
    match version {
        Some(ver) => {
            match gray_tab.jsgrays.get_mut(ver) {
                Some(gray) => {
                    gray.insert(name, Arc::new(jsgray));
                }
                None => {
                    panic!("version not found {:?}", version);
                }
            }
        }
        None => {
            match gray_tab.jsgrays.last_mut() {
                Some(gray) => {
                    gray.insert(name, Arc::new(jsgray));
                }
                None => {
                    let mut map = FnvHashMap::default();
                    map.insert(name, Arc::new(jsgray));
                    gray_tab.jsgrays.push(map);
                }
            }
        }
    }

    // TODO: 更新 byte code cache
}

// 克隆一个版本的字节码
fn clone_byte_code_cache(version: usize) {
    let mut gray_tab = GRAY_TABLE.write().unwrap();
    let mut map = FnvHashMap::default();

    match gray_tab.byte_code_cache.get(version) {
        Some(byte_codes) => {
            for (k, v) in byte_codes.iter() {
                map.insert(k.clone(), v.clone());
            }
            gray_tab.byte_code_cache.push(map);
            gray_tab.last_version += 1;
        }
        None => {}
    }
}

fn get_byte_code_with_version(modId: String, version: usize) -> Option<Arc<Vec<u8>>> {
    let gray_tab = GRAY_TABLE.read().unwrap();
    match gray_tab.byte_code_cache.get(version) {
        Some(byte_codes) => byte_codes.get(&modId).cloned(),
        None => None
    }
}

fn remove_byte_code_with_version(modId: String, version: usize) {
    let mut gray_tab = GRAY_TABLE.write().unwrap();
    match gray_tab.byte_code_cache.get_mut(version) {
        Some(byte_codes) => {
            byte_codes.remove(&modId);
        }
        None => {}
    }
}

pub fn compile_byte_code(name: String, source_code: String, version: usize) -> Option<Arc< Vec<u8>>> {
    let opts = JS::new(1, Atom::from("compile"), Arc::new(NativeObjsAuth::new(None, None)), None).unwrap();
	match opts.compile(name.clone(), source_code) {
		Some(r) => {
            match GRAY_TABLE.write().unwrap().byte_code_cache.get_mut(version) {
                Some(byte_code) => {
                    byte_code.insert(name, Arc::new(r.clone()));
                    Some(Arc::new(r))
                }
                None => None
            }
		}
		None => None,
	}
}

pub struct GrayTable {
    // 最新版本号
    pub last_version: usize,
    // 每个灰度版本的字节码缓存
    pub byte_code_cache: Vec<FnvHashMap<String, Arc<Vec<u8>>>>,
    // 每个灰度版本的所有 jsgray
    pub jsgrays: Vec<FnvHashMap<Atom, Arc<JSGray>>>,
}

impl GrayTable {
    pub fn new() -> Self {
        GrayTable {
            last_version: 0,
            byte_code_cache: vec![],
            jsgrays: vec![],
        }
    }
}


//灰度表结构
pub struct GrayMgr {
    map: FnvHashMap<Atom, Arc<RwLock<GrayTab<JSGray>>>>,
}

impl GrayMgr {
    pub fn new() -> GrayMgr {
        //创建GrayMgr
        GrayMgr{
            map: FnvHashMap::default(),
        }
    }

    pub fn update_gray(&mut self, key: &str, mgr: &Mgr, factor: Arc<VMFactory>) -> bool{
        match self.map.get(&Atom::from(key)) {
            Some(v) => {
                let mut v = v.write().unwrap();
                let name = {
                    let last = v.get_last();
                    last.name.clone()
                };
                v.add(JSGray::new(&mgr, factor, name.as_str()));
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

    pub fn get_all_vmf_names(&self) -> Vec<Atom> {
        self.map.keys().cloned().collect()
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
}

pub fn graymgr_to_arc(gray_mgr: GrayMgr) -> Arc<Mutex<GrayMgr>>{
    Arc::new(Mutex::new(gray_mgr))
}

pub fn gray_table_to_arc(gray_tab: GrayTable) -> Arc<RwLock<GrayTable>> {
    Arc::new(RwLock::new(gray_tab))
}


pub fn hotfix_listen1(path: String) {
    let listener = FSListener(Arc::new(move |event: FSChangeEvent| {
        match event {
            FSChangeEvent::Create(path) => {
                // 创建新的模块，其他地方引入时会自己 require
                debug!("new file created1: {:?}", path);
            },
            FSChangeEvent::Write(path) => {
                debug!("new file created2: {:?}", path);
                let mut version: usize;
                {
                    println!("before write lock");
                    let gray_tab = GRAY_TABLE.read().unwrap();
                    version = gray_tab.last_version;
                    println!("release write lock version: {:?}", version);
                }
                clone_byte_code_cache(version);
                
                println!("last version {:?}", GRAY_TABLE.read().unwrap().last_version);
            },
            FSChangeEvent::Remove(path) => {
                debug!("new file created3: {:?}", path);
            },
            FSChangeEvent::Rename(old, new) => {
                debug!("new file created4: {:?}, {:?}", old, new);
            },
        };
    }));
    let mut monitor = FSMonitor::new(FSMonitorOptions::Dir(Atom::from(path), true, 1000), listener);
    monitor.run().expect("watch dir failed");
    forget(monitor);
}


pub fn hotfix_listen(gray_mgr: Arc<Mutex<GrayMgr>>, path: String) {
    let listener = FSListener(Arc::new(move |event: FSChangeEvent| {
        match event {
            FSChangeEvent::Create(path) => {
                // 创建新的模块，其他地方引入时会自己 require
                debug!("new file created: {:?}", path);
            },
            FSChangeEvent::Write(path) => {
                module_change(gray_mgr.clone(), path);
            },
            FSChangeEvent::Remove(path) => {
                debug!("module {:?} removed", path);
                // 移除被删除模块的字节码缓存
                let mod_id = normalize_module_id(path.to_str().unwrap());
                remove_byte_code_cache(mod_id);
                // 模块删除和模块修改的处理代码一样
                module_change(gray_mgr.clone(), path);
            },
            FSChangeEvent::Rename(old, new) => {
                // 模块重命名和模块修改的处理代码一样
                debug!("file name changed old name {:?}, new name {:?}", old, new);
                let old_mod_id = normalize_module_id(old.to_str().unwrap());
                let new_mod_id = normalize_module_id(new.to_str().unwrap());

                // 模块名字改变，缓存字节码的名字也要改变
                rename_byte_code_cache(old_mod_id, new_mod_id);
                module_change(gray_mgr.clone(), new);
            },
        };
    }));
    let mut monitor = FSMonitor::new(FSMonitorOptions::Dir(Atom::from(path), true, 1000), listener);
    monitor.run().expect("watch dir failed");
    forget(monitor);
}

fn module_change(gray_mgr: Arc<Mutex<GrayMgr>>, path: PathBuf) {
    let auth = Arc::new(NativeObjsAuth::new(None, None));
    let js = JS::new(1, Atom::from("hotfix compile"), auth.clone(), None).unwrap();
    load_core_env(&js);

    let mod_id = normalize_module_id(path.to_str().unwrap());
    let vmf_names = gray_mgr.lock().unwrap().get_all_vmf_names();

    vmf_names.iter().for_each(|vmf_name| {
        if is_depend(&js, &gray_mgr, vmf_name, &mod_id) {
            debug!("{} is depend for vmf {:?}", mod_id.clone(), vmf_name);

            let cur_exe = env::current_exe().unwrap();
            let env_code = read_code(&cur_exe.join("../env.js"));
            let core_code = read_code(&cur_exe.join("../core.js"));

            let env_code = js.compile("env.js".to_string(), env_code).unwrap();
            let core_code = js.compile("core.js".to_string(), core_code).unwrap();

            let mgr = gray_mgr.lock().unwrap().get_gray_tab(vmf_name).unwrap().read().unwrap().get_last().mgr.clone();
            let auth = Arc::new(NativeObjsAuth::new(None, None));
            let mut vmf = VMFactory::new(vmf_name, 128, 2, 33554432, 33554432, auth);

            // env.js / core.js 代码
            vmf = vmf.append(Arc::new(env_code));
            vmf = vmf.append(Arc::new(core_code));

            let rpc_boot_code = "pi_pt/net/rpc_entrance.js";

            // 移除当前vmf的代码缓存，否则 Module.require 还会使用原来的代码
            remove_byte_code_cache(vmf_name.clone().to_string());

            let extra_code = format!("Module.require(\'{}\', '');", rpc_boot_code);
            let extra_code = extra_code + format!("Module.require(\'{}\', '');", vmf_name.clone().to_string()).as_str();
            let extra_code = js.compile("rpc_entrance".to_string(), extra_code).unwrap();

            // rpc 功能依赖的代码，和实际处理rpc需要的代码
            vmf = vmf.append(Arc::new(extra_code));
            vmf.produce(2);

            // 更新灰度
            if gray_mgr.lock().unwrap().update_gray(vmf_name, &mgr, Arc::new(vmf)) {
                debug!("update gray for {:?} success", vmf_name);
            } else {
                error!("update gray for {:?} failed", vmf_name);
            }
        } else {
            debug!("{} is not depend for vmf {:?}, use previous code", mod_id.clone(), vmf_name);
            // 没有改变的直接克隆
            let mgr = gray_mgr.lock().unwrap().get_gray_tab(vmf_name).unwrap().read().unwrap().get_last().mgr.clone();
            let clone_vmf = gray_mgr.lock().unwrap().get_gray_tab(vmf_name).unwrap().read().unwrap().get_last().factory.clone();
            gray_mgr.lock().unwrap().update_gray(&clone_vmf.name(), &mgr, clone_vmf);
        }
    });
}

fn is_depend(js: &Arc<JS>, gray_mgr: &Arc<Mutex<GrayMgr>>, vmf_name: &str, mod_id: &str) -> bool {
    match gray_mgr.lock().unwrap().get_gray_tab(vmf_name) {
        Some(_gray) => {
            let cur_dir = env_var("PROJECT_ROOT").unwrap();

            if js.get_link_function("Module.require".to_string()) {
                js.new_str(vmf_name.clone().to_string()).unwrap();
                js.new_str(cur_dir).unwrap();
                js.call(2);
            } else {
                panic!("Module.require function is not exist");
            }

            js.get_js_function("isDepend".to_string());
            js.new_str(mod_id.clone().to_string());
            let ret = js.invoke(1);
            if ret.get_boolean() {
                return true;
            } else {
                return false;
            }
        }
        None => {
            println!("gray tab not found");
            return false;
        }
    }
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

fn normalize_module_id(mod_id: &str) -> String {
    mod_id.replace("\\", "/")
        .as_str()
        .trim_start_matches(&(env_var("PROJECT_ROOT").unwrap() + "/"))
        .to_string()
}


