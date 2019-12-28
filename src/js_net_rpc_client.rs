use std::sync::{Arc, Mutex, RwLock};
use std::net::SocketAddr;
use std::io::{Error, Result as IOResult};

use fnv::FnvHashMap;
use mqtt3;

use pi_vm::adapter::{JS};
use pi_vm::pi_vm_impl::{remove_queue};
use pi_vm::bonmgr::{ptr_jstype};
use handler::{Args, Handler};
use gray::{GrayVersion, GrayTab};
use atom::Atom;
// use pi_p2p::manage::P2PManage;
use rpc_tmp::traits::RPCServerTraits;
use rpc_tmp::server::RPCServer;
use net::data::{RawSocket, RawStream};
use net::tls::{TlsSocket, TlsStream, TlsConfig};
use net::{Config, Protocol};
use net::api::{Socket, Stream};
use net::api::{NetManager, TlsManager};
use nodec::rpc::RPCClient as NetRPCClient;
use nodec::mqttc::SharedMqttClient as NetSharedMqttClient;
use net::data::ListenerFn;
use mqtt_tmp::server::{ServerNode, ClientStub};
use mqtt_tmp::data::Server;
use mqtt_tmp::session::Session;
use js_lib::JSGray;
use worker::task::TaskType;
use worker::impls::cast_net_task;

/*
* RPC异步访问任务类型
*/
const ASYNC_RPC_TASK_TYPE: TaskType = TaskType::Async(false);

/*
* RPC异步访问任务优先级
*/
const ASYNC_RPC_PRIORITY: usize = 100;

/**
* RPC客户端
*/
#[derive(Clone)]
pub struct RPCClient(Arc<NetRPCClient>);

impl RPCClient {
    /**
    * 创建一个RPC客户端
    * @param url 需要连接的RPC服务器端url
    * @returns 返回创建结果，成功返回RPC客户端
    * @throws 失败抛出原因描述
    */
    pub fn create(url: &str) -> Result<Self, String> {
        match NetRPCClient::create(url) {
            Err(e) => Err(e.to_string()),
            Ok(r) => Ok(RPCClient(Arc::new(r))),
        }
    }

    /**
    * 使用RPC客户端建立一个连接
    * @param keep_alive 连接保持时长，单位秒
    * @param client_id 客户端唯一id，相同的客户端唯一id，同一时间只可创建一个连接，如已创建连接，则忽略
    * @param timeout 建立连接超时时长，单位秒
    * @param closed_handler 关闭连接的异步回调
    * @param connect_callback 建立连接的异步回调，成功返回空数组，失败返回原因描述
    */
    pub fn connect(&self,
                   keep_alive: u16,
                   client_id: &str,
                   timeout: u8,
                   closed_handler: Option<CloseHandler>,
                   connect_callback: Arc<Fn(Result<Option<Vec<u8>>, String>)>) {
        let client = self.clone();
        self.0.connect(keep_alive, client_id, timeout, Arc::new(move |r: IOResult<Option<Vec<u8>>>|{
            match r {
                Err(e) => connect_callback(Err(e.to_string())),
                Ok(e) => connect_callback(Ok(e)),
            }
        }), Arc::new(move |_r| {
            match &closed_handler {
                Some(r) => {r.handle(client.0.clone());},
                None => (),
            };
        }));
    }

    /**
    * 使用RPC客户端发送请求
    * @param cmd Topic
    * @param body 数据
    * @param timeout 发送请求的超时时长，单位秒
    * @param callback 发送请求的异步回调，成功返回回应数据，失败返回原因描述
    */
    pub fn request(&self,
                   cmd: String,
                   body: &[u8],
                   timeout: u8,
                   callback: Arc<Fn(Result<Option<Vec<u8>>, String>)>) {
        self.0.request(cmd, Vec::from(body), timeout, Arc::new(move |r: IOResult<Option<Vec<u8>>>| {
            match r {
                Err(e) => callback(Err(e.to_string())),
                Ok(e) => {
                    match e {
                        Some(r) => callback(Ok(Some(r))),
                        None => callback(Ok(None)),
                    };
                },
            }
        }));
    }

    /**
    * 关闭连接
    */
    pub fn close(&self) {
        self.0.close();
    }
}

impl GrayVersion for RPCClient {
    fn get_gray(&self) -> &Option<usize> {
        println!("GrayVersion rpcclient ");
        &None
    }
    fn set_gray(&mut self, gray: Option<usize>) {

    }
	fn get_id(&self) -> usize {
        0
    }
}

/*
* rpc 客户端连接关闭事件处理
*/
#[derive(Clone)]
pub struct CloseHandler {
    handler: Atom, //处理函数名称（js函数）
    gray_tab: Arc<RwLock<GrayTab<JSGray>>>, //灰度表
}

unsafe impl Send for CloseHandler {}
unsafe impl Sync for CloseHandler {}

impl CloseHandler {
    //构建一个处理器
    pub fn new(handler: String, gray: JSGray) -> CloseHandler {
        CloseHandler {
            gray_tab: Arc::new(RwLock::new(GrayTab::new(gray))),
            handler: Atom::from(handler),
        }
    }

	fn handle(&self, env: Arc<NetRPCClient>) {
        let gray_tab = self.gray_tab.read().unwrap();
        let gray =  gray_tab.get_last().clone();
        let handler_name = self.handler.clone();
        let func = Box::new(move |_lock| {
            let mgr = gray.mgr.clone();
            let nobjs = gray.nobjs.clone();
            let real_args = Box::new(move |vm: Arc<JS>| -> usize {
                // RPCClient
                ptr_jstype(vm.get_objs(), vm.clone(), Box::into_raw(Box::new(RPCClient(env))) as usize, 4088898725);
                // mgr
                ptr_jstype(vm.get_objs(), vm.clone(), Box::into_raw(Box::new(mgr.clone())) as usize, 2976191628);
                // nobj
                nobjs.to_map(&vm);
                2
            });
            gray.factory.call(None, handler_name, real_args, Atom::from("rpc client close task"));
        });
        cast_net_task(ASYNC_RPC_TASK_TYPE, ASYNC_RPC_PRIORITY, None, func, Atom::from("rpc client close ".to_string() + &self.handler + " handle task"));
	}
}