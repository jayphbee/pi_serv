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

use js_lib::Nobjs;
use js_lib::JSGray;
use js_vm::{ remove_byte_code_cache, rename_byte_code_cache, compile_sync, load_module };

use js_env::env_var;
use init_js::{read_code, load_core_env};

//灰度管理器
pub struct GrayMgr{
    map: FnvHashMap<Atom, Arc<RwLock<GrayTab<JSGray>>>>, //所有灰度表的汇总
    nobjs: Nobjs,
}

impl GrayMgr {
    pub fn new(mgr: &Mgr, nobjs: &Nobjs) -> GrayMgr {
        //创建GrayMgr
        GrayMgr{
            map: FnvHashMap::default(),
            nobjs: nobjs.clone()
        }
    }

    pub fn update_gray(&mut self, key: &str, mgr: &Mgr, factor: Arc<VMFactory>) -> bool{
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

    pub fn set_obj(&mut self, key: String, obj: &JSType, path: String, name: String, js: &Arc<JS>) -> Result<bool, String>{
        self.nobjs.set_obj(key, obj, path, name, js)
    }
}

pub fn graymgr_to_arc(gray_mgr: GrayMgr) -> Arc<Mutex<GrayMgr>>{
    Arc::new(Mutex::new(gray_mgr))
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


