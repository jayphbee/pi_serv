use std::sync::Arc;
use std::net::SocketAddr;
use std::io::{Error};
use std::any::Any;

use fnv::FnvHashMap;

use pi_lib::atom::Atom;
use pi_lib::handler::{Env, GenType};
use pi_p2p::manage::P2PManage;
use rpc::traits::RPCServerTraits;
use rpc::server::RPCServer;
use net::{Config, Protocol};
use net::api::NetManager;
use mqtt::server::{ServerNode, ClientStub};
use std::io::{Result as IOResult};
use mqtt::data::Server;
use mqtt::session::Session;
use handler::TopicHandler;


//为mqtt绑定网络， 返回mqttserver
pub fn mqtt_bind(mgr: &NetManager, addr: String, protocol: String, send_buf_size: usize, recv_timeout: usize) -> ServerNode{
    let cfg = Config{
        protocol: match protocol.as_str() {
            "tcp" => Protocol::TCP,
            _ => {panic!("nonsupport protocol:{}", protocol);},
        },
        addr: addr.parse().unwrap()
    };
    let server = ServerNode::new();
    let copy = server.clone();
    mgr.bind(cfg, Box::new(move |peer, _addr| {
        let (socket, stream) = peer.unwrap();
        {
            let s = &mut stream.write().unwrap();

            s.set_close_callback(Box::new(|id, _reason| {
                println!("server handle_close, stream_id = {}",id);
            }));
            s.set_send_buf_size(send_buf_size);
            s.set_recv_timeout(recv_timeout);
            s.set_socket(socket.clone());
        }
        let mut copy = copy.clone();
        copy.add_stream(socket, stream);
    }));
    server
}

pub fn clone_server_node(node: &ServerNode) -> ServerNode{
    node.clone()
}

pub fn set_mqtt_topic(server_node: ServerNode, topic: String, can_publish: bool, can_subscribe: bool, only_one_key: Option<String>) -> Result<bool, String> {
    let only_one_key = match only_one_key {
        Some(s) => Some(Atom::from(s)),
        None => None,
    };
    match server_node.set_topic_meta(Atom::from(topic), can_publish,can_subscribe,only_one_key, Box::new(|_c:ClientStub, _r:IOResult<Arc<Vec<u8>>>| {})) {
        Ok(_) => Ok(true),
        Err(s) => Err(s.to_string()),
    } 
}

pub fn mqtt_respond(session: &Arc<Session>, topic: String, data: &[u8]) {
    println!("mqtt_respond------------------------{:p}", session);
    session.respond(Atom::from(topic), Vec::from(data));
}

pub fn get_attr(session: &Arc<Session>, key: String) -> Option<Vec<u8>> {
    match session.get_attr(Atom::from(key)) {
        Some(v) => match v {
            GenType::Bin(arr) => Some(arr),
            _ => {println!("session get_attr err, expect GenType::ArcBin, found unknow", );  None},
        },
        None => None,
    }
}
//设置属性，返回上个属性值
pub fn set_attr(session: &Arc<Session>, key: String, value: &[u8]) {
    session.set_attr(Atom::from(key), GenType::Bin(Vec::from(value)));
}
//移除属性
pub fn remove_attr(session: &Arc<Session>, key: String){
    session.remove_attr(Atom::from(key));
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
pub fn p2p_manage_new(addr: &str, arr1: Vec<String>, arr2: Vec<u32>) -> P2PManage {

    let mut map: FnvHashMap<SocketAddr, u64> = FnvHashMap::default();
    let mut i = 0;
    for time in arr2 {
        map.insert(arr1.get(i).unwrap().parse().unwrap(), time as u64);
        i += 1;
    }
    P2PManage::new(addr.parse().unwrap(), map)
}