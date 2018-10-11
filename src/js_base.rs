use std::sync::{Arc, Mutex};
use std::ops::Deref;
use std::boxed::FnBox;
use std::sync::atomic::{AtomicIsize};
use std::collections::HashMap;
use std::fs::{read, File};
use std::path::PathBuf;
use std::io::Read;

use rand::rngs::OsRng;
use rand::RngCore;

use pi_vm::pi_vm_impl::{VMFactory, register_async_request};
use pi_vm::adapter::{JSType, JS};
use pi_vm::bonmgr::{BON_MGR, ptr_jstype, NativeObjsAuth};
use pi_lib::atom::Atom;
use pi_lib::sinfo::StructInfo;
use pi_lib::bon::{ReadBuffer, Decode, WriteBuffer, Encode};
use pi_base::timer::TIMER;
use pi_base::fs_monitor::{FSMonitorOptions, FSListener, FSMonitor, FSChangeEvent};
use pi_db::mgr::Mgr;
use pi_db::db::{TabKV};

use js_async::AsyncRequestHandler;
use depend::{Depend, FileDes};
use init_js::push_pre;
use util::{read_file_list, read_depend, read_file_str};
use js_lib::Nobjs;

lazy_static! {
	pub static ref IS_END: Arc<Mutex<(bool,bool)>> = Arc::new(Mutex::new((false, false)));
}

//创建一个Arc<StructInfo>
pub fn create_sinfo(data: &[u8]) -> Arc<StructInfo>{
	let mut buf = ReadBuffer::new(data, 0);
	Arc::new(StructInfo::decode(&mut buf))
}
//clone vm工厂（VMFactory没有显示实现clone方法， 无法导出， 需要封装）
pub fn clone_vm_factory(factory: &VMFactory) -> VMFactory{
    factory.clone()
}

pub fn arc_new_async_request_handler(arh: AsyncRequestHandler) -> Arc<AsyncRequestHandler> {
    Arc::new(arh)
}

//为async注册handler
pub fn register_async_handler(topic: String, handler: &Arc<AsyncRequestHandler>){
    register_async_request(Atom::from(topic), handler.clone());
}

//new一个arc
pub fn arc_new<T>(v: T) -> Arc<T>{
    Arc::new(v)
}

//new一个arc
pub fn arc_deref< T>(v: &Arc<T>) -> &T{
    v.deref()
}

//new一个box
pub fn box_new<T>(v: T) -> Box<T>{
    Box::new(v)
}

//getdepend
pub fn get_depend(dp: &Depend, path: &[String]) -> Vec<String> {
    let d = dp.depend(path);
    let mut arr = Vec::new();
    let mut arr1 = Vec::new();
    for v in d.into_iter(){
        let path = v.borrow().path.clone();
        if path.ends_with(".s.js"){
            arr.push(path);
        }else {
            arr1.push(path);
        }
    }
    arr.extend_from_slice(arr1.as_slice());
    push_pre(&mut arr);
    arr
}

//休眠
pub fn sleep(ms: u32, f: Box<FnBox()>){
	TIMER.set_timeout(f, ms);
}

pub struct AtomIndex(Arc<AtomicIsize>);
pub fn set_timeout(ms: u32, f: Box<FnBox()>) -> AtomIndex{
	AtomIndex(TIMER.set_timeout(f, ms))
}

pub fn clear_timeout(index: AtomIndex){
	TIMER.cancel(index.0);
}

pub struct Rand(OsRng);

//创建一个随机对象
pub fn create_rand() -> Rand{
	Rand(OsRng::new().expect("create_osrng fail"))
}

//取到一个随机值
pub fn next_u32(or: &mut Rand) -> u32{
	or.0.next_u32()
}

//取到一个随机值
pub fn next_u64(or: &mut Rand) -> u64{
	or.0.next_u64()
}

//取到一个随机值
pub fn fill_bytes(or: &mut Rand, len: usize) -> Vec<u8>{
    let mut arr = Vec::with_capacity(len);
    unsafe{arr.set_len(len);};
	or.0.fill_bytes(arr.as_mut_slice());
    arr
}

//取到一个随机值
pub fn try_fill_bytes(or: &mut Rand, len: usize) -> Result<Vec<u8>, String> {
    let mut arr = Vec::new();
    unsafe{arr.set_len(len);};
	match or.0.try_fill_bytes(arr.as_mut_slice()) {
        Ok(_) => Ok(arr),
        Err(e) => Err(String::from(e.msg)),
    }
}

//销毁nativeobject
pub fn drop_native_obj(t: &JSType, js: &Arc<JS>) -> Result<bool, String> {
    if !t.is_native_object(){
        return Err(String::from("drop_native_obj err, param is not NativeObject!"))
    }

    let ptr = t.get_native_object();
    let objs = js.get_objs();
    let mut objs = objs.borrow_mut();
    let struct_metas = BON_MGR.struct_metas.lock().unwrap();
    match objs.remove(&ptr){
        Some(v) => {
            let meta = struct_metas.get(&v.meta_hash).unwrap();
            //println!("drop_native_obj---------------------------------------------{}", meta.name);
            (meta.drop_fn)(ptr);
            Ok(true)
        },
        None => {
            //println!("drop_native_obj fail---------------------------------------------");
            Ok(false)
        }
    }
}

pub fn end(js: &Arc<JS>) {
    IS_END.lock().unwrap().0 = true;
    let b = js.get_objs();
    let b = b.borrow();
    println!("end--------------------------------------------------{}, native_obj_count:{}", IS_END.lock().unwrap().0, b.len());
}

// pub struct FileChangeHandler{
//     mgr: Mgr,
//     factory: Arc<VMFactory>,
//     nobjs: Nobjs,
//     handler_name: Atom,
// }

// impl FileChangeHandler {
//     fn new(handler_name:String, mgr: &Mgr, factory: Arc<VMFactory>, nobjs: &Nobjs) -> FileChangeHandler {
//         FileChangeHandler{
//             handler_name: Atom::from(handler_name),
//             nobjs: nobjs.clone(),
//             mgr: mgr.clone(),
//             factory: factory,
//         }
//     }
// }

// pub fn listen_depend(handler: FileChangeHandler,  path: String) {
    
//     let handler = Arc::new(Mutex::new(handler));
//     let listener = FSListener(Arc::new(move |event: FSChangeEvent| {
//         match event {
//             FSChangeEvent::Create(path) => depend_change(handler.clone(), path),
//             FSChangeEvent::Write(path) => depend_change(handler.clone(), path),
//             FSChangeEvent::Remove(_) => (), //删除depend什么也不做
//             FSChangeEvent::Rename(_, _) => (), //重命名depend什么也不做
//         };
//     }));
//     let mut monitor = FSMonitor::new(FSMonitorOptions::File(Atom::from(path), 1000), listener);
//     monitor.run().expect("");
// }

// fn depend_change(handler: Arc<Mutex<FileChangeHandler>>, path: PathBuf) {
//     let handler = handler.lock().unwrap();
//     let nobjs = handler.nobjs.clone();
//     let mut old = read_depend(&handler.mgr);
//     let mut diff = HashMap::new();
//     let file_list = read_file_list(&path);
//     for n in file_list.into_iter(){
//         match old.remove(&n.path){
//             Some(o) => {
//                 if o.sign != n.sign{
//                     diff.insert(n.path.clone(), FileEvent::Modify(o));
//                 }
//             },
//             None => {diff.insert(n.path.clone(), FileEvent::Modify(n));},
//         };
//     }

//     //遍历剩余的文件，设置为删除状态
//     for o in old{
//         diff.insert(o.0.clone(), FileEvent::Remove);
//     }

//     let mgr = write_depend_diff(&diff, &handler.mgr, path); //将depend差异写入数据库，得到新的mgr

//     let real_args = Box::new(move |vm: Arc<JS>| -> usize {
//         //事件对象
//         let event = vm.new_object();
//         vm.set_field(&event, String::from("event_name"), &mut vm.new_str("depend_change".to_string()));
//         vm.new_array(); //map 的第一层数组
//         vm.get_type("Map".to_string());
//         let temp = vm.new_array();
//         for (path, d) in diff{
//             match d {
//                 FileEvent::Modify(_) => {
//                     vm.set_index(&temp, 0, &mut vm.new_str(path));
//                     vm.set_index(&temp, 1, &mut vm.new_u8(1));
//                 },
//                 FileEvent::Remove => {
//                     vm.set_index(&temp, 0, &mut vm.new_str(path));
//                     vm.set_index(&temp, 1, &mut vm.new_u8(2));
//                 },
//                 _ => (),
//             }
//         }
//         vm.set_field(&event, String::from("diff"), &mut vm.new_type("Map".to_string(), 1));//必须保证“Map”类型存在
        
//         //mgr
//         ptr_jstype(vm.get_objs(), vm.clone(), Box::into_raw(Box::new(mgr.clone())) as usize, 2976191628);
//         //nobjs
//         nobjs.to_json(&vm);
//         3
//     });
//     handler.factory.call(0, handler.handler_name.clone(), real_args, Atom::from("depend_change task"));
// }

// //写数据库的差异
// fn write_depend_diff(diff: &HashMap<String, FileEvent>, mgr: &Mgr, path: PathBuf) -> Mgr{
//     let mgr = mgr.clone();
//     //遍历差异列表，修改代码差异和depend差异
//     let mut items = Vec::new();
//     let ware = Atom::from("memory");
//     let depend_tab = Atom::from("_$depend");
//     let code_tab = Atom::from("_$code");
//     let js = JS::new(0x100, Arc::new(NativeObjsAuth::new(None, None))).unwrap();
//     for (mod_path, d) in diff{
//         let mut key_bb = WriteBuffer::new();
//         mod_path.encode(&mut key_bb);
//         let key = Arc::new(key_bb.unwrap());

//         match d {
//             FileEvent::Modify(f) => {
//                 let mut bb = WriteBuffer::new();
//                 f.encode(&mut bb);
//                 items.push(TabKV{
//                     ware: ware.clone(),
//                     tab: depend_tab.clone(),
//                     key: key.clone(),
//                     value:  Some(Arc::new(bb.unwrap())),
//                     index: 0,
//                 });
//                 //如果是js文件， 读取文件并编译
//                 if mod_path.ends_with(".js") {
//                     let code = match js.compile(mod_path.clone(), read_file_str(&path.as_path().join(mod_path))) {
//                         Some(v) => v,
//                         None => {println!("warn!!! compile fail, path:{}", mod_path); continue},
//                     };
//                     items.push(TabKV{
//                         ware: ware.clone(),
//                         tab: code_tab.clone(),
//                         key: key.clone(),
//                         value: Some(Arc::new(code)),
//                         index: 0,
//                     });
//                 }
//             },
//             FileEvent::Remove => {
//                 items.push(TabKV{
//                     ware: ware.clone(),
//                     tab: depend_tab.clone(),
//                     key: key.clone(),
//                     value: None,
//                     index: 0,
//                 });
//                 if mod_path.ends_with(".js") {
//                     items.push(TabKV{
//                         ware: ware.clone(),
//                         tab: code_tab.clone(),
//                         key: key.clone(),
//                         value: None,
//                         index: 0,
//                     });
//                 }
//             },
//             _ => {panic!("");},
//         };
//     };

//     let tr = mgr.transaction(true);
//     tr.modify(items, None, false, Arc::new(|_|{}));
//     tr.prepare(Arc::new(|_|{}));
//     tr.commit(Arc::new(|_|{}));
//     mgr
// }


// pub enum FileEvent{
//     Create(FileDes),
//     Modify(FileDes),
//     Remove,
//     Rename
// }