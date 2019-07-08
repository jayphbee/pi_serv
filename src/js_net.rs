use std::sync::{Arc, Mutex, RwLock};
use std::net::SocketAddr;
use std::io::{Error};

use std::time::SystemTime;
use std::sync::atomic::Ordering;

use fnv::FnvHashMap;
use mqtt3;

use pi_vm::adapter::{JS};
use pi_vm::pi_vm_impl::{new_queue, remove_queue};
use pi_vm::bonmgr::{ptr_jstype};
use handler::{Args, Handler};
use gray::{GrayVersion, GrayTab};
use atom::Atom;
// use pi_p2p::manage::P2PManage;
use rpc::traits::RPCServerTraits;
use rpc::server::RPCServer;
use net::data::{RawSocket, RawStream};
use net::tls::{TlsSocket, TlsStream, TlsConfig};
use net::{Config, Protocol};
use net::api::{Socket, Stream};
use net::api::{NetManager, TlsManager};
use net::data::ListenerFn;
use mqtt::server::{ServerNode, ClientStub};
use std::io::{Result as IOResult};
use mqtt::data::Server;
use mqtt::session::Session;
use js_lib::JSGray;
use worker::task::TaskType;
use worker::impls::cast_net_task;

pub struct NetMgr {
    pub mgr: NetManager,
    pub handler: Arc<Mutex<FnvHashMap<Atom, Vec<Box<Fn(Arc<Result<(RawSocket, Arc<RwLock<RawStream>>),Error>>,
    Arc<Result<SocketAddr,Error>>) + Send>>>>>,
    pub close_handler: Arc<Mutex<FnvHashMap<Atom, Vec<Box<Fn(usize, RawSocket) + Send>>>>>,
}

impl NetMgr {
    pub fn new() -> NetMgr{
        NetMgr{
            mgr: NetManager::new(),
            handler: Arc::new(Mutex::new(FnvHashMap::default())),
            close_handler: Arc::new(Mutex::new(FnvHashMap::default())),
        }
    }

    fn add_handler(&mut self, addr: String, protocol: String, f: Box<Fn(Arc<Result<(RawSocket, Arc<RwLock<RawStream>>),Error>>, Arc<Result<SocketAddr,Error>>) + Send>){
        let key = Atom::from(addr.clone() + ":" + protocol.as_str());
        let h = self.handler.clone();
        let mut r = self.handler.lock().unwrap();
        let c_h = self.close_handler.clone();
        let key_copy = key.clone();
        let v = r.entry(key).or_insert_with(||{
            let c_h = c_h.clone();
            {
                let mut close_handler = c_h.lock().unwrap();
                close_handler.insert(key_copy.clone(), Vec::new());
            }
            let arr = Vec::new();
            let callback: ListenerFn = Box::new(move |peer: Result<(RawSocket, Arc<RwLock<RawStream>>),Error>, addr: Result<SocketAddr,Error>|{ 
                let peer = Arc::new(peer);
                let addr = Arc::new(addr);
                let c_h = c_h.clone();

                //设置关闭链接的回调
                match peer.as_ref() {
                    &Ok(ref peer) => {
                        let socket = peer.0.clone();
                        let key_copy = key_copy.clone();
                        let stream = &peer.1;
                        stream.write().unwrap().set_close_callback(Box::new(move |id: usize, _: IOResult<()>| {
                            let c_h = c_h.clone();
                            let socket = socket.clone();
                            let close_handler = c_h.lock().unwrap();
                            let close_handler = close_handler.get(&key_copy).unwrap();
                            //通知链接关闭处理器
                            for h in close_handler.iter() {
                                h(id, socket.clone());
                            }
                        }));
                    } ,
                    Err(s) => println!("{}", s),
                };

                //链接成功， 通知链接成功处理器
                let r = h.lock().unwrap();
                let rr = r.get(&key_copy).unwrap();
                for v in rr.iter(){
                    v(peer.clone(),  addr.clone());
                }
            });
            let cfg = Config{
                protocol: match protocol.as_str() {
                    "tcp" => Protocol::TCP,
                    _ => {panic!("nonsupport protocol:{}", protocol);},
                },
                addr: addr.parse().unwrap()
            };
            self.mgr.bind(cfg, callback);
            arr
        });
        v.push(f);
    }

    fn add_close_handler(&mut self, addr: &str, protocol: &str, f: Box<Fn(usize, RawSocket) + Send>){
        let key = Atom::from(addr.to_string() + ":" + protocol);
        let mut close_handler = self.close_handler.lock().unwrap();
        let mut close_handler = close_handler.get_mut(&key).unwrap();
        close_handler.push(f);
    }
}

pub struct TlsNetMgr {
    pub mgr: TlsManager,
    pub handler: Arc<Mutex<FnvHashMap<Atom, Vec<Box<Fn(Arc<Result<(TlsSocket, Arc<RwLock<TlsStream>>),Error>>,
    Arc<Result<SocketAddr,Error>>) + Send>>>>>,
    pub close_handler: Arc<Mutex<FnvHashMap<Atom, Vec<Box<Fn(usize, TlsSocket) + Send>>>>>,
}

impl TlsNetMgr {
    pub fn new(recv_buff_size: usize) -> TlsNetMgr{
        TlsNetMgr{
            mgr: TlsManager::new(recv_buff_size),
            handler: Arc::new(Mutex::new(FnvHashMap::default())),
            close_handler: Arc::new(Mutex::new(FnvHashMap::default())),
        }
    }

    fn add_handler(&mut self, addr: String, protocol: String, cert_path:String, key_path: String, f: Box<Fn(Arc<Result<(TlsSocket, Arc<RwLock<TlsStream>>),Error>>, Arc<Result<SocketAddr,Error>>) + Send>){
        let key = Atom::from(addr.clone() + ":" + protocol.as_str());
        let h = self.handler.clone();
        let mut r = self.handler.lock().unwrap();
        let c_h = self.close_handler.clone();
        let key_copy = key.clone();
        let v = r.entry(key).or_insert_with(||{
            let c_h = c_h.clone();
            {
                let mut close_handler = c_h.lock().unwrap();
                close_handler.insert(key_copy.clone(), Vec::new());
            }
            let arr = Vec::new();
            let callback = Box::new(move |peer: Result<(TlsSocket, Arc<RwLock<TlsStream>>),Error>, addr: Result<SocketAddr,Error>|{
                let peer = Arc::new(peer);
                let addr = Arc::new(addr);
                let c_h = c_h.clone();

                //设置关闭链接的回调
                match peer.as_ref() {
                    &Ok(ref peer) => {
                        let socket = peer.0.clone();
                        let key_copy = key_copy.clone();
                        let stream = &peer.1;
                        stream.write().unwrap().set_close_callback(Box::new(move |id: usize, _: IOResult<()>| {
                            let c_h = c_h.clone();
                            let socket = socket.clone();
                            let close_handler = c_h.lock().unwrap();
                            let close_handler = close_handler.get(&key_copy).unwrap();
                            //通知链接关闭处理器
                            for h in close_handler.iter() {
                                h(id, socket.clone());
                            }
                        }));
                    } ,
                    Err(s) => println!("{}", s),
                };

                //链接成功， 通知链接成功处理器
                let r = h.lock().unwrap();
                let rr = r.get(&key_copy).unwrap();
                for v in rr.iter(){
                    v(peer.clone(),  addr.clone());
                }
            });
            let cfg = TlsConfig::new(
                match protocol.as_str() {
                    "tcp" => Protocol::TCP,
                    _ => {panic!("nonsupport protocol:{}", protocol);},
                },
                addr.parse().unwrap(),
                &cert_path,
                &key_path
            );
            self.mgr.bind(cfg, callback);
            arr
        });
        v.push(f);
    }

    fn add_close_handler(&mut self, addr: &str, protocol: &str, f: Box<Fn(usize, TlsSocket) + Send>){
        let key = Atom::from(addr.to_string() + ":" + protocol);
        let mut close_handler = self.close_handler.lock().unwrap();
        let mut close_handler = close_handler.get_mut(&key).unwrap();
        close_handler.push(f);
    }
}

/*
* 网络连接Handler
*/
#[derive(Clone)]
pub struct NetHandler {
    handler: Atom, //处理函数名称（js函数）
    gray_tab: Arc<RwLock<GrayTab<JSGray>>>, //灰度表
}

unsafe impl Send for NetHandler {}
unsafe impl Sync for NetHandler {}

impl Handler for NetHandler {
	type A = usize; //连接id
    type B = ();
    type C = ();
    type D = ();
    type E = ();
    type F = ();
    type G = ();
    type H = ();
	type HandleResult = Result<(), String>;

	fn handle(&self, env: Arc<dyn GrayVersion>, event_name: Atom, args: Args<Self::A, Self::B, Self::C, Self::D, Self::E, Self::F, Self::G, Self::H>) -> Self::HandleResult {
        let id = env.get_id();
        let conect_id = match args {
            Args::OneArgs(conect_id) => conect_id,
            _ => return Err(String::from("invalid net event handler args")),
        };
        let gray_tab = self.gray_tab.read().unwrap();
        let gray = match env.get_gray() {
            Some(v) => match gray_tab.get(v) {
                Some(g) => g.clone(),
                None => return Err(String::from("gray is not exist, version:") + v.to_string().as_str()),
            },
            None => gray_tab.get_last().clone(),
        };

        let handler_name = self.handler.clone();
        let event_name_copy = event_name.clone();
        let func = Box::new(move |_lock| {
            let mgr = gray.mgr.clone();
            let nobjs = gray.nobjs.clone();
            let event_name1 = event_name.clone();
            let real_args = Box::new(move |vm: Arc<JS>| -> usize {
                //事件对象
                let event = vm.new_object();
                vm.set_field(&event, String::from("event_name"), &mut vm.new_str((*event_name1).to_string()).unwrap());
                vm.set_field(&event, String::from("connect_id"), &mut vm.new_u32(conect_id as u32));
                //mgr
                ptr_jstype(vm.get_objs(), vm.clone(), Box::into_raw(Box::new(mgr.clone())) as usize, 2976191628);
                //env
                ptr_jstype(vm.get_objs(), vm.clone(),  Box::into_raw(Box::new(env.clone())) as usize, 589055833);
                //nobj
                nobjs.to_map(&vm);
                4
            });
            gray.factory.call(Some(id), handler_name, real_args, Atom::from((*event_name).to_string() + " net task"));
        });
        cast_net_task(TaskType::Sync(true), 0, Some(new_queue(id)), func, Atom::from("net ".to_string() + &self.handler + ":" + &event_name_copy + " handle task"));

        Ok(())
	}
}

impl NetHandler {
	//构建一个处理器
	pub fn new(handler: String, gray: JSGray) -> NetHandler {
		NetHandler {
			gray_tab: Arc::new(RwLock::new(GrayTab::new(gray))),
            handler: Atom::from(handler),
		}
	}
}

/*
* Topic处理器
*/
#[derive(Clone)]
pub struct TopicHandler {
	gray_tab: 	Arc<RwLock<GrayTab<JSGray>>>, //灰度表
}

unsafe impl Send for TopicHandler {}
unsafe impl Sync for TopicHandler {}

impl Handler for TopicHandler {
	type A = u8;
    type B = Option<SocketAddr>;
    type C = Arc<Vec<u8>>;
    type D = ();
    type E = ();
    type F = ();
    type G = ();
    type H = ();
	type HandleResult = ();

	fn handle(&self, env: Arc<dyn GrayVersion>, topic: Atom, args: Args<Self::A, Self::B, Self::C, Self::D, Self::E, Self::F, Self::G, Self::H>) -> Self::HandleResult {
        let topic_handler = self.clone();
        let topic_name = topic.clone();
        let id = env.get_id();
        let func = Box::new(move |_lock| {
            let gray_tab = topic_handler.gray_tab.read().unwrap();
            let gray = match env.get_gray() {
                Some(v) => match gray_tab.get(v) {
                    Some(g) => g,
                    None => panic!("gray is not exist, version:{}", v),
                },
                None => gray_tab.get_last(),
            };
            let mgr = gray.mgr.clone();
            let nobjs = gray.nobjs.clone();
            let topic_name = topic.clone();
            let real_args = Box::new(move |vm: Arc<JS>| -> usize {
                vm.new_str((*topic_name).to_string());
                let peer_addr = match args {
                    Args::ThreeArgs(_, peer, bin) => {
                        let buffer = vm.new_uint8_array(bin.len() as u32);
                        buffer.from_bytes(bin.as_slice());
                        peer
                    },
                    _ => panic!("invalid topic handler args"),
                };
                let ptr = Box::into_raw(Box::new(mgr.clone())) as usize;
                ptr_jstype(vm.get_objs(), vm.clone(), ptr, 2976191628);
                let ptr = Box::into_raw(Box::new(env.clone())) as usize;
                ptr_jstype(vm.get_objs(), vm.clone(), ptr, 226971089);
                nobjs.to_map(&vm);
                vm.new_u32(id as u32);
                match peer_addr {
                    Some(addr) => {
                        vm.new_str(addr.to_string());
                    },
                    None => {
                        vm.new_undefined();
                    },
                }
                7
            });
            gray.factory.call(Some(id), Atom::from("_$rpc"), real_args, Atom::from((*topic).to_string() + "rpc task"));
        });
        cast_net_task(TaskType::Sync(true), 0, Some(new_queue(id)), func, Atom::from("topic ".to_string() + &topic_name + " handle task"));
	}
}

impl TopicHandler {
	//构建一个处理器
	pub fn new(gray: &Arc<RwLock<GrayTab<JSGray>>>) -> Self {
		TopicHandler {
			gray_tab: gray.clone()
		}
	}
}


//为mqtt绑定网络， 返回mqttserver
pub fn mqtt_bind(mgr: &mut NetMgr, addr: String, protocol: String, send_buf_size: usize, recv_timeout: usize) -> ServerNode{
    let server = ServerNode::new();
    let copy = server.clone();
    let f = Box::new(move |peer:Arc<Result<(RawSocket, Arc<RwLock<RawStream>>),Error>>, _addr: Arc<Result<SocketAddr,Error>> | {
        match peer.as_ref() {
            &Ok(ref peer) => {
                let socket = &peer.0;
                let stream = &peer.1;
                {let s = &mut stream.write().unwrap();
                    s.set_send_buf_size(send_buf_size);
                    s.set_recv_timeout(recv_timeout);
                    s.set_socket(socket.clone());
                }
                copy.clone().add_stream(Socket::Raw(socket.clone()), Stream::Raw(stream.clone()));
            } ,
            Err(s) => println!("{}", s),
        };
    });
    mgr.add_handler(addr.clone(), protocol.clone(), f);
    let server_copy = server.clone();
    mgr.add_close_handler(&addr, &protocol, Box::new(move |id, socket| {
        server_copy.handle_close(id);
    }));
    server
}

pub fn net_connect_bind(mgr: &mut NetMgr, addr: String, protocol: String, handler: &NetHandler, close_handler: &NetHandler) {
    let handler = handler.clone();
    let close_handler = close_handler.clone();
    let close_callback = Box::new(move |id: usize, socket: RawSocket| {
        remove_queue(id);
        let socket = Arc::new(socket);
        match close_handler.handle(socket.clone(), Atom::from("net_connect_close"), Args::OneArgs(id)) {
            Ok(_) => (),
            Err(s) => {
                println!("{}", s);
            },
        };
    });
    let f = Box::new(move |peer:Arc<Result<(RawSocket, Arc<RwLock<RawStream>>),Error>>, _addr: Arc<Result<SocketAddr,Error>>| {
        match peer.as_ref() {
            &Ok(ref peer) => {
                let socket = peer.0.clone();
                let stream = &peer.1;
                let id = socket.socket;
                let socket = Arc::new(socket);
                // let socket1 = socket.clone();
                // let close_handler = close_handler.clone();
                // stream.write().unwrap().set_close_callback(Box::new(move |id: usize, _| {
                //     remove_queue(id);
                //     match close_handler.handle(socket1.clone(), Atom::from("net_connect_close"), Args::OneArgs(id)) {
                //         Ok(_) => (),
                //         Err(s) => {
                //             println!("{}", s);
                //         },
                //     };
                // }));

                match handler.handle(socket, Atom::from("net_connect"), Args::OneArgs(id)){
                    Ok(_) => (),
                    Err(s) => {
                        println!("{}", s);
                    },
                }
            } ,
            Err(s) => println!("{}", s),
        };
        
    });
    mgr.add_handler(addr.clone(), protocol.clone(), f);
    mgr.add_close_handler(&addr, &protocol, close_callback);
}

//为mqtt绑定安全网络， 返回mqttserver
pub fn mqtt_bind_tls(mgr: &mut TlsNetMgr, addr: String, protocol: String, cert_path: String, key_path: String, send_buf_size: usize, recv_timeout: usize) -> ServerNode{
    let server = ServerNode::new();
    let copy = server.clone();
    let f = Box::new(move |peer:Arc<Result<(TlsSocket, Arc<RwLock<TlsStream>>),Error>>, _addr: Arc<Result<SocketAddr,Error>> | {
        match peer.as_ref() {
            &Ok(ref peer) => {
                let socket = &peer.0;
                let stream = &peer.1;
                {let s = &mut stream.write().unwrap();
                    s.set_send_buf_size(send_buf_size);
                    s.set_recv_timeout(recv_timeout);
                    s.set_socket(socket.clone());
                }
                copy.clone().add_stream(Socket::Tls(socket.clone()), Stream::Tls(stream.clone()));
            } ,
            Err(s) => println!("{}", s),
        };
    });
    mgr.add_handler(addr.clone(), protocol.clone(), cert_path, key_path, f);
    let server_copy = server.clone();
    mgr.add_close_handler(&addr, &protocol, Box::new(move |id, socket| {
        server_copy.handle_close(id);
    }));
    server
}

pub fn net_connect_bind_tls(mgr: &mut TlsNetMgr, addr: String, protocol: String, cert_path: String, key_path: String, handler: &NetHandler, close_handler: &NetHandler) {
    let handler = handler.clone();
    let close_handler = close_handler.clone();
    let close_callback = Box::new(move |id: usize, socket: TlsSocket| {
        remove_queue(id);
        let socket = Arc::new(socket);
        match close_handler.handle(socket.clone(), Atom::from("net_connect_close"), Args::OneArgs(id)) {
            Ok(_) => (),
            Err(s) => {
                println!("{}", s);
            },
        };
    });
    let f = Box::new(move |peer:Arc<Result<(TlsSocket, Arc<RwLock<TlsStream>>),Error>>, _addr: Arc<Result<SocketAddr,Error>>| {
        match peer.as_ref() {
            &Ok(ref peer) => {
                let socket = peer.0.clone();
                let stream = &peer.1;
                let id = socket.socket;
                let socket = Arc::new(socket);

                match handler.handle(socket, Atom::from("net_connect"), Args::OneArgs(id)){
                    Ok(_) => (),
                    Err(s) => {
                        println!("{}", s);
                    },
                }
            } ,
            Err(s) => println!("{}", s),
        };
        
    });
    mgr.add_handler(addr.clone(), protocol.clone(), cert_path, key_path, f);
    mgr.add_close_handler(&addr, &protocol, close_callback);
}

pub fn clone_server_node(node: &ServerNode) -> ServerNode{
    node.clone()
}

pub fn clone_rpc_server(server: &RPCServer) -> RPCServer{
    server.clone()
}

pub fn set_mqtt_topic(server_node: &ServerNode, topic: String, can_publish: bool, can_subscribe: bool) -> Result<bool, String> {
    let topic = Atom::from(topic);
    let server_node1 = server_node.clone();
    match server_node.set_topic_meta(topic.clone(), can_publish,can_subscribe, Box::new(move |_c:ClientStub, r:IOResult<Arc<Vec<u8>>>| {
        match r {
            Ok(v) => {
                match server_node1.publish(false, mqtt3::QoS::AtMostOnce, topic.clone(),Vec::from(v.as_slice())) {
                    Ok(_) => (),
                    Err(s) => {println!("{}, topic:{}", s.to_string(), topic.as_str());},
                }
            },
            Err(s) => {
                println!("{}, topic:{}", s.to_string(), topic.as_str());
            },
        }
    })) {
        Ok(_) => Ok(true),
        Err(s) => Err(s.to_string()),
    } 
}

pub fn unset_mqtt_topic(server_node: &ServerNode, topic: String) -> Result<(), String> {
    match server_node.unset_topic_meta(Atom::from(topic)) {
        Ok(r) => Ok(r),
        Err(r) => Err(r.to_string()),
    }
}

pub enum QoS{
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

pub fn mqtt_publish(server: &ServerNode, retain: bool, qos: QoS, topic: String, payload: &[u8]) -> Result<(), Error>{
    let qos = match qos {
        QoS::AtMostOnce => mqtt3::QoS::AtMostOnce,
        QoS::AtLeastOnce => mqtt3::QoS::AtLeastOnce,
        QoS::ExactlyOnce => mqtt3::QoS::ExactlyOnce,
    };
    server.publish(retain, qos, Atom::from(topic), Vec::from(payload))
}

pub fn mqtt_respond(session: &Arc<Session>, topic: String, data: &[u8]) {
    session.respond(Atom::from(topic), Vec::from(data));
}

//为rpc注册handler
pub fn register_rpc_handler(serv: &mut RPCServer, topic: String, sync: bool, handler: &Arc<TopicHandler>) -> Result<(), Error> {
    serv.register(Atom::from(topic), sync, handler.clone())
}

//为rpc注册handler
pub fn arc_new_topic_handler(th: TopicHandler) -> Arc<TopicHandler> {
    Arc::new(th)
}

//为pi_p2p封装一个P2PManage::new方法
// pub fn p2p_manage_new(addr: &str, arr1: Vec<String>, arr2: Vec<u32>) -> P2PManage {

//     let mut map: FnvHashMap<SocketAddr, u64> = FnvHashMap::default();
//     let mut i = 0;
//     for time in arr2 {
//         map.insert(arr1.get(i).unwrap().parse().unwrap(), time as u64);
//         i += 1;
//     }
//     P2PManage::new(addr.parse().unwrap(), map)
// }

pub fn creat_arc_sokect(socket: Socket ) -> Arc<Socket>{
    Arc::new(socket)
}