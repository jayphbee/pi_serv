// 后台定时任务
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

use crossbeam_channel::{unbounded, Receiver, Sender};

use atom::Atom;
use pi_db::mgr::Mgr;
use pi_vm::adapter::JS;
use pi_vm::bonmgr::ptr_jstype;
use pi_vm::pi_vm_impl::VMFactory;
use time::run_millis;
use wheel::slab_wheel::Wheel;
use wheel::wheel::Item;
use worker::impls::{cast_js_task, unlock_js_task_queue};
use worker::task::TaskType;

lazy_static! {
    // 定时任务的虚拟机工厂
    static ref FACTORY: Arc<RwLock<HashMap<String, (Arc<VMFactory>, Mgr)>>> =
        Arc::new(RwLock::new(HashMap::new()));
    // 所有的定时任务集合，用来支持取消任务
    static ref TASK_MAP: Arc<RwLock<HashMap<String, usize>>> =
        Arc::new(RwLock::new(HashMap::new()));

    static ref TIMER_TASK_CHAN: (Sender<TimerTaskMsg>, Receiver<TimerTaskMsg>) = unbounded();
}

pub fn register_timer_task_vm_factory(topic: String, vmf: Arc<VMFactory>, dbmgr: Mgr) {
    FACTORY.write().insert(topic, (vmf, dbmgr));
}

pub fn insert_oneshot_task(topic: String, after: usize) {
    let _ =TIMER_TASK_CHAN
        .0
        .send(TimerTaskMsg::InsertOneShotTask { topic, after });
}

pub fn insert_periodic_task(topic: String, period: usize) {
    let _ = TIMER_TASK_CHAN
        .0
        .send(TimerTaskMsg::InsertPeriodicTask { topic, period });
}

pub fn cancel_task(topic: String) {
    let _ = TIMER_TASK_CHAN.0.send(TimerTaskMsg::CancelTask { topic });
}

pub fn tick() {
    let _ = TIMER_TASK_CHAN.0.send(TimerTaskMsg::Tick);
}

pub fn timer_task_loop() {
    let _ = thread::Builder::new()
        .name("timer task".to_string())
        .spawn(move || {
            let mut wheel = Wheel::new();
            wheel.set_time(run_millis());
            let mut periodic_tasks = HashMap::new();

            loop {
                match TIMER_TASK_CHAN.1.recv() {
                    Ok(TimerTaskMsg::InsertOneShotTask { topic, after }) => {
                        let cur_time = wheel.get_time();
                        let index = wheel.insert(Item {
                            elem: topic.clone(),
                            time_point: cur_time + after as u64,
                        });
                        TASK_MAP.write().insert(topic, index);
                        println!("after insert one shot task {:?}", wheel);
                    }
                    Ok(TimerTaskMsg::InsertPeriodicTask { topic, period }) => {
                        let cur_time = wheel.get_time();
                        let index = wheel.insert(Item {
                            elem: topic.clone(),
                            time_point: cur_time + period as u64,
                        });
                        TASK_MAP.write().insert(topic.clone(), index);
                        periodic_tasks.insert(topic, period);
                        println!("after insert periodic task {:?}", wheel);

                    }
                    Ok(TimerTaskMsg::CancelTask { topic }) => {
                        TASK_MAP.write().remove(&topic).and_then(|index| {
                            wheel.remove(index)
                        });

                        if let Some(_) = periodic_tasks.get(&topic) {
                            periodic_tasks.remove(&topic);
                        }
                    }
                    Ok(TimerTaskMsg::Tick) => {
                        let factory = FACTORY.read();
                        let expired = wheel.roll();
                        for (Item { elem, time_point }, idx) in expired {
                            // 如果执行的是周期性任务, 再次添加到时间轮里面
                            if let Some(period) = periodic_tasks.get(&elem) {
                                let cur_time = wheel.get_time();
                                let index = wheel.insert(Item {
                                    elem: elem.clone(),
                                    time_point: cur_time + *period as u64,
                                });
                            }

                            // 通过 elem 找到虚拟机工厂
                            let fact = factory.get(&elem).cloned();
                            if let Some((vmf, dbmgr)) = fact {
                                let real_args = Box::new(move |vm: Arc<JS>| {
                                    let ptr = Box::into_raw(Box::new(dbmgr.clone())) as usize;
                                    ptr_jstype(vm.get_objs(), vm.clone(), ptr, 2976191628);
                                    let _ = vm.new_str(elem.to_string()); // topic 参数
                                    2
                                });
                                vmf.call(
                                    None,
                                    Atom::from("_$timer_task"),
                                    real_args,
                                    Atom::from("timer task"),
                                );
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
        });
}

enum TimerTaskMsg {
    // 新增一次性任务
    InsertOneShotTask { topic: String, after: usize },
    // 新增周期性任务
    InsertPeriodicTask { topic: String, period: usize },
    // 取消任务
    CancelTask { topic: String },
    // 时钟周期
    Tick,
}
