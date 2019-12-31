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
use rpc_tmp::traits::RPCServerTraits;
use rpc_tmp::server::RPCServer;
use net::data::{RawSocket, RawStream};
use net::tls::{TlsSocket, TlsStream, TlsConfig as TlsCfg};
use net::{Config, Protocol};
use net::api::{Socket, Stream};
use net::api::{NetManager, TlsManager};
use net::data::ListenerFn;
use mqtt_tmp::server::{ServerNode, ClientStub};
use std::io::{Result as IOResult};
use mqtt_tmp::data::Server;
use mqtt_tmp::session::Session;
use js_lib::JSGray;
use worker::task::TaskType;
use worker::impls::{unlock_js_task_queue, cast_js_task};
use tcp::connect::TcpSocket;
use tcp::server::{AsyncWaitsHandle, AsyncPortsFactory, SocketListener};
use tcp::driver::{Socket as SocketTrait, SocketConfig, AsyncIOWait, AsyncServiceFactory};
use tcp::buffer_pool::WriteBufferPool;
use tcp::util::TlsConfig;
use ws::server::WebsocketListenerFactory;
use mqtt::v311::{WS_MQTT3_BROKER, WsMqtt311, WsMqtt311Factory, add_topic, publish_topic};
use base::service::{BaseListener, BaseService};
use base::connect::encode;
use rpc::service::{RpcService, RpcListener};
use rpc::connect::RpcConnect;

fn now_millis() -> isize {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Err(e) => -1,
        Ok(n) => n.as_millis() as isize,
    }
}

/**
* Tcp网络管理器
*/
pub struct NetMgr {
    pub mgr: NetManager,
    pub handler: Arc<Mutex<FnvHashMap<Atom, Vec<Box<Fn(Arc<Result<(RawSocket, Arc<RwLock<RawStream>>),Error>>,
    Arc<Result<SocketAddr,Error>>) + Send>>>>>,
    pub close_handler: Arc<Mutex<FnvHashMap<Atom, Vec<Box<Fn(usize, RawSocket) + Send>>>>>,
}

impl NetMgr {
    /**
    * 构建Tcp网络管理器
    * @returns 返回Tcp网络管理器
    */
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

/**
* Tls网络管理器
*/
pub struct TlsNetMgr {
    pub mgr: TlsManager,
    pub handler: Arc<Mutex<FnvHashMap<Atom, Vec<Box<Fn(Arc<Result<(TlsSocket, Arc<RwLock<TlsStream>>),Error>>,
    Arc<Result<SocketAddr,Error>>) + Send>>>>>,
    pub close_handler: Arc<Mutex<FnvHashMap<Atom, Vec<Box<Fn(usize, TlsSocket) + Send>>>>>,
}

impl TlsNetMgr {
    /**
    * 构建Tls网络管理器
    * @returns 返回Tls网络管理器
    */
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
            let cfg = TlsCfg::new(
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

/**
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

        let queue = new_queue(id); //创建指定socket的同步静态队列
        let handler_name = self.handler.clone();
        let event_name_copy = event_name.clone();
        let func = Box::new(move |lock: Option<isize>| {
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

            //解锁当前同步静态队列，保证虚拟机执行
            if !unlock_js_task_queue(queue) {
                warn!("!!!> Net Handle Error, unlock task queue failed, queue: {:?}", queue);
            }
        });
        cast_js_task(TaskType::Sync(true), 0, Some(queue), func, Atom::from("net ".to_string() + &self.handler + ":" + &event_name_copy + " handle task"));

        Ok(())
	}
}

impl NetHandler {
	/**
	* 构建一个网络连接Handler
	* @param handler 处理器名称
	* @param gray 灰度对象
	* @returns 返回网络连接Handler
	*/
	pub fn new(handler: String, gray: JSGray) -> NetHandler {
		NetHandler {
			gray_tab: Arc::new(RwLock::new(GrayTab::new(gray))),
            handler: Atom::from(handler),
		}
	}
}

/**
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
        let queue = new_queue(id); //创建指定socket的同步静态队列
        let func = Box::new(move |lock: Option<isize>| {
            println!("{}, net trace, run rpc task, token: {:?}, topic: {:?}", now_millis(), id, topic);
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
                ptr_jstype(vm.get_objs(), vm.clone(), ptr, 717646231);
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
            gray.factory.call(Some(id), Atom::from("_$rpc_tmp"), real_args, Atom::from((*topic).to_string() + " rpc task"));

            //解锁当前同步静态队列，保证虚拟机执行
            if !unlock_js_task_queue(queue) {
                warn!("!!!> Topic Handle Error, unlock task queue failed, queue: {:?}", queue);
            }
        });
        cast_js_task(TaskType::Sync(true), 0, Some(queue), func, Atom::from("topic ".to_string() + &topic_name + " handle task"));
        println!("{}, net trace, topic handle, token: {:?}, topic: {:?}", now_millis(), id, topic_name);
	}
}

impl TopicHandler {
	/**
	* 构建一个Topic处理器
	* @param gray 灰度对象
	* @returns 返回Topic处理器
	*/
	pub fn new(gray: &Arc<RwLock<GrayTab<JSGray>>>) -> Self {
		TopicHandler {
			gray_tab: gray.clone()
		}
	}
}


/**
* 为mqtt绑定Tcp网络
* @param mgr Tcp网络管理器
* @param addr 绑定的地址
* @param protocol 绑定的协议名
* @param send_buf_size 发送缓冲区大小
* @param recv_timeout 接收超时时长，单位毫秒
* @returns 返回Mqtt服务器
*/
pub fn mqtt_bind(server: &ServerNode, mgr: &mut NetMgr, addr: String, protocol: String, send_buf_size: usize, recv_timeout: usize){
    // let server = ServerNode::new();
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
}

/**
* 设置Tcp网络连接和关闭处理器
* @param mgr Tcp网络管理器
* @param addr 绑定的地址
* @param protocol 绑定的协议名
* @param handler 连接处理器
* @param close_handler 关闭处理器
*/
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

/**
* 为mqtt绑定Tls网络
* @param mgr Tls网络管理器
* @param addr 绑定的地址
* @param protocol 绑定的协议名
* @param send_buf_size 发送缓冲区大小
* @param recv_timeout 接收超时时长，单位毫秒
* @returns 返回Mqtt服务器
*/
pub fn mqtt_bind_tls(server: &ServerNode, mgr: &mut TlsNetMgr, addr: String, protocol: String, cert_path: String, key_path: String, send_buf_size: usize, recv_timeout: usize){
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
}

/**
* 设置Tls网络连接和关闭处理器
* @param mgr Tls网络管理器
* @param addr 绑定的地址
* @param protocol 绑定的协议名
* @param handler 连接处理器
* @param close_handler 关闭处理器
*/
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

/**
* Copy指定的Mqtt服务器
* @param node 待Copy的Mqtt服务器
* @returns 返回已Copy的Mqtt服务器
*/
pub fn clone_server_node(node: &ServerNode) -> ServerNode{
    node.clone()
}

/**
* Copy指定的RPC服务器
* @param server 待Copy的RPC服务器
* @returns 返回已Copy的RPC服务器
*/
pub fn clone_rpc_server(server: &RPCServer) -> RPCServer{
    server.clone()
}

/**
* 为指定的Mqtt服务器设置topic
* @param server_node Mqtt服务器
* @param topic Topic
* @param can_publish 是否可发布
* @param can_subscribe 是否可订阅
* @returns 返回是否设置成功
* @throws 失败抛出原因描述
*/
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

/**
* 为指定的Mqtt服务器取消topic
* @param server_node Mqtt服务器
* @param topic Topic
* @returns 返回设置结果，成功返回空
* @throws 失败抛出原因描述
*/
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

/**
* 发布指定topic的消息
* @param server Mqtt服务器
* @param retain 是否保留Mqtt会话
* @param qos Qos
* @param topic Topic
* @param payload 消息体
* @returns 返回发布结果，成功返回空
& @throws 失败抛出原因描述
*/
pub fn mqtt_publish(server: &ServerNode, retain: bool, qos: QoS, topic: String, payload: &[u8]) -> Result<(), Error>{
    let qos = match qos {
        QoS::AtMostOnce => mqtt3::QoS::AtMostOnce,
        QoS::AtLeastOnce => mqtt3::QoS::AtLeastOnce,
        QoS::ExactlyOnce => mqtt3::QoS::ExactlyOnce,
    };
    server.publish(retain, qos, Atom::from(topic), Vec::from(payload))
}

/**
* 回应指定指定topic发布的消息
* @param session Mqtt会话
* @param topic Topic
* @param data 回应的数据
*/
pub fn mqtt_respond(session: &Arc<Session>, topic: String, data: &[u8]) {
    session.respond(Atom::from(topic), Vec::from(data));
}

/**
* 为rpc注册handler
*/
pub fn register_rpc_handler(serv: &mut RPCServer, topic: String, sync: bool, handler: &Arc<TopicHandler>) -> Result<(), Error> {
    serv.register(Atom::from(topic), sync, handler.clone())
}

/**
* 创建一个Topic处理器的引用计数
*/
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

// /**
// * 创建一个公共Socket的引用计数
// */
// pub fn creat_arc_sokect(socket: Socket ) -> Arc<Socket>{
//     Arc::new(socket)
// }

/**
* 网络事件处理器
*/
#[derive(Clone)]
pub struct NetEventHandler {
    handler: Atom, //处理函数名称（js函数）
    gray_tab: Arc<RwLock<GrayTab<JSGray>>>, //灰度表
}

unsafe impl Send for NetEventHandler {}
unsafe impl Sync for NetEventHandler {}

impl Handler for NetEventHandler {
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

        let queue = new_queue(id); //创建指定socket的同步静态队列
        let handler_name = self.handler.clone();
        let event_name_copy = event_name.clone();
        let func = Box::new(move |lock: Option<isize>| {
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

            //解锁当前同步静态队列，保证虚拟机执行
            if !unlock_js_task_queue(queue) {
                warn!("!!!> Net Handle Error, unlock task queue failed, queue: {:?}", queue);
            }
        });
        cast_js_task(TaskType::Sync(true), 0, Some(queue), func, Atom::from("net ".to_string() + &self.handler + ":" + &event_name_copy + " handle task"));

        Ok(())
    }
}

impl NetEventHandler {
    /**
    * 构建一个网络事件处理器
    * @param handler 处理器名称
    * @param gray 灰度对象
    * @returns 返回网络事件处理器
    */
    pub fn new(handler: String, gray: JSGray) -> NetEventHandler {
        NetEventHandler {
            gray_tab: Arc::new(RwLock::new(GrayTab::new(gray))),
            handler: Atom::from(handler),
        }
    }
}

/**
* Rpc请求处理器
*/
#[derive(Clone)]
pub struct RequestHandler {
    gray_tab: 	Arc<RwLock<GrayTab<JSGray>>>, //灰度表
}

unsafe impl Send for RequestHandler {}
unsafe impl Sync for RequestHandler {}

impl Handler for RequestHandler {
    type A = u8;
    type B = Option<SocketAddr>;
    type C = u32;
    type D = Arc<Vec<u8>>;
    type E = ();
    type F = ();
    type G = ();
    type H = ();
    type HandleResult = ();

    fn handle(&self, env: Arc<dyn GrayVersion>, topic: Atom, args: Args<Self::A, Self::B, Self::C, Self::D, Self::E, Self::F, Self::G, Self::H>) -> Self::HandleResult {
		let topic_handler = self.clone();
        let topic_name = topic.clone();
        let id = env.get_id();
        let queue = new_queue(id); //创建指定socket的同步静态队列
        let func = Box::new(move |lock: Option<isize>| {
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
                    Args::FourArgs(_, peer, rid, bin) => {
                        let buffer = vm.new_uint8_array(bin.len() as u32);
                        buffer.from_bytes(bin.as_slice());
						vm.new_u32(rid); // rid
                        peer
                    },
                    _ => panic!("invalid topic handler args"),
                };
                let ptr = Box::into_raw(Box::new(mgr.clone())) as usize;
                ptr_jstype(vm.get_objs(), vm.clone(), ptr, 2976191628);
                let ptr = Box::into_raw(Box::new(env.clone())) as usize;
                ptr_jstype(vm.get_objs(), vm.clone(), ptr, 3092548949);
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
                8
            });
            gray.factory.call(Some(id), Atom::from("_$rpc"), real_args, Atom::from((*topic).to_string() + " rpc task"));

            //解锁当前同步静态队列，保证虚拟机执行
            if !unlock_js_task_queue(queue) {
                warn!("!!!> Topic Handle Error, unlock task queue failed, queue: {:?}", queue);
            }
        });
        cast_js_task(TaskType::Sync(true), 0, Some(queue), func, Atom::from("topic ".to_string() + &topic_name + " handle task"));
    }
}

impl RequestHandler {
    /**
    * 构建一个Rpc请求处理器
    * @param gray 灰度对象
    * @returns 返回Rpc请求处理器
    */
    pub fn new(gray: &Arc<RwLock<GrayTab<JSGray>>>) -> Self {
        RequestHandler {
            gray_tab: gray.clone()
        }
    }
}

/**
* 创建Rpc服务
*/
pub fn create_rpc_service(handler: &RequestHandler) -> Arc<BaseService> {
	let mut rpc_service = RpcService::new();
    rpc_service.set_request_handler(Arc::new(handler.clone()));
	let rpc_service = Arc::new(rpc_service);
	Arc::new(BaseService::with_service(rpc_service))
}

/**
* 注册网络事件监听器
*/
pub fn register_rcp_listener(conect_handler: Option<&NetEventHandler>, close_handler: Option<&NetEventHandler>) -> Arc<RpcListener> {
	let mut rpc_listener = RpcListener::new();
	if let Some(r) = conect_handler {
        rpc_listener.set_connected_handler(Arc::new(r.clone()));
	}
	if let Some(r) = close_handler {
        rpc_listener.set_closed_handler(Arc::new(r.clone()));
	}
    let rpc_listener = Arc::new(rpc_listener);
    let listener = Arc::new(BaseListener::with_listener(rpc_listener.clone()));
	WS_MQTT3_BROKER.register_listener(listener);
    rpc_listener
}

/**
* 为指定的Mqtt主题，注册指定的Rpc服务
*/
pub fn register_rpc_topic(topic: String, service: &Arc<BaseService>) {
    WS_MQTT3_BROKER.register_service(topic, service.clone());
}

/**
* rpc回应
*/
pub fn rpc_reply(connect: &Arc<RpcConnect>, rid: u32, data: &[u8]) {
    connect.reply(rid, Vec::from(data));
}

/**
* rpc发送
*/
pub fn rpc_send(connect: &Arc<RpcConnect>, topic: String, rid: u32, data: &[u8]) {
    connect.send(topic, rid, Vec::from(data));
}

/**
* 为指定地址的指定端口，设置指定Websocket子协议名的全局Mqtt服务器，并绑定对应的Tcp端口
*/
pub fn global_mqtt_bind_tcp_ports(ip: String,                       //绑定的本地ip地址
                                  ports: &[u16],
                                  recv_buffer_size: usize,          //连接的接收缓冲区，单位B
                                  send_buffer_size: usize,          //连接的发送缓冲区，单位B
                                  read_buffer_capacity: usize,      //连接的读缓冲区，单位B
                                  write_buffer_capacity: usize,     //连接的写缓冲区，单位次
                                  pool_size: usize,                 //连接池的初始容量
                                  stack_size: usize,                //连接线程的栈大小
                                  timeout: usize,                   //连接轮询的间隔时长，单位毫秒
                                  protocol: String) {
    let mut factory = AsyncPortsFactory::<TcpSocket>::new();
    for port in ports {
        factory.bind(port.clone(),
                     Box::new(WebsocketListenerFactory::<TcpSocket>::with_protocol_factory(
                         Arc::new(WsMqtt311Factory::with_name(&protocol)))));
    }

    let mut config = SocketConfig::new(&ip, factory.bind_ports().as_slice());
    config.set_option(recv_buffer_size, send_buffer_size, read_buffer_capacity, write_buffer_capacity);
    let buffer = WriteBufferPool::new(10000, 10, 3).ok().unwrap();
    match SocketListener::bind(factory, buffer, config, TlsConfig::empty(), pool_size, stack_size, 1024, Some(timeout)) {
        Err(e) => {
            panic!("Mqtt bind tcp port Error, reason: {:?}", e);
        },
        Ok(_) => {
            info!("===> Mqtt bind tcp port ok");
        }
    }
}

/**
* 可以在运行时线程安全的，为全局Mqtt服务器增加指定的主题
*/
pub fn add_global_mqtt_topic(is_public: bool,   //是否为公共主题，指定用户的主题不是公共主题
                             topic: String) {
    add_topic(is_public, topic, 0, None);
}

/**
* 可以在运行时线程安全的，在全局Mqtt服务器上发布指定主题的消息
*/
pub fn publish_global_mqtt_topic(is_public: bool,   //是否为公共主题，指定用户的主题不是公共主题
                                 topic: String, msg: &[u8]) {
    if let Ok(bin) = encode(0, false, 0, msg) {
        publish_topic(is_public, topic, 0, None, Arc::new(bin));
    }
}

