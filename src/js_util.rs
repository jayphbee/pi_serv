use std::sync::Arc;
use std::mem::transmute_copy;

use pi_vm::adapter::{JSType, JS, dukc_pop};
use sinfo::{StructInfo, EnumType, EnumInfo};
use lib_util::err_map;
use bon::{ReadBuffer, Decode};
use pi_db::db::{TabKV, TabMeta};


pub fn decode_by_sinfo(js: &Arc<JS>, bon: &mut ReadBuffer, sinfo: &StructInfo) -> Result<JSType, String> {
    let name = sinfo.name.as_str();
    match name {
        "" => { //该类型为元组
            let arr = js.new_array();
            for v in sinfo.fields.iter(){
                let name = match v.name.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        unsafe { dukc_pop(js.get_vm()) };
                        return Err(format!("String cannot be converted to digits, field:{:?}, struct:{}", v.name, name));
                    },
                };
                let mut value = match decode_by_type(js, bon, &v.ftype) {
                    Ok(v) => v,
                    Err(s) => {
                        unsafe { dukc_pop(js.get_vm()) };
                        return Err(s);
                    },
                };
                js.set_index(&arr, name, &mut value);
            }
            return Ok(arr);
        },
        "_$Json" => {//一个普通的Json
            let obj = js.new_object();
            for v in sinfo.fields.iter(){
                let mut value = match decode_by_type(js, bon, &v.ftype) {
                    Ok(v) => v,
                    Err(s) => {
                        unsafe { dukc_pop(js.get_vm()) };
                        return Err(s);
                    },
                };
                js.set_field(&obj, String::from(v.name.as_str()), &mut value);
            }
            return Ok(obj);
        }
        _ => (),
    };

    let index = match name.find("."){
        Some(v) => v,
        None => panic!("illegal module name, lack '.', modName: {}", name),
    };
    let r = name.split_at(index);// r.0为模块名， r.1为类型名称;
    let type_name = String::from("pi_modules['") + r.0 + ".s']" + ".exports" + r.1;
    js.get_type(type_name.clone());
    let obj = js.new_type(type_name.clone(), 0);
    if obj.is_undefined(){
        unsafe { dukc_pop(js.get_vm()) };
        return Err(String::from("module is not exist, please make sure the module has been loaded, modName:")+ &type_name);
    }

    for v in sinfo.fields.iter(){
        let mut value = match decode_by_type(js, bon, &v.ftype) {
            Ok(v) => v,
            Err(s) => {
                unsafe { dukc_pop(js.get_vm()) };
                return Err(s);
            },
        };
        js.set_field(&obj, String::from(v.name.as_str()), &mut value);
    }
    Ok(obj)
}

pub fn decode_by_enuminfo(js: &Arc<JS>, bon: &mut ReadBuffer, einfo: &EnumInfo) -> Result<JSType, String> {
    let name = einfo.name.as_str();
    let index = match name.find("."){
        Some(v) => v,
        None => panic!("illegal module name, lack '.', modName: {}", name),
    };
    let r = name.split_at(index);// r.0为模块名， r.1为类型名称;
    let type_name = String::from("pi_modules['") + r.0 + ".s']" + ".exports" + r.1;
    js.get_type(type_name.clone());
    let obj = js.new_type(type_name.clone(), 0);
    if obj.is_undefined(){
        unsafe { dukc_pop(js.get_vm()) };
        return Err(String::from("module is not exist, please make sure the module has been loaded, modName:")+ &type_name);
    }

    let index = err_map(usize::decode(bon))?;
    js.set_field(&obj, String::from("enum_type"), &mut js.new_u8(index as u8));
    let t = &einfo.members[index - 1];
    match t {
        &Some(ref ftype) => {
            let mut value = match decode_by_type(js, bon, &ftype) {
                Ok(v) => v,
                Err(s) => {
                    unsafe { dukc_pop(js.get_vm()) };
                    return Err(s);
                },
            };
            js.set_field(&obj, String::from("value"), &mut value);
        },
        None => (),
    }
    Ok(obj)
}

pub fn decode_by_type(js: &Arc<JS>, bon: &mut ReadBuffer, t: &EnumType) -> Result<JSType, String> {
    let r = match t {
        EnumType::Bool => js.new_boolean(err_map(bool::decode(bon))?),
        EnumType::U8 => js.new_u8(err_map(u8::decode(bon))?),
        EnumType::U16 => js.new_u16(err_map(u16::decode(bon))?),
        EnumType::U32 => js.new_u32(err_map(u32::decode(bon))?),
        EnumType::U64 => {
            let arr:[u8; 8] = unsafe{transmute_copy(&u64::decode(bon))};
            js.check_function("pi_modules['pi/bigint/util'].exports.u64Merge".to_string());
            js.new_uint8_array(8).from_bytes(&arr);
            let r = js.invoke(1);
            if r.is_none(){
                unsafe { dukc_pop(js.get_vm()) };
                return Err("call function error: pi_modules['pi/bigint/util'].exports.u64Merge".to_string());
            }
            r
        },
        EnumType::U128 => {
            let r = u128::decode(bon);
            let arr:[u8; 16] = unsafe{transmute_copy(&r)};
            
            js.check_function("pi_modules['pi/bigint/util'].exports.u128Merge".to_string());
            js.new_uint8_array(16).from_bytes(&arr);
            let r = js.invoke(1);
            if r.is_none(){
                unsafe { dukc_pop(js.get_vm()) };
                return Err("call function error: pi_modules['pi/bigint/util'].exports.u128Merge".to_string());
            }
            r
        }
        //todo
        EnumType::U256 => js.new_u64(err_map(u64::decode(bon))?),
        EnumType::Usize => js.new_u64(err_map(u64::decode(bon))?),
        EnumType::I8 => js.new_i8(err_map(i8::decode(bon))?),
        EnumType::I16 => js.new_i16(err_map(i16::decode(bon))?),
        EnumType::I32 => js.new_i32(err_map(i32::decode(bon))?),
        //todo
        EnumType::I64 => js.new_i64(err_map(i64::decode(bon))?),
        //todo
        EnumType::I128 => js.new_i64(err_map(i64::decode(bon))?),
        //TODO
        EnumType::I256 => js.new_i64(err_map(i64::decode(bon))?),
        EnumType::Isize => js.new_i64(err_map(i64::decode(bon))?),
        EnumType::F32 => js.new_f32(err_map(f32::decode(bon))?),
        EnumType::F64 => js.new_f64(err_map(f64::decode(bon))?),
        //TODO
        EnumType::BigI => js.new_i64(err_map(i64::decode(bon))?),
        EnumType::Str => {
            let r = err_map(String::decode(bon))?;
            js.new_str(r)
        },
        //Bin应该有一个直接从片段new出array_buffer的方法， js未提供 TODO
        EnumType::Bin => {
            let bin = err_map(bon.read_bin())?;
            js.new_array_buffer(bin.len() as u32)
        },
        EnumType::Arr(v_type) => {
            let len = err_map(usize::decode(bon))?;
            let arr = js.new_array();
            for i in 0..len{
                let mut value = match decode_by_type(js, bon, v_type) {
                    Ok(v) => v,
                    Err(s) => {
                        unsafe { dukc_pop(js.get_vm()) };
                        return Err(s);
                    },
                };
                js.set_index(&arr, i as u32, &mut value);
            }
            arr
        },
        EnumType::Map(_k_type, v_type) => {
            js.get_type("Map".to_string());
            let len = err_map(usize::decode(bon))?;
            let temp = js.new_array();
            for i in 0..len{
                let mut elem = js.new_array();
                let mut key = match decode_by_type(js, bon, _k_type) {
                    Ok(v) => v,
                    Err(s) => {
                        unsafe { dukc_pop(js.get_vm()) };
                        unsafe { dukc_pop(js.get_vm()) };
                        return Err(s);
                    },
                };
                js.set_index(&elem, 0, &mut key);
                let mut value = match decode_by_type(js, bon, v_type) {
                    Ok(v) => v,
                    Err(s) => {
                        unsafe { dukc_pop(js.get_vm()) };
                        unsafe { dukc_pop(js.get_vm()) };
                        return Err(s);
                    },
                };
                js.set_index(&elem, 1, &mut value);
                js.set_index(&temp, i as u32, &mut elem);

            };
            let tmp = js.new_type("Map".to_string(), 1); //必须保证“Map”类型存在
            tmp
        },
        EnumType::Struct(v_type) => {
            decode_by_sinfo(js, bon, v_type)?
        },
        EnumType::Option(v_type) => {
            if err_map(bon.is_nil())? {
                js.new_undefined()
            }else{
                decode_by_type(js, bon, v_type)?
            }
        },
        EnumType::Enum(v_type) => {
            if err_map(bon.is_nil())? {
                js.new_undefined()
            }else{
                decode_by_enuminfo(js, bon, v_type)?
            }
        },
    };
    Ok(r)
}

//将TabKV转化为js中的Json
pub fn decode_by_tabkv(js: &Arc<JS>, tabkv: &TabKV, meta: &TabMeta) -> Result<JSType, String>{
    let obj = js.new_object();
    js.set_field(&obj, "ware".to_string(), &mut js.new_str(tabkv.ware.as_str().to_string()));
    js.set_field(&obj, "tab".to_string(), &mut js.new_str(tabkv.tab.as_str().to_string()));
    let mut key = match decode_by_type(js, &mut ReadBuffer::new(&tabkv.key, 0), &meta.k) {
        Ok(v) => v,
        Err(s) => {
            unsafe { dukc_pop(js.get_vm()) };
            return Err(s);
        },
    };
    js.set_field(&obj, "key".to_string(), &mut key);
    match &tabkv.value {
        &Some(ref v) => {
            let mut value = match decode_by_type(js, &mut ReadBuffer::new(&v, 0), &meta.v) {
                Ok(v) => v,
                Err(s) => {
                    unsafe { dukc_pop(js.get_vm()) };
                    return Err(s);
                },
            };
            js.set_field(&obj, "value".to_string(), &mut value);
        },
        None => (),
    }

    Ok(obj)
}