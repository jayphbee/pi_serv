use depend;
use handler;
use httpc;
use js_base;
use js_db;
use js_httpc;
use js_net;
use mqtt;
use net;
use pi_base::task::TaskType;
use pi_db;
use pi_lib;
use pi_lib::atom::Atom;
use pi_p2p;
use pi_store;
use pi_vm;
use pi_vm::adapter::{JSType, JS};
use pi_vm::bonmgr::{jstype_ptr, ptr_jstype, BonMgr, CallResult, FnMeta, StructMeta};
use pi_vm::pi_vm_impl::{block_reply, block_throw, push_callback};
use rpc;
use std::io::Error;
use std::mem::{transmute, uninitialized};
use std::sync::Arc;
use util;

fn call_3763610783_sync(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in next";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3289224548, false, param_error).expect("");
    let jst0 = unsafe { &mut *(ptr as *mut js_db::DBIter) };

    let jscopy = js.clone();
    let call_back = move |r: Result<Option<(Arc<Vec<u8>>, Arc<Vec<u8>>)>, String>| {
        let r = match r {
            Ok(r) => {
                block_reply(
                    jscopy.clone(),
                    Box::new(move |js: Arc<JS>| {
                        let r = match r {
                            Some(v) => {
                                let array = js.new_array();
                                let v_elem = v.0;
                                let ptr = Box::into_raw(Box::new(v_elem)) as usize;
                                let v_elem = ptr_jstype(js.get_objs(), js.clone(), ptr, 2886438122);

                                js.set_index(&array, 0, &v_elem);
                                let v_elem = v.1;
                                let ptr = Box::into_raw(Box::new(v_elem)) as usize;
                                let v_elem = ptr_jstype(js.get_objs(), js.clone(), ptr, 2886438122);

                                js.set_index(&array, 1, &v_elem);
                                let v = array;
                                v
                            }
                            None => js.new_null(),
                        };
                    }),
                    TaskType::Sync,
                    10,
                    Atom::from(""),
                );
            }
            Err(v) => {
                block_throw(
                    jscopy.clone(),
                    v + ", Result is Err",
                    TaskType::Sync,
                    10,
                    Atom::from("block throw task"),
                );
                return;
            }
        };
    };
    let r = js_db::DBIter::next(jst0, Arc::new(call_back));
    if r.is_some() {
        let r = r.unwrap();
        let r = match r {
            Ok(r) => {
                let r = match r {
                    Some(v) => {
                        let array = js.new_array();
                        let v_elem = v.0;
                        let ptr = Box::into_raw(Box::new(v_elem)) as usize;
                        let v_elem = ptr_jstype(js.get_objs(), js.clone(), ptr, 2886438122);

                        js.set_index(&array, 0, &v_elem);
                        let v_elem = v.1;
                        let ptr = Box::into_raw(Box::new(v_elem)) as usize;
                        let v_elem = ptr_jstype(js.get_objs(), js.clone(), ptr, 2886438122);

                        js.set_index(&array, 1, &v_elem);
                        let v = array;
                        v
                    }
                    None => js.new_null(),
                };
                r
            }
            Err(v) => {
                return Some(CallResult::Err(v + ", Result is Err"));
            }
        };

        return Some(CallResult::Ok);
    }
    None
}

fn call_2701929727_sync(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in next_elem";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3289224548, false, param_error).expect("");
    let jst0 = unsafe { &mut *(ptr as *mut js_db::DBIter) };

    let jscopy = js.clone();
    let call_back = move |r: Result<Option<pi_vm::adapter::JSType>, String>| {
        let r = match r {
            Ok(r) => {
                block_reply(
                    jscopy.clone(),
                    Box::new(move |js: Arc<JS>| {
                        let r = match r {
                            Some(v) => {
                                let v = js.new_undefined();
                                v
                            }
                            None => js.new_null(),
                        };
                    }),
                    TaskType::Sync,
                    10,
                    Atom::from(""),
                );
            }
            Err(v) => {
                block_throw(
                    jscopy.clone(),
                    v + ", Result is Err",
                    TaskType::Sync,
                    10,
                    Atom::from("block throw task"),
                );
                return;
            }
        };
    };
    let r = js_db::DBIter::next_elem(jst0, Arc::new(call_back), &js);
    if r.is_some() {
        let r = r.unwrap();
        let r = match r {
            Ok(r) => {
                let r = match r {
                    Some(v) => v,
                    None => js.new_null(),
                };
                r
            }
            Err(v) => {
                return Some(CallResult::Err(v + ", Result is Err"));
            }
        };

        return Some(CallResult::Ok);
    }
    None
}

fn call_1993779671(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in new";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1751456239, true, param_error).expect("");
    let jst0 = *unsafe { Box::from_raw(ptr as *mut mqtt::server::ServerNode) };

    let jst1 = &v[1];
    if !jst1.is_uint8_array() && !jst1.is_array_buffer() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.to_bytes();

    let result = js_db::DBToMqttMonitor::new(jst0, jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 2627601653);

    Some(CallResult::Ok)
}

fn call_1967373661_sync(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in iter_db";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let jst2 = &v[2];
    if !jst2.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst2 = jst2.get_str();

    let jst3 = &v[3];
    let jst3 = if jst3.is_undefined() || jst3.is_null() {
        None
    } else {
        if !jst3.is_uint8_array() && !jst3.is_array_buffer() {
            return Some(CallResult::Err(String::from(param_error)));
        }
        let jst3 = jst3.to_bytes();

        Some(jst3)
    };

    let jst4 = &v[4];
    if !jst4.is_boolean() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst4 = jst4.get_boolean();

    let jst5 = &v[5];
    let jst5 = if jst5.is_undefined() || jst5.is_null() {
        None
    } else {
        if !jst5.is_string() {
            return Some(CallResult::Err(String::from(param_error)));
        }
        let jst5 = jst5.get_str();

        Some(jst5)
    };

    let jscopy = js.clone();
    let call_back = move |r: Result<js_db::DBIter, String>| {
        let r = match r {
            Ok(r) => {
                block_reply(
                    jscopy.clone(),
                    Box::new(move |js: Arc<JS>| {
                        let ptr = Box::into_raw(Box::new(r)) as usize;
                        let r = ptr_jstype(js.get_objs(), js.clone(), ptr, 3289224548);
                    }),
                    TaskType::Sync,
                    10,
                    Atom::from(""),
                );
            }
            Err(v) => {
                block_throw(
                    jscopy.clone(),
                    v + ", Result is Err",
                    TaskType::Sync,
                    10,
                    Atom::from("block throw task"),
                );
                return;
            }
        };
    };
    let r = js_db::iter_db(jst0, jst1, jst2, jst3, jst4, jst5, Arc::new(call_back));
    if r.is_some() {
        let r = r.unwrap();
        let r = match r {
            Ok(r) => {
                let ptr = Box::into_raw(Box::new(r)) as usize;
                let r = ptr_jstype(js.get_objs(), js.clone(), ptr, 3289224548);

                r
            }
            Err(v) => {
                return Some(CallResult::Err(v + ", Result is Err"));
            }
        };

        return Some(CallResult::Ok);
    }
    None
}

fn call_1420275781(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in clone_db_mgr";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };

    let result = js_db::clone_db_mgr(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 2976191628);

    Some(CallResult::Ok)
}

fn call_1905006775(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in register_memery_db";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 1237457629, true, param_error).expect("");
    let jst2 = *unsafe { Box::from_raw(ptr as *mut pi_db::memery_db::DB) };

    let result = js_db::register_memery_db(jst0, jst1, jst2);
    let result = js.new_boolean(result);

    Some(CallResult::Ok)
}

fn call_3038249291(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in register_file_db";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 4204700632, true, param_error).expect("");
    let jst2 = *unsafe { Box::from_raw(ptr as *mut pi_store::db::DB) };

    let result = js_db::register_file_db(jst0, jst1, jst2);
    let result = js.new_boolean(result);

    Some(CallResult::Ok)
}

fn call_2097131752(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in tabkv_with_value";

    let jst0 = &v[0];
    if !jst0.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = &jst0.get_str();

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = &jst1.get_str();

    let jst2 = &v[2];
    if !jst2.is_uint8_array() && !jst2.is_array_buffer() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst2 = jst2.to_bytes();

    let jst3 = &v[3];
    if !jst3.is_uint8_array() && !jst3.is_array_buffer() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst3 = jst3.to_bytes();

    let result = js_db::tabkv_with_value(jst0, jst1, jst2, jst3);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 4000136370);

    Some(CallResult::Ok)
}

fn call_1247562096(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in tabkv_new";

    let jst0 = &v[0];
    if !jst0.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = &jst0.get_str();

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = &jst1.get_str();

    let jst2 = &v[2];
    if !jst2.is_uint8_array() && !jst2.is_array_buffer() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst2 = jst2.to_bytes();

    let result = js_db::tabkv_new(jst0, jst1, jst2);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 4000136370);

    Some(CallResult::Ok)
}

fn call_1579404380(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in tabkv_get_value";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4000136370, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const pi_db::db::TabKV) };

    let result = js_db::tabkv_get_value(jst0);
    let result = match result {
        Some(v) => {
            let ptr = Box::into_raw(Box::new(v)) as usize;
            let v = ptr_jstype(js.get_objs(), js.clone(), ptr, 2886438122);

            v
        }
        None => js.new_null(),
    };

    Some(CallResult::Ok)
}

fn call_2680255887_sync(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in alter";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let jst2 = &v[2];
    if !jst2.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst2 = jst2.get_str();

    let jst3 = &v[3];
    let jst3 = if jst3.is_undefined() || jst3.is_null() {
        None
    } else {
        if !jst3.is_uint8_array() && !jst3.is_array_buffer() {
            return Some(CallResult::Err(String::from(param_error)));
        }
        let jst3 = jst3.to_bytes();

        Some(jst3)
    };

    let jscopy = js.clone();
    let call_back = move |r: Result<(), String>| {
        let r = match r {
            Ok(r) => {
                block_reply(
                    jscopy.clone(),
                    Box::new(move |js: Arc<JS>| {
                        let array = js.new_array();
                        let r = array;
                    }),
                    TaskType::Sync,
                    10,
                    Atom::from(""),
                );
            }
            Err(v) => {
                block_throw(
                    jscopy.clone(),
                    v + ", Result is Err",
                    TaskType::Sync,
                    10,
                    Atom::from("block throw task"),
                );
                return;
            }
        };
    };
    let r = js_db::alter(jst0, jst1, jst2, jst3, Arc::new(call_back));
    if r.is_some() {
        let r = r.unwrap();
        let r = match r {
            Ok(r) => {
                let array = js.new_array();
                let r = array;
                r
            }
            Err(v) => {
                return Some(CallResult::Err(v + ", Result is Err"));
            }
        };

        return Some(CallResult::Ok);
    }
    None
}

fn call_2725879080_sync(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in modify";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };

    let jst1 = &v[1];

    let jst2 = &v[2];
    let jst2 = if jst2.is_undefined() || jst2.is_null() {
        None
    } else {
        if !jst2.is_number() {
            return Some(CallResult::Err(String::from(param_error)));
        }
        let jst2 = jst2.get_u32() as usize;

        Some(jst2)
    };

    let jst3 = &v[3];
    if !jst3.is_boolean() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst3 = jst3.get_boolean();

    let jscopy = js.clone();
    let call_back = move |r: Result<(), String>| {
        let r = match r {
            Ok(r) => {
                block_reply(
                    jscopy.clone(),
                    Box::new(move |js: Arc<JS>| {
                        let array = js.new_array();
                        let r = array;
                    }),
                    TaskType::Sync,
                    10,
                    Atom::from(""),
                );
            }
            Err(v) => {
                block_throw(
                    jscopy.clone(),
                    v + ", Result is Err",
                    TaskType::Sync,
                    10,
                    Atom::from("block throw task"),
                );
                return;
            }
        };
    };
    let r = js_db::modify(jst0, jst1, jst2, jst3, Arc::new(call_back));
    if r.is_some() {
        let r = r.unwrap();
        let r = match r {
            Ok(r) => {
                let array = js.new_array();
                let r = array;
                r
            }
            Err(v) => {
                return Some(CallResult::Err(v + ", Result is Err"));
            }
        };

        return Some(CallResult::Ok);
    }
    None
}

fn call_583163851_sync(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in query";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1754972364, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Tr) };

    let jst1 = &v[1];

    let jst2 = &v[2];
    let jst2 = if jst2.is_undefined() || jst2.is_null() {
        None
    } else {
        if !jst2.is_number() {
            return Some(CallResult::Err(String::from(param_error)));
        }
        let jst2 = jst2.get_u32() as usize;

        Some(jst2)
    };

    let jst3 = &v[3];
    if !jst3.is_boolean() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst3 = jst3.get_boolean();

    let jscopy = js.clone();
    let call_back = move |r: Result<pi_vm::adapter::JSType, String>| {
        let r = match r {
            Ok(r) => {
                block_reply(
                    jscopy.clone(),
                    Box::new(move |js: Arc<JS>| {
                        let r = js.new_undefined();
                    }),
                    TaskType::Sync,
                    10,
                    Atom::from(""),
                );
            }
            Err(v) => {
                block_throw(
                    jscopy.clone(),
                    v + ", Result is Err",
                    TaskType::Sync,
                    10,
                    Atom::from("block throw task"),
                );
                return;
            }
        };
    };
    let r = js_db::query(jst0, jst1, jst2, jst3, Arc::new(call_back), &js);
    if r.is_some() {
        let r = r.unwrap();
        let r = match r {
            Ok(r) => r,
            Err(v) => {
                return Some(CallResult::Err(v + ", Result is Err"));
            }
        };

        return Some(CallResult::Ok);
    }
    None
}

fn call_1869880364(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in register_db_to_mqtt_monitor";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const pi_db::mgr::Mgr) };

    let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 2627601653, true, param_error).expect("");
    let jst1 = *unsafe { Box::from_raw(ptr as *mut js_db::DBToMqttMonitor) };

    js_db::register_db_to_mqtt_monitor(jst0, jst1);
    Some(CallResult::Ok)
}

fn call_2239806005(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in read_file";

    let jst0 = &v[0];
    if !jst0.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = &jst0.get_str();

    let result = util::read_file(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 104530634);

    Some(CallResult::Ok)
}

fn call_2877621631(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in new";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2976191628, true, param_error).expect("");
    let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_db::mgr::Mgr) };

    let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 730519735, true, param_error).expect("");
    let jst1 = *unsafe { Box::from_raw(ptr as *mut pi_vm::pi_vm_impl::VMFactory) };

    let result = handler::JSGray::new(jst0, jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 1493343770);

    Some(CallResult::Ok)
}

fn call_928306607(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in set_obj";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1493343770, false, param_error).expect("");
    let jst0 = unsafe { &mut *(ptr as *mut handler::JSGray) };

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let jst2 = &v[2];

    let jst3 = &v[3];
    if !jst3.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst3 = jst3.get_str();

    let jst4 = &v[4];
    if !jst4.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst4 = jst4.get_str();

    let result = handler::JSGray::set_obj(jst0, jst1, jst2, jst3, jst4, &js);
    let result = match result {
        Ok(r) => {
            let r = js.new_boolean(r);
            r
        }
        Err(v) => {
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}

fn call_3133367430(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in new";

    let jst0 = &v[0];
    if !jst0.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.get_u32() as usize;

    let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1493343770, true, param_error).expect("");
    let jst1 = *unsafe { Box::from_raw(ptr as *mut handler::JSGray) };

    let result = handler::TopicHandler::new(jst0, jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 585492653);

    Some(CallResult::Ok)
}

fn call_1272003303(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in new";

    let jst0 = &v[0];
    if !jst0.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.get_u32() as usize;

    let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1493343770, true, param_error).expect("");
    let jst1 = *unsafe { Box::from_raw(ptr as *mut handler::JSGray) };

    let result = handler::AsyncRequestHandler::new(jst0, jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 3366364668);

    Some(CallResult::Ok)
}

fn call_1347190475(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in create_sinfo";

    let jst0 = &v[0];
    if !jst0.is_uint8_array() && !jst0.is_array_buffer() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.to_bytes();

    let result = js_base::create_sinfo(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 1721307497);

    Some(CallResult::Ok)
}

fn call_3993207385(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in clone_vm_factory";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 730519735, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const pi_vm::pi_vm_impl::VMFactory) };

    let result = js_base::clone_vm_factory(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 730519735);

    Some(CallResult::Ok)
}

fn call_4111533257(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in arc_new_async_request_handler";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3366364668, true, param_error).expect("");
    let jst0 = *unsafe { Box::from_raw(ptr as *mut handler::AsyncRequestHandler) };

    let result = js_base::arc_new_async_request_handler(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 1012806580);

    Some(CallResult::Ok)
}

fn call_3272869145(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in register_async_handler";

    let jst0 = &v[0];
    if !jst0.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.get_str();

    let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1012806580, false, param_error).expect("");
    let jst1 = unsafe { &*(ptr as *const Arc<handler::AsyncRequestHandler>) };

    js_base::register_async_handler(jst0, jst1);
    Some(CallResult::Ok)
}

fn call_3741531906(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in arc_deref";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2886438122, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const Arc<Vec<u8>>) };

    let result = js_base::arc_deref(jst0);
    let ptr = result as *const Vec<u8> as usize;
    let result = ptr_jstype(js.get_objs_ref(), js.clone(), ptr, 104530634);

    Some(CallResult::Ok)
}

fn call_509141093(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in get_depend";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1797798710, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const depend::Depend) };

    let jst1 = &v[1];
    if !jst1.is_array() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let a_len = jst1.get_array_length();

    let mut jst1_ = Vec::new();
    for i in 0..a_len {
        let jst1_e = jst1.get_index(i as u32);
        if !jst1_e.is_string() {
            return Some(CallResult::Err(String::from(param_error)));
        }
        let jst1_e = jst1_e.get_str();
        jst1_.push(jst1_e);
    }
    let jst1 = jst1_.as_slice();

    let result = js_base::get_depend(jst0, jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 1542823015);

    Some(CallResult::Ok)
}

fn call_1810043215_sync(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in sleep";

    let jst0 = &v[0];
    if !jst0.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.get_u32();

    let jscopy = js.clone();
    let call_back = move || {
        block_reply(
            jscopy.clone(),
            Box::new(move |js: Arc<JS>| {
                js.new_null();
            }),
            TaskType::Sync,
            10,
            Atom::from(""),
        );
    };
    js_base::sleep(jst0, Box::new(call_back));
    None
}

fn call_3344344275_async(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in set_timeout";
    let jst0 = &v[0];
    if !jst0.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.get_u32();

    let call_index = &v[1];
    if !call_index.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let call_index = call_index.get_u32();

    let jscopy = js.clone();
    let call_back = move || {
        push_callback(
            jscopy.clone(),
            call_index,
            Box::new(move |js: Arc<JS>| 0),
            Atom::from(""),
        );
    };

    let result = js_base::set_timeout(jst0, Box::new(call_back));
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 1496374710);

    Some(CallResult::Ok)
}

fn call_3285798497(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in clear_timeout";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1496374710, true, param_error).expect("");
    let jst0 = *unsafe { Box::from_raw(ptr as *mut js_base::AtomIndex) };

    js_base::clear_timeout(jst0);
    Some(CallResult::Ok)
}

fn call_59144274(js: Arc<JS>) -> Option<CallResult> {
    let result = js_base::create_rand();
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 3355421248);

    Some(CallResult::Ok)
}

fn call_3881780156(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in next_u32";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3355421248, false, param_error).expect("");
    let jst0 = unsafe { &mut *(ptr as *mut js_base::Rand) };

    let result = js_base::next_u32(jst0);
    let result = js.new_u32(result);

    Some(CallResult::Ok)
}

fn call_3908949488(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in next_u64";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3355421248, false, param_error).expect("");
    let jst0 = unsafe { &mut *(ptr as *mut js_base::Rand) };

    let result = js_base::next_u64(jst0);
    let result = js.new_u64(result);

    Some(CallResult::Ok)
}

fn call_2556550051(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in fill_bytes";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3355421248, false, param_error).expect("");
    let jst0 = unsafe { &mut *(ptr as *mut js_base::Rand) };

    let jst1 = &v[1];
    if !jst1.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_u32() as usize;

    let result = js_base::fill_bytes(jst0, jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 104530634);

    Some(CallResult::Ok)
}

fn call_957759389(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in try_fill_bytes";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3355421248, false, param_error).expect("");
    let jst0 = unsafe { &mut *(ptr as *mut js_base::Rand) };

    let jst1 = &v[1];
    if !jst1.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_u32() as usize;

    let result = js_base::try_fill_bytes(jst0, jst1);
    let result = match result {
        Ok(r) => {
            let ptr = Box::into_raw(Box::new(r)) as usize;
            let r = ptr_jstype(js.get_objs(), js.clone(), ptr, 104530634);

            r
        }
        Err(v) => {
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}

fn call_370495443(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in drop_native_obj";

    let jst0 = &v[0];

    let result = js_base::drop_native_obj(jst0, &js);
    let result = match result {
        Ok(r) => {
            let r = js.new_boolean(r);
            r
        }
        Err(v) => {
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}

fn call_373179692(js: Arc<JS>) -> Option<CallResult> {
    let result = js_httpc::HttpClientOptions::default();
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 1131624585);

    Some(CallResult::Ok)
}

fn call_145125716(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in normal";

    let jst0 = &v[0];
    if !jst0.is_boolean() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.get_boolean();

    let jst1 = &v[1];
    if !jst1.is_boolean() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_boolean();

    let jst2 = &v[2];
    if !jst2.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst2 = jst2.get_i32() as isize;

    let jst3 = &v[3];
    if !jst3.is_uint8_array() && !jst3.is_array_buffer() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let arr = unsafe { *(jst3.to_bytes().as_ptr() as usize as *const [u8; 8]) };
    let jst3 = unsafe { transmute::<[u8; 8], u64>(arr) };

    let result = js_httpc::HttpClientOptions::normal(jst0, jst1, jst2, jst3);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 1131624585);

    Some(CallResult::Ok)
}

fn call_2887071833(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in vaild_host";

    let jst0 = &v[0];
    if !jst0.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.get_str();

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let jst2 = &v[2];
    if !jst2.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst2 = jst2.get_str();

    let jst3 = &v[3];
    if !jst3.is_boolean() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst3 = jst3.get_boolean();

    let jst4 = &v[4];
    if !jst4.is_boolean() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst4 = jst4.get_boolean();

    let jst5 = &v[5];
    if !jst5.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst5 = jst5.get_i32() as isize;

    let jst6 = &v[6];
    if !jst6.is_uint8_array() && !jst6.is_array_buffer() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let arr = unsafe { *(jst6.to_bytes().as_ptr() as usize as *const [u8; 8]) };
    let jst6 = unsafe { transmute::<[u8; 8], u64>(arr) };

    let result = js_httpc::HttpClientOptions::vaild_host(jst0, jst1, jst2, jst3, jst4, jst5, jst6);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 1131624585);

    Some(CallResult::Ok)
}

fn call_2011091417(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in proxy";

    let jst0 = &v[0];
    if !jst0.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.get_str();

    let jst1 = &v[1];
    if !jst1.is_boolean() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_boolean();

    let jst2 = &v[2];
    if !jst2.is_boolean() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst2 = jst2.get_boolean();

    let jst3 = &v[3];
    if !jst3.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst3 = jst3.get_i32() as isize;

    let jst4 = &v[4];
    if !jst4.is_uint8_array() && !jst4.is_array_buffer() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let arr = unsafe { *(jst4.to_bytes().as_ptr() as usize as *const [u8; 8]) };
    let jst4 = unsafe { transmute::<[u8; 8], u64>(arr) };

    let result = js_httpc::HttpClientOptions::proxy(jst0, jst1, jst2, jst3, jst4);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 1131624585);

    Some(CallResult::Ok)
}

fn call_2937777264(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in valid_host_proxy";

    let jst0 = &v[0];
    if !jst0.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.get_str();

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let jst2 = &v[2];
    if !jst2.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst2 = jst2.get_str();

    let jst3 = &v[3];
    if !jst3.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst3 = jst3.get_str();

    let jst4 = &v[4];
    if !jst4.is_boolean() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst4 = jst4.get_boolean();

    let jst5 = &v[5];
    if !jst5.is_boolean() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst5 = jst5.get_boolean();

    let jst6 = &v[6];
    if !jst6.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst6 = jst6.get_i32() as isize;

    let jst7 = &v[7];
    if !jst7.is_uint8_array() && !jst7.is_array_buffer() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let arr = unsafe { *(jst7.to_bytes().as_ptr() as usize as *const [u8; 8]) };
    let jst7 = unsafe { transmute::<[u8; 8], u64>(arr) };

    let result = js_httpc::HttpClientOptions::valid_host_proxy(
        jst0, jst1, jst2, jst3, jst4, jst5, jst6, jst7,
    );
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 1131624585);

    Some(CallResult::Ok)
}

fn call_2175286088(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in get_json_val";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4139279264, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const js_httpc::HttpClientBody<Vec<u8>>) };

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let result = js_httpc::HttpClientBody::<Vec<u8>>::get_json_val(jst0, jst1);
    let result = match result {
        Some(v) => {
            let v = js.new_str(String::from(v.as_str()));

            v
        }
        None => js.new_null(),
    };

    Some(CallResult::Ok)
}

fn call_1065006446(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in add_json_kv";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4139279264, false, param_error).expect("");
    let jst0 = unsafe { &mut *(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let jst2 = &v[2];
    if !jst2.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst2 = jst2.get_str();

    let result = js_httpc::HttpClientBody::<Vec<u8>>::add_json_kv(jst0, jst1, jst2);
    let result = js.new_u32(result as u32);

    Some(CallResult::Ok)
}

fn call_1500292772(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in remove_json_kv";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4139279264, false, param_error).expect("");
    let jst0 = unsafe { &mut *(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let result = js_httpc::HttpClientBody::<Vec<u8>>::remove_json_kv(jst0, jst1);
    let result = match result {
        Some(v) => {
            let v = js.new_str(v);

            v
        }
        None => js.new_null(),
    };

    Some(CallResult::Ok)
}

fn call_2345066455(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in clear_json_kvs";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4139279264, false, param_error).expect("");
    let jst0 = unsafe { &mut *(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };

    js_httpc::HttpClientBody::<Vec<u8>>::clear_json_kvs(jst0);
    Some(CallResult::Ok)
}

fn call_1016322459(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in add_form_kv";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4139279264, true, param_error).expect("");
    let jst0 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let jst2 = &v[2];
    if !jst2.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst2 = jst2.get_str();

    let result: js_httpc::HttpClientBody<Vec<u8>> =
        js_httpc::HttpClientBody::<Vec<u8>>::add_form_kv(jst0, jst1, jst2);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 4139279264);

    Some(CallResult::Ok)
}

fn call_2344044784(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in add_form_file";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4139279264, true, param_error).expect("");
    let jst0 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let jst2 = &v[2];
    if !jst2.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst2 = jst2.get_str();

    let result = js_httpc::HttpClientBody::<Vec<u8>>::add_form_file(jst0, jst1, jst2);
    let result = match result {
        Ok(r) => {
            let ptr = Box::into_raw(Box::new(r)) as usize;
            let r = ptr_jstype(js.get_objs(), js.clone(), ptr, 4139279264);

            r
        }
        Err(v) => {
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}

fn call_2113618061(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in body";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, true, param_error).expect("");
    let jst0 = *unsafe { Box::from_raw(ptr as *mut Vec<u8>) };

    let result: js_httpc::HttpClientBody<Vec<u8>> = js_httpc::HttpClientBody::<Vec<u8>>::body(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 4139279264);

    Some(CallResult::Ok)
}

fn call_794872933(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in body";

    let jst0 = &v[0];
    if !jst0.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.get_str();

    let result: js_httpc::HttpClientBody<String> = js_httpc::HttpClientBody::<String>::body(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 3642917301);

    Some(CallResult::Ok)
}

fn call_965054041(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in json";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1411051473, true, param_error).expect("");
    let jst0 = *unsafe { Box::from_raw(ptr as *mut pi_lib::atom::Atom) };

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let result: js_httpc::HttpClientBody<String> =
        js_httpc::HttpClientBody::<String>::json(jst0, jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 3642917301);

    Some(CallResult::Ok)
}

fn call_2118843620(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in form";

    let jst0 = &v[0];
    if !jst0.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.get_str();

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let result: js_httpc::HttpClientBody<String> =
        js_httpc::HttpClientBody::<String>::form(jst0, jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 3642917301);

    Some(CallResult::Ok)
}

fn call_997239765(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in create_http_client";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1131624585, true, param_error).expect("");
    let jst0 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientOptions) };

    let result = js_httpc::create_http_client(jst0);
    let result = match result {
        Ok(r) => {
            let ptr = Box::into_raw(Box::new(r)) as usize;
            let r = ptr_jstype(js.get_objs(), js.clone(), ptr, 1107924793);

            r
        }
        Err(v) => {
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}

fn call_739596726_async(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in get";
    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1107924793, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const Arc<httpc::HttpClient>) };

    let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1411051473, true, param_error).expect("");
    let jst1 = *unsafe { Box::from_raw(ptr as *mut pi_lib::atom::Atom) };

    let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 4139279264, true, param_error).expect("");
    let jst2 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };

    let call_index = &v[3];
    if !call_index.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let call_index = call_index.get_u32();

    let jscopy = js.clone();
    let call_back = move |r: (
        Arc<httpc::HttpClient>,
        Result<httpc::HttpClientResponse, String>,
    )| {
        push_callback(
            jscopy.clone(),
            call_index,
            Box::new(move |js: Arc<JS>| {
                let array = js.new_array();
                let r_elem = r.0;
                let ptr = Box::into_raw(Box::new(r_elem)) as usize;
                let r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr, 1107924793);

                js.set_index(&array, 0, &r_elem);
                let r_elem = r.1;
                let r_elem = match r_elem {
                    Ok(r) => {
                        let ptr = Box::into_raw(Box::new(r)) as usize;
                        let r = ptr_jstype(js.get_objs(), js.clone(), ptr, 606449873);

                        r
                    }
                    Err(v) => js.new_str(v + ", Result is Err"),
                };
                js.set_index(&array, 1, &r_elem);
                let r = array;

                1
            }),
            Atom::from(""),
        );
    };

    js_httpc::get(jst0, jst1, jst2, Box::new(call_back));
    Some(CallResult::Ok)
}

fn call_2173630691_async(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in get";
    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1107924793, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const Arc<httpc::HttpClient>) };

    let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1411051473, true, param_error).expect("");
    let jst1 = *unsafe { Box::from_raw(ptr as *mut pi_lib::atom::Atom) };

    let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 3642917301, true, param_error).expect("");
    let jst2 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<String>) };

    let call_index = &v[3];
    if !call_index.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let call_index = call_index.get_u32();

    let jscopy = js.clone();
    let call_back = move |r: (
        Arc<httpc::HttpClient>,
        Result<httpc::HttpClientResponse, String>,
    )| {
        push_callback(
            jscopy.clone(),
            call_index,
            Box::new(move |js: Arc<JS>| {
                let array = js.new_array();
                let r_elem = r.0;
                let ptr = Box::into_raw(Box::new(r_elem)) as usize;
                let r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr, 1107924793);

                js.set_index(&array, 0, &r_elem);
                let r_elem = r.1;
                let r_elem = match r_elem {
                    Ok(r) => {
                        let ptr = Box::into_raw(Box::new(r)) as usize;
                        let r = ptr_jstype(js.get_objs(), js.clone(), ptr, 606449873);

                        r
                    }
                    Err(v) => js.new_str(v + ", Result is Err"),
                };
                js.set_index(&array, 1, &r_elem);
                let r = array;

                1
            }),
            Atom::from(""),
        );
    };

    js_httpc::get(jst0, jst1, jst2, Box::new(call_back));
    Some(CallResult::Ok)
}

fn call_1358301807_async(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in post";
    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1107924793, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const Arc<httpc::HttpClient>) };

    let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1411051473, true, param_error).expect("");
    let jst1 = *unsafe { Box::from_raw(ptr as *mut pi_lib::atom::Atom) };

    let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 4139279264, true, param_error).expect("");
    let jst2 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };

    let call_index = &v[3];
    if !call_index.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let call_index = call_index.get_u32();

    let jscopy = js.clone();
    let call_back = move |r: (
        Arc<httpc::HttpClient>,
        Result<httpc::HttpClientResponse, String>,
    )| {
        push_callback(
            jscopy.clone(),
            call_index,
            Box::new(move |js: Arc<JS>| {
                let array = js.new_array();
                let r_elem = r.0;
                let ptr = Box::into_raw(Box::new(r_elem)) as usize;
                let r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr, 1107924793);

                js.set_index(&array, 0, &r_elem);
                let r_elem = r.1;
                let r_elem = match r_elem {
                    Ok(r) => {
                        let ptr = Box::into_raw(Box::new(r)) as usize;
                        let r = ptr_jstype(js.get_objs(), js.clone(), ptr, 606449873);

                        r
                    }
                    Err(v) => js.new_str(v + ", Result is Err"),
                };
                js.set_index(&array, 1, &r_elem);
                let r = array;

                1
            }),
            Atom::from(""),
        );
    };

    js_httpc::post(jst0, jst1, jst2, Box::new(call_back));
    Some(CallResult::Ok)
}

fn call_3423707807_async(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in post";
    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1107924793, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const Arc<httpc::HttpClient>) };

    let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1411051473, true, param_error).expect("");
    let jst1 = *unsafe { Box::from_raw(ptr as *mut pi_lib::atom::Atom) };

    let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 3642917301, true, param_error).expect("");
    let jst2 = *unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<String>) };

    let call_index = &v[3];
    if !call_index.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let call_index = call_index.get_u32();

    let jscopy = js.clone();
    let call_back = move |r: (
        Arc<httpc::HttpClient>,
        Result<httpc::HttpClientResponse, String>,
    )| {
        let r = push_callback(
            jscopy.clone(),
            call_index,
            Box::new(move |js: Arc<JS>| {
                let array = js.new_array();
                let r_elem = r.0;
                let ptr = Box::into_raw(Box::new(r_elem)) as usize;
                let r_elem = ptr_jstype(js.get_objs(), js.clone(), ptr, 1107924793);

                js.set_index(&array, 0, &r_elem);
                let r_elem = r.1;
                let r_elem = match r_elem {
                    Ok(r) => {
                        let ptr = Box::into_raw(Box::new(r)) as usize;
                        let r = ptr_jstype(js.get_objs(), js.clone(), ptr, 606449873);

                        r
                    }
                    Err(v) => js.new_str(v + ", Result is Err"),
                };
                js.set_index(&array, 1, &r_elem);
                let r = array;

                1
            }),
            Atom::from(""),
        );
    };
    js_httpc::post(jst0, jst1, jst2, Box::new(call_back));
    Some(CallResult::Ok)
}

fn call_357009886(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in mqtt_bind";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2761082466, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const net::api::NetManager) };

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let jst2 = &v[2];
    if !jst2.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst2 = jst2.get_str();

    let jst3 = &v[3];
    if !jst3.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst3 = jst3.get_u32() as usize;

    let jst4 = &v[4];
    if !jst4.is_number() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst4 = jst4.get_u32() as usize;

    let result = js_net::mqtt_bind(jst0, jst1, jst2, jst3, jst4);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 1751456239);

    Some(CallResult::Ok)
}

fn call_2248917003(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in clone_server_node";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1751456239, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const mqtt::server::ServerNode) };

    let result = js_net::clone_server_node(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 1751456239);

    Some(CallResult::Ok)
}

fn call_2482429183(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in set_mqtt_topic";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1751456239, true, param_error).expect("");
    let jst0 = *unsafe { Box::from_raw(ptr as *mut mqtt::server::ServerNode) };

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let jst2 = &v[2];
    if !jst2.is_boolean() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst2 = jst2.get_boolean();

    let jst3 = &v[3];
    if !jst3.is_boolean() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst3 = jst3.get_boolean();

    let result = js_net::set_mqtt_topic(jst0, jst1, jst2, jst3);
    let result = match result {
        Ok(r) => {
            let r = js.new_boolean(r);
            r
        }
        Err(v) => {
            return Some(CallResult::Err(v + ", Result is Err"));
        }
    };

    Some(CallResult::Ok)
}

fn call_2874114884(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in mqtt_respond";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 226971089, false, param_error).expect("");
    let jst0 = unsafe { &*(ptr as *const Arc<mqtt::session::Session>) };

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let jst2 = &v[2];
    if !jst2.is_uint8_array() && !jst2.is_array_buffer() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst2 = jst2.to_bytes();

    js_net::mqtt_respond(jst0, jst1, jst2);
    Some(CallResult::Ok)
}

fn call_138660483(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in register_rpc_handler";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1285687456, false, param_error).expect("");
    let jst0 = unsafe { &mut *(ptr as *mut rpc::server::RPCServer) };

    let jst1 = &v[1];
    if !jst1.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst1 = jst1.get_str();

    let jst2 = &v[2];
    if !jst2.is_boolean() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst2 = jst2.get_boolean();

    let jst3 = &v[3];
    let ptr = jstype_ptr(&jst3, js.clone(), 471985604, false, param_error).expect("");
    let jst3 = unsafe { &*(ptr as *const Arc<handler::TopicHandler>) };

    let result = js_net::register_rpc_handler(jst0, jst1, jst2, jst3);
    let result = match result {
        Ok(r) => {
            let array = js.new_array();
            let r = array;
            r
        }
        Err(v) => {
            return Some(CallResult::Err(v.to_string() + "Result is Err"));
        }
    };

    Some(CallResult::Ok)
}

fn call_527952504(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in arc_new_topic_handler";

    let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 585492653, true, param_error).expect("");
    let jst0 = *unsafe { Box::from_raw(ptr as *mut handler::TopicHandler) };

    let result = js_net::arc_new_topic_handler(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 471985604);

    Some(CallResult::Ok)
}

fn call_3189305086(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in p2p_manage_new";

    let jst0 = &v[0];
    if !jst0.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = &jst0.get_str();

    let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1542823015, true, param_error).expect("");
    let jst1 = *unsafe { Box::from_raw(ptr as *mut Vec<String>) };

    let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 1662444400, true, param_error).expect("");
    let jst2 = *unsafe { Box::from_raw(ptr as *mut Vec<u32>) };

    let result = js_net::p2p_manage_new(jst0, jst1, jst2);
    let ptr = Box::into_raw(Box::new(result)) as usize;
    let result = ptr_jstype(js.get_objs(), js.clone(), ptr, 3035778520);

    Some(CallResult::Ok)
}

fn drop_3289224548(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut js_db::DBIter) };
}

fn drop_2886438122(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut Arc<Vec<u8>>) };
}

fn drop_4252329727(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut pi_vm::adapter::JSType) };
}

fn drop_1751456239(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut mqtt::server::ServerNode) };
}

fn drop_2627601653(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut js_db::DBToMqttMonitor) };
}

fn drop_1754972364(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut pi_db::mgr::Tr) };
}

fn drop_2976191628(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut pi_db::mgr::Mgr) };
}

fn drop_1237457629(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut pi_db::memery_db::DB) };
}

fn drop_4204700632(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut pi_store::db::DB) };
}

fn drop_4000136370(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut pi_db::db::TabKV) };
}

fn drop_104530634(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut Vec<u8>) };
}

fn drop_730519735(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut pi_vm::pi_vm_impl::VMFactory) };
}

fn drop_1493343770(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut handler::JSGray) };
}

fn drop_585492653(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut handler::TopicHandler) };
}

fn drop_3366364668(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut handler::AsyncRequestHandler) };
}

fn drop_1721307497(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut Arc<pi_lib::sinfo::StructInfo>) };
}

fn drop_1012806580(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut Arc<handler::AsyncRequestHandler>) };
}

fn drop_1797798710(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut depend::Depend) };
}

fn drop_1542823015(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut Vec<String>) };
}

fn drop_1496374710(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut js_base::AtomIndex) };
}

fn drop_3355421248(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut js_base::Rand) };
}

fn drop_1131624585(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientOptions) };
}

fn drop_4139279264(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<Vec<u8>>) };
}

fn drop_3642917301(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut js_httpc::HttpClientBody<String>) };
}

fn drop_1411051473(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut pi_lib::atom::Atom) };
}

fn drop_1107924793(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut Arc<httpc::HttpClient>) };
}

fn drop_606449873(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut httpc::HttpClientResponse) };
}

fn drop_2761082466(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut net::api::NetManager) };
}

fn drop_226971089(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut Arc<mqtt::session::Session>) };
}

fn drop_1285687456(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut rpc::server::RPCServer) };
}

fn drop_471985604(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut Arc<handler::TopicHandler>) };
}

fn drop_1662444400(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut Vec<u32>) };
}

fn drop_3035778520(ptr: usize) {
    unsafe { Box::from_raw(ptr as *mut pi_p2p::manage::P2PManage) };
}
pub fn register(mgr: &BonMgr) {
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("js_db::DBIter"),
            drop_fn: drop_3289224548,
        },
        3289224548,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("Arc<Vec<u8>>"),
            drop_fn: drop_2886438122,
        },
        2886438122,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("pi_vm::adapter::JSType"),
            drop_fn: drop_4252329727,
        },
        4252329727,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("mqtt::server::ServerNode"),
            drop_fn: drop_1751456239,
        },
        1751456239,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("js_db::DBToMqttMonitor"),
            drop_fn: drop_2627601653,
        },
        2627601653,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("pi_db::mgr::Tr"),
            drop_fn: drop_1754972364,
        },
        1754972364,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("pi_db::mgr::Mgr"),
            drop_fn: drop_2976191628,
        },
        2976191628,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("pi_db::memery_db::DB"),
            drop_fn: drop_1237457629,
        },
        1237457629,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("pi_store::db::DB"),
            drop_fn: drop_4204700632,
        },
        4204700632,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("pi_db::db::TabKV"),
            drop_fn: drop_4000136370,
        },
        4000136370,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("Vec<u8>"),
            drop_fn: drop_104530634,
        },
        104530634,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("pi_vm::pi_vm_impl::VMFactory"),
            drop_fn: drop_730519735,
        },
        730519735,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("handler::JSGray"),
            drop_fn: drop_1493343770,
        },
        1493343770,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("handler::TopicHandler"),
            drop_fn: drop_585492653,
        },
        585492653,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("handler::AsyncRequestHandler"),
            drop_fn: drop_3366364668,
        },
        3366364668,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("Arc<pi_lib::sinfo::StructInfo>"),
            drop_fn: drop_1721307497,
        },
        1721307497,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("Arc<handler::AsyncRequestHandler>"),
            drop_fn: drop_1012806580,
        },
        1012806580,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("depend::Depend"),
            drop_fn: drop_1797798710,
        },
        1797798710,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("Vec<String>"),
            drop_fn: drop_1542823015,
        },
        1542823015,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("js_base::AtomIndex"),
            drop_fn: drop_1496374710,
        },
        1496374710,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("js_base::Rand"),
            drop_fn: drop_3355421248,
        },
        3355421248,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("js_httpc::HttpClientOptions"),
            drop_fn: drop_1131624585,
        },
        1131624585,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("js_httpc::HttpClientBody<Vec<u8>>"),
            drop_fn: drop_4139279264,
        },
        4139279264,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("js_httpc::HttpClientBody<String>"),
            drop_fn: drop_3642917301,
        },
        3642917301,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("pi_lib::atom::Atom"),
            drop_fn: drop_1411051473,
        },
        1411051473,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("Arc<httpc::HttpClient>"),
            drop_fn: drop_1107924793,
        },
        1107924793,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("httpc::HttpClientResponse"),
            drop_fn: drop_606449873,
        },
        606449873,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("net::api::NetManager"),
            drop_fn: drop_2761082466,
        },
        2761082466,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("Arc<mqtt::session::Session>"),
            drop_fn: drop_226971089,
        },
        226971089,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("rpc::server::RPCServer"),
            drop_fn: drop_1285687456,
        },
        1285687456,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("Arc<handler::TopicHandler>"),
            drop_fn: drop_471985604,
        },
        471985604,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("Vec<u32>"),
            drop_fn: drop_1662444400,
        },
        1662444400,
    );
    mgr.regist_struct_meta(
        StructMeta {
            name: String::from("pi_p2p::manage::P2PManage"),
            drop_fn: drop_3035778520,
        },
        3035778520,
    );
    mgr.regist_fun_meta(FnMeta::CallArg(call_3763610783_sync), 3763610783);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2701929727_sync), 2701929727);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1993779671), 1993779671);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1967373661_sync), 1967373661);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1420275781), 1420275781);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1905006775), 1905006775);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3038249291), 3038249291);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2097131752), 2097131752);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1247562096), 1247562096);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1579404380), 1579404380);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2680255887_sync), 2680255887);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2725879080_sync), 2725879080);
    mgr.regist_fun_meta(FnMeta::CallArg(call_583163851_sync), 583163851);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1869880364), 1869880364);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2239806005), 2239806005);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2877621631), 2877621631);
    mgr.regist_fun_meta(FnMeta::CallArg(call_928306607), 928306607);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3133367430), 3133367430);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1272003303), 1272003303);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1347190475), 1347190475);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3993207385), 3993207385);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4111533257), 4111533257);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3272869145), 3272869145);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3741531906), 3741531906);
    mgr.regist_fun_meta(FnMeta::CallArg(call_509141093), 509141093);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1810043215_sync), 1810043215);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3344344275_async), 3344344275);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3285798497), 3285798497);
    mgr.regist_fun_meta(FnMeta::Call(call_59144274), 59144274);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3881780156), 3881780156);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3908949488), 3908949488);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2556550051), 2556550051);
    mgr.regist_fun_meta(FnMeta::CallArg(call_957759389), 957759389);
    mgr.regist_fun_meta(FnMeta::CallArg(call_370495443), 370495443);
    mgr.regist_fun_meta(FnMeta::Call(call_373179692), 373179692);
    mgr.regist_fun_meta(FnMeta::CallArg(call_145125716), 145125716);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2887071833), 2887071833);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2011091417), 2011091417);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2937777264), 2937777264);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2175286088), 2175286088);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1065006446), 1065006446);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1500292772), 1500292772);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2345066455), 2345066455);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1016322459), 1016322459);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2344044784), 2344044784);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2113618061), 2113618061);
    mgr.regist_fun_meta(FnMeta::CallArg(call_794872933), 794872933);
    mgr.regist_fun_meta(FnMeta::CallArg(call_965054041), 965054041);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2118843620), 2118843620);
    mgr.regist_fun_meta(FnMeta::CallArg(call_997239765), 997239765);
    mgr.regist_fun_meta(FnMeta::CallArg(call_739596726_async), 739596726);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2173630691_async), 2173630691);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1358301807_async), 1358301807);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3423707807_async), 3423707807);
    mgr.regist_fun_meta(FnMeta::CallArg(call_357009886), 357009886);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2248917003), 2248917003);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2482429183), 2482429183);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2874114884), 2874114884);
    mgr.regist_fun_meta(FnMeta::CallArg(call_138660483), 138660483);
    mgr.regist_fun_meta(FnMeta::CallArg(call_527952504), 527952504);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3189305086), 3189305086);
}
