use std::sync::{Arc, Mutex, RwLock, MutexGuard};
use std::rc::{Rc};
use std::cell::RefCell;
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
use pi_lib::atom::Atom;

use pi_lib::bon::{WriteBuffer, Encode};
use pi_base::fs_monitor::{FSMonitorOptions, FSListener, FSMonitor, FSChangeEvent};
use pi_db::mgr::Mgr;
use pi_db::db::{TabKV, SResult};
use pi_lib::gray::GrayTab;

use depend::{Depend, FileDes};
use init_js::{push_pre, compeil_global};
use util::{read_file_list, read_depend, read_file_str};
use js_lib::Nobjs;
use jsloader::Loader;
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
        let depend = read_depend(&mgr); //从mgr中读到旧的文件列表
        //println!("depend len---------------------------{}", depend.len());
        //创建逆向的依赖表
        let mut dependent = FnvHashMap::default();
        for (name, v) in depend{
            dependent.entry(name.clone()).or_insert_with(||{
                Vec::new()
            });
            match v.depend {
                Some(dp) => {
                    let dp = match dp.get("js") {
                        Some(v) => v,
                        None => {continue;},
                    };
                    for v in dp{
                        let elem = dependent.entry(Atom::from(v.as_str())).or_insert_with(||{
                            Vec::new()
                        });
                        elem.push(name.clone());
                    }
                },
                None => (),
            }
        }

        //创建GrayMgr
        GrayMgr{
            last_mgr: mgr.clone(),
            map: FnvHashMap::default(),
            dependent: dependent,
            nobjs: nobjs.clone()
        }
    }

    pub fn update_gray(&mut self, key: &str, mgr: &Mgr, factor: VMFactory) -> bool{
        //println!("update_gray----------------------------------------{}", key);
        match self.map.get(&Atom::from(key)) {
            Some(v) => {
                //println!("update_gray some----------------------------------------");
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

    pub fn remove_gray(&self, version: usize) -> bool {
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
            FSChangeEvent::Create(path) => depend_change(gray_mgr.clone(), path, &list_u),
            FSChangeEvent::Write(path) => depend_change(gray_mgr.clone(), path, &list_u),
            FSChangeEvent::Remove(_) => (), //删除depend什么也不做
            FSChangeEvent::Rename(_, _) => (), //重命名depend什么也不做
        };
    }));
    let mut monitor = FSMonitor::new(FSMonitorOptions::File(Atom::from(path), 1000), listener);
    monitor.run().expect("");
    forget(monitor);
}

fn depend_change(gray_mgr: Arc<Mutex<GrayMgr>>, path: PathBuf, list_u: &[String]) {
    let mut gray_mgr_lock = gray_mgr.lock().unwrap();
    let old_mgr = gray_mgr_lock.last_mgr.clone();
    let old_list = read_depend(&gray_mgr_lock.last_mgr); //从mgr中读到旧的文件列表
    let new_list = read_file_list(&path); //读新的文件列表
    let mut old_list1 = Vec::new();
    //println!("len----------------------------------{}, {}", old_list.len(), new_list.len());
    let mut diff_list = diff(old_list, &new_list, &mut old_list1);
    //println!("diff_list----------------------------------{:?}", diff_list);

    let old_depend = Depend::new_sample(old_list1);
    let new_depend = Depend::new_sample(new_list);
    let new_mgr = write_depend_diff(&diff_list, &gray_mgr_lock.last_mgr, path, &new_depend); //将depend差异写入数据库，得到新的mgr

    //计算变化的文件（包括真正变化的文件以及依赖它们的文件）
    let diff_list_all = diff1(&mut diff_list, &gray_mgr_lock);
    //println!("diff_list_all----------------------------------{:?}", diff_list_all);

    //更新灰度管理器上的mgr和反向依赖表
    modify_gray_mgr(&mut gray_mgr_lock, &new_mgr, &diff_list, &new_depend, &old_depend);

    let mut list_c = Vec::new();
    for (f,_) in diff_list{
        if f.ends_with(".c.js"){
            list_c.push(String::from(f.as_str()));
        }
    }
    list_c.extend_from_slice(list_u);
    start_vm(list_c, diff_list_all, &new_mgr, &old_mgr,new_depend, gray_mgr_lock);
}

fn start_vm(mut list_c: Vec<String>, diff_list_all: HashMap<Atom, FileEvent>, new_mgr: &Mgr, old_mgr: &Mgr, depend: Depend, gray_mgr: MutexGuard<GrayMgr>) {
    let js = JS::new(0x100, Arc::new(NativeObjsAuth::new(None, None))).unwrap();
    let global_code = compeil_global(&js);//插入全局变量定义函数的字节码
    js.load(&global_code);//加载全局变量定义函数的字节码
    let nobjs = gray_mgr.nobjs.clone();
    let mut list = nobjs.get_depend();
    let nobjs_len = Loader::list_with_depend(&list, &depend).len();
    list.extend_from_slice(&list_c);
    push_pre(&mut list);
    let list = Loader::list_with_depend(&list, &depend);
    

    let file_map = read_code(new_mgr, &list);
    //"evn.js", "core.js", "first.js", "next.js", nobjs的依赖
    for i in 0..4 + nobjs_len{
        let path = String::from(list[i].borrow().path.as_ref());
        //println!("path-------------------------------------{}", path);
        let u8arr = file_map.get(&Atom::from(path.clone())).unwrap().as_slice(); 
        js.load(u8arr);
    }
    let u8arr = file_map.get(&Atom::from("last.js")).unwrap().as_slice(); 
    js.load(u8arr);
    loop{
        if js.is_ran(){
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    {
        let gray_mgr_ptr = gray_mgr.deref() as *const GrayMgr as usize;
        let nobjs = gray_mgr.nobjs.clone();
        let gray_mutax = GrayMgrMutax::new(gray_mgr);
        let ptr = Box::into_raw(Box::new(gray_mutax)) as usize;
        ptr_jstype(js.get_objs(), js.clone(), ptr, 646865374); //将灰度管理器的锁设置到虚拟机中， 只有虚拟机销毁时，锁才会释放

        //调用全局变量定义函数， 定义全局变量_$mgr
        js.get_js_function("_$defineGlobal".to_string());
        js.new_str(String::from("_$new_mgr"));
        let ptr = Box::into_raw(Box::new(new_mgr.clone())) as usize;
        ptr_jstype(js.get_objs(), js.clone(), ptr, 2976191628); //new native obj作为参数
        js.call(2);

        //调用全局变量定义函数， 定义全局变量_$mgr
        js.get_js_function("_$defineGlobal".to_string());
        js.new_str(String::from("_$old_mgr"));
        let ptr = Box::into_raw(Box::new(old_mgr.clone())) as usize;
        ptr_jstype(js.get_objs(), js.clone(), ptr, 2976191628); //new native obj作为参数
        js.call(2);

        //调用全局变量定义函数， 定义全局变量_$mgr
        js.get_js_function("_$defineGlobal".to_string());
        js.new_str(String::from("_$depend"));
        let ptr = Box::into_raw(Box::new(depend)) as usize;
        ptr_jstype(js.get_objs(), js.clone(), ptr, 1797798710); //new native obj作为参数
        js.call(2);

        //调用全局变量定义函数， 定义全局变量_$mgr
        js.get_js_function("_$defineGlobal".to_string());
        js.new_str(String::from("_$nobjs"));
        nobjs.to_map(&js);
        js.call(2);

        //调用全局变量定义函数， 定义全局变量_$mgr
        js.get_js_function("_$defineGlobal".to_string());
        js.new_str(String::from("_$gray_mgr"));
        ptr_jstype(js.get_objs_ref(), js.clone(), gray_mgr_ptr, 3355816649); //new native obj作为参数
        js.call(2);

        //调用全局变量定义函数， 定义全局变量_$mgr
        js.get_js_function("_$defineGlobal".to_string());
        js.new_str(String::from("_$diff"));
        js.get_type("Map".to_string());
        let temp = js.new_array();
        let mut i = 0;
        for (key, value) in diff_list_all{
            let mut arr =  js.new_array();
            js.set_index(&arr, 0, &mut js.new_str(key.as_str().to_string()));
            let mut v = match value {
                FileEvent::Create => js.new_u8(0),
                FileEvent::Modify => js.new_u8(1),
                FileEvent::Remove => js.new_u8(2),
            };
            js.set_index(&arr, 1, &mut v);
            js.set_index(&temp, i, &mut arr);
            i += 1;
        }
        js.new_type("Map".to_string(), 1);
        js.call(2);
    }
    for i in 2..list.len(){
        let des = &list[i];
        let path = String::from(des.borrow().path.as_ref());
        //println!("des:{}", &path);
        if path.ends_with(".js"){
            let u8arr = file_map.get(&Atom::from(path.clone())).unwrap().as_slice();
            js.load(u8arr);
            loop{
                if js.is_ran(){
                    break;
                }
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}

fn diff(mut old_list: HashMap<Atom, FileDes>, new_list: &Vec<FileDes>, old_list1: &mut Vec<FileDes>) -> HashMap<Atom, FileEvent> {
    let mut diff = HashMap::new(); //文件差异
    //遍历旧的和新的文件列表，如果新旧两个列表中都存在该文件并且签名不同，表示该文件被修改， 如果新的的列表中存在但旧的列表中不存在， 表示该文件为新增文件（从旧的列表中删除比较过的条目， 剩余条目则为删除的文件）
    for n in new_list{
        let key = Atom::from(n.path.as_str());
        match old_list.remove(&key){
            Some(o) => {
                if o.sign != n.sign{
                    diff.insert(key.clone(), FileEvent::Modify);
                }
                old_list1.push(o);
            },
            None => {diff.insert(key.clone(), FileEvent::Create);},
        };
    }
    //遍历剩余的旧的文件列表，设置为删除状态
    for o in old_list{
        diff.insert(o.0.clone(), FileEvent::Remove);
        old_list1.push(o.1);
    }
    diff
}

fn diff1(diff: &mut HashMap<Atom, FileEvent>, gray_mgr: &GrayMgr) -> HashMap<Atom, FileEvent>{
    let mut map = HashMap::new();
    let mut ds = Vec::new(); //文件差异
    //遍历旧的和新的文件列表，如果新旧两个列表中都存在该文件并且签名不同，表示该文件被修改， 如果新的的列表中存在但旧的列表中不存在， 表示该文件为新增文件（从旧的列表中删除比较过的条目， 剩余条目则为删除的文件）
    for (k, e) in diff{
        ds.push(k.clone());
        map.insert(k.clone(), e.clone());
    }
    diff2(&mut map, &ds, gray_mgr);
    map
}

fn diff2(diff: &mut HashMap<Atom, FileEvent>, files: &Vec<Atom>, gray_mgr: &GrayMgr){
        for d in files{
            diff.entry(d.clone()).or_insert(FileEvent::Modify);
            //如果有文件依赖该文件， 递归这些文件的被依赖文件， 尝试将其列入diff中
            match gray_mgr.dependent.get(d){
                Some(v) => {diff2(diff, &v, gray_mgr);},
                None => (),
            }
        }
    }

//写数据库的差异
fn write_depend_diff(diff: &HashMap<Atom, FileEvent>, mgr: &Mgr, path: PathBuf, depend: &Depend) -> Mgr{
    let mgr = mgr.clone();
    //遍历差异列表，修改代码差异和depend差异
    let mut items = Vec::new();
    let ware = Atom::from("memory");
    let depend_tab = Atom::from("_$depend");
    let code_tab = Atom::from("_$code");
    let js = JS::new(0x100, Arc::new(NativeObjsAuth::new(None, None))).unwrap();
    for (mod_path, d) in diff{
        let mut key_bb = WriteBuffer::new();
        mod_path.encode(&mut key_bb);
        let key = Arc::new(key_bb.unwrap());

        match d {
            FileEvent::Create | FileEvent::Modify => {
                let f = depend.get(mod_path).unwrap().borrow();
                let mut bb = WriteBuffer::new();
                f.encode(&mut bb);
                items.push(TabKV{
                    ware: ware.clone(),
                    tab: depend_tab.clone(),
                    key: key.clone(),
                    value:  Some(Arc::new(bb.unwrap())),
                    index: 0,
                });
                //如果是js文件， 读取文件并编译
                if mod_path.ends_with(".js") {

                    let code = match js.compile(String::from(mod_path.as_str()), read_file_str(&path.as_path().join(String::from("../") + mod_path.as_str()))) {
                        Some(v) => Arc::new(v),
                        None => {println!("warn!!! compile fail, path:{}", mod_path.as_str()); continue},
                    };
                    items.push(TabKV{
                        ware: ware.clone(),
                        tab: code_tab.clone(),
                        key: key.clone(),
                        value: Some(code),
                        index: 0,
                    });
                }
            },
            FileEvent::Remove => {
                items.push(TabKV{
                    ware: ware.clone(),
                    tab: depend_tab.clone(),
                    key: key.clone(),
                    value: None,
                    index: 0,
                });
                if mod_path.ends_with(".js") {
                    items.push(TabKV{
                        ware: ware.clone(),
                        tab: code_tab.clone(),
                        key: key.clone(),
                        value: None,
                        index: 0,
                    });
                }
            }
        };
    };

    let tr = mgr.transaction(true);
    tr.modify(items, None, false, Arc::new(|_|{}));
    tr.prepare(Arc::new(|_|{}));
    tr.commit(Arc::new(|_|{}));
    mgr
}

//修改灰度管理器， 
fn modify_gray_mgr(gray_mgr: &mut GrayMgr, mgr: &Mgr, diff: &HashMap<Atom, FileEvent>, new_depend: &Depend, old_depend: &Depend) {
    gray_mgr.last_mgr = mgr.clone();
    for (dst, v) in diff{
        match v {
            FileEvent::Create => {
                match new_depend.get(&dst).unwrap().borrow().depend{
                    Some(ref v) => {
                        for (k, _) in v{
                            gray_mgr.add_dependent(&Atom::from(k.as_str()), dst.clone());
                        }
                    },
                    None => (),
                };
            },
            FileEvent::Modify => {
                let old_d = match old_depend.get(&dst).unwrap().borrow().depend{
                    Some(ref v) => {
                        let mut r = HashMap::new();
                        for (k, _) in v{
                            r.insert(Atom::from(k.as_str()), ());
                        }
                        r
                    },
                    None => HashMap::new(),
                };
                let mut new_d = match new_depend.get(&dst).unwrap().borrow().depend{
                    Some(ref v) => {
                        let mut r = HashMap::new();
                        for (k, _) in v{
                            r.insert(Atom::from(k.as_str()), ());
                        }
                        r
                    },
                    None => HashMap::new(),
                };

                for (k, _) in old_d {
                    let k = Atom::from(k);
                    match new_d.remove(& k) {
                        Some(_) => (),
                        None => gray_mgr.remove_dependent(&k, Some(dst.clone())),
                    }
                }
                
                for (v, _) in new_d {
                    gray_mgr.add_dependent(&v, dst.clone());
                }
            }
            FileEvent::Remove => {
                 match old_depend.get(&dst).unwrap().borrow().depend{
                    Some(ref v) => {
                        for (k, _) in v {
                            gray_mgr.remove_dependent(&Atom::from(k.as_str()), Some(dst.clone()));
                        }
                    },
                    None => (),
                };
                
            }
        }
    }
}

fn read_code(mgr: &Mgr, files: &Vec<Rc<RefCell<FileDes>>>) -> HashMap<Atom, Arc<Vec<u8>>>{
    let ware = Atom::from("memory");
    let tab = Atom::from("_$code");
    let mut file_map = HashMap::new();
    let mut arr = Vec::new();
    for v in files{
        let mut bb = WriteBuffer::new();
        v.borrow().path.encode(&mut bb);
        arr.push(TabKV{ware: ware.clone(), tab: tab.clone(), key: Arc::new(bb.unwrap()), value: None, index: 0});
    }
    let tr = mgr.transaction(false);
    let r = tr.query(arr, None, false, Arc::new(|r: SResult<Vec<TabKV>>|{})).unwrap();
    match r {
        Ok(r) => {
            let mut i = 0;
            for v in r.into_iter(){
                file_map.insert(Atom::from(files[i].borrow().path.as_str()),v.value.unwrap());
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


