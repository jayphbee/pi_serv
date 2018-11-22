use std::sync::{Arc, Mutex};
use std::ops::Deref;
use std::boxed::FnBox;
use std::sync::atomic::{AtomicUsize};

use rand::rngs::OsRng;
use rand::RngCore;

use pi_vm::pi_vm_impl::{VMFactory, register_async_request};
use pi_vm::adapter::{JSType, JS};
use pi_vm::bonmgr::{BON_MGR};
use pi_lib::atom::Atom;
use pi_lib::sinfo::StructInfo;
use pi_lib::bon::{ReadBuffer, Decode};
use pi_lib::timer::{TIMER, FuncRuner};

use js_async::AsyncRequestHandler;
use depend::{Depend};
use init_js::push_pre;
use pi_lib::bon::ReadBonErr;

lazy_static! {
	pub static ref IS_END: Arc<Mutex<(bool,bool)>> = Arc::new(Mutex::new((false, false)));
}

//创建一个Arc<StructInfo>
pub fn create_sinfo(data: &[u8]) -> Result<Arc<StructInfo>, ReadBonErr>{
	let mut buf = ReadBuffer::new(data, 0);
	Ok(Arc::new(StructInfo::decode(&mut buf)?))
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
    let d = dp.depend(Vec::from(path));
    let mut arr = Vec::new();
    let mut arr1 = Vec::new();
    for path in d.into_iter(){
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
	TIMER.set_timeout(FuncRuner::new(f), ms);
}

pub struct AtomIndex(Arc<AtomicUsize>);
pub fn set_timeout(ms: u32, f: Box<FnBox()>) -> AtomIndex{
	AtomIndex(TIMER.set_timeout(FuncRuner::new(f), ms))
}

pub fn clear_timeout(index: AtomIndex){
	TIMER.cancel(&index.0);
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