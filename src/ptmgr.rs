use std::sync::Arc;

use fnv::FnvHashMap;
use parking_lot::RwLock;

use pi_db::mgr::Mgr;

lazy_static! {
    pub static ref PLAT_MGR: GlobalPlatMgr = GlobalPlatMgr(Arc::new(RwLock::new(PlatMgr::default())));
}

pub trait PlatMgrTrait {
    // 注册启动的项目名字，项目名字可以根据命令行参数确定
    fn register_project(&self, proj_name: String);

    // 注册每个项目的数据库管理器，现阶段的情况是多个项目共用的同一个管理器，
    // 拿到数据库管理器就可以对表数据进行增删改查:
    // 1. 注册和取消注册一个库 (register/unregister)
    // 2. 获取某个库的某个表的元信息 (tab_info)
    // 3. 创建一个事务 (transaction)
    // 4. 注册数据库监听器 (listen)
    // 5. 获取数据库管理器上注册的所有库 (ware_name_list)
    fn register_db_mgr(&self, proj_name: Option<String>, mgr: Mgr);

    // 注册每个项目的rpc列表，所有 rpc 都在 *.event.js 中定义
    // 可以在创建虚拟机工厂的时候捕获
    // vmf_name + topic_name 就可以调用一个rpc了
    fn register_rpc(&self, proj_name: String, vmf_name: String, topic_name: String);

    // 注册每个项目的数据库监听器列表，数据库监听器向前端推送数据库的修改。
    // 这里保存数据库监听器对应的mqtt topic 名字
    fn register_db_monitor(&self, proj_name: String, endpoint: String);

    // 注册每项目使用了的mqtt topic
    fn register_mqtt_topic(&self, proj_name: String, topic: String);

    // 注册每个项目启动的网络服务, 记录服务名字和端口号
    fn register_net_service(&self, proj_name: String, service_name: String, port: u16);
}

// 平台管理端结构
#[derive(Default, Debug)]
pub struct PlatMgr {
    // 启动的项目
    projects: Vec<String>,
    // 全局的数据库管理器
    db_mgr: Option<Mgr>,
    // 每个项目的rpc列表, {"虚拟机工厂名 => [rpc函数名]"}
    rpcs: FnvHashMap<String, (String, Vec<String>)>,
    // 每个项目注册的数据库监听器
    db_monitors: FnvHashMap<String, Vec<String>>,
    // 每个项目注册的mqtt topic
    mqtt_topics: FnvHashMap<String, Vec<String>>,
    // 每个项目注册的网络服务
    net_services: FnvHashMap<String, Vec<(String, u16)>>
}


pub struct GlobalPlatMgr(Arc<RwLock<PlatMgr>>);

impl GlobalPlatMgr {
    pub fn db_mgr(&self) -> Option<Mgr> {
        self.0.read().db_mgr.clone()
    }

    pub fn projects(&self) -> Vec<String> {
        self.0.read().projects.clone()
    }

    pub fn project_rpcs(&self, proj_name: String) -> Vec<String> {
        let mut res = vec![];
        match self.0.read().rpcs.get(&proj_name) {
            Some(rpcs) => {
                for rpc in rpcs.1.clone() {
                    let endpoint = rpcs.0.clone() + &rpc;
                    res.push(endpoint);
                }
                res
            }
            None => vec![]
        }
    }

    pub fn project_db_monitors(&self, proj_name: String) -> Vec<String> {
        match self.0.read().db_monitors.get(&proj_name) {
            Some(db_monitors) => db_monitors.clone(),
            None => vec![]
        }
    }

    pub fn project_mqtt_topics(&self, proj_name: String) -> Vec<String> {
        match self.0.read().mqtt_topics.get(&proj_name) {
            Some(mqtt_topics) => mqtt_topics.clone(),
            None => vec![]
        }
    }

    pub fn project_net_services(&self, proj_name: String) -> Vec<(String, u16)> {
        match self.0.read().net_services.get(&proj_name) {
            Some(net_services) => net_services.clone(),
            None => vec![]
        }
    }
}

impl PlatMgrTrait for GlobalPlatMgr {
    fn register_project(&self, proj_name: String) {
        self.0.write().projects.push(proj_name);
    }

    fn register_db_mgr(&self, _proj_name: Option<String>, mgr: Mgr) {
        self.0.write().db_mgr = Some(mgr);
    }

    fn register_rpc(&self, proj_name: String, vmf_name: String, topic_name: String) {
        let mut plat_mgr = self.0.write();
        match plat_mgr.rpcs.get_mut(&proj_name) {
            Some(rpcs) => {
                if rpcs.0 == vmf_name {
                    rpcs.1.push(topic_name);
                } else {
                    plat_mgr.rpcs.insert(proj_name, (vmf_name, vec![topic_name]));
                }
            }
            None => {
                plat_mgr.rpcs.insert(proj_name, (vmf_name, vec![topic_name]));
            }
        }
    }

    fn register_db_monitor(&self, proj_name: String, endpoint: String) {
        let mut plat_mgr = self.0.write();
        plat_mgr.db_monitors.entry(proj_name)
            .and_modify(|endpoints| endpoints.push(endpoint.clone()))
            .or_insert(vec![endpoint]);
    }

    fn register_mqtt_topic(&self, proj_name: String, topic: String) {
        let mut plat_mgr = self.0.write();
        plat_mgr.mqtt_topics.entry(proj_name)
            .and_modify(|mqtt_topics| mqtt_topics.push(topic.clone()))
            .or_insert(vec![topic]);
    }

    fn register_net_service(&self, proj_name: String, service_name: String, port: u16) {
        let mut plat_mgr = self.0.write();
        plat_mgr.net_services.entry(proj_name)
            .and_modify(|net_services| net_services.push((service_name.clone(), port)))
            .or_insert(vec![(service_name, port)]);
    }
}