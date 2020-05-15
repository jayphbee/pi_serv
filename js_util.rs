use std::sync::Arc;
use std::mem::transmute_copy;

use pi_vm::adapter::{JSType, JS, dukc_pop};
use sinfo::{StructInfo, EnumType, EnumInfo};
use lib_util::err_string;
use bon::{ReadBuffer, Decode, ReadBonErr};
use pi_db::db::{TabKV, TabMeta};
use js_env::{env_var};


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
		"_$Object" => {
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
        },
		"_$Json" => {
			return decode_json(js, bon);
		},//一个普通的Json 形如{...}, [...]
		_ => (),
    };

    let index = match name.find("."){
        Some(v) => v,
        None => panic!("illegal module name, lack '.', modName: {}", name),
    };
	let proj_root = env_var("PROJECT_ROOT").unwrap();
	let r = name.split_at(index);// r.0为模块名， r.1为类型名称;
    let type_name = format!("Module.modules['{}/{}.struct.js'].exports{}", proj_root, r.0, r.1);
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
    let proj_root = env_var("PROJECT_ROOT").unwrap();
    let r = name.split_at(index);// r.0为模块名， r.1为类型名称;
    let type_name = format!("Module.modules['{}/{}.struct.js'].exports{}", proj_root, r.0, r.1);
    js.get_type(type_name.clone());
    let obj = js.new_type(type_name.clone(), 0);
    if obj.is_undefined(){
		unsafe { dukc_pop(js.get_vm()) };
		return Err(String::from("module is not exist, please make sure the module has been loaded, modName:")+ &type_name);
	}

    let index = err_string(usize::decode(bon))?;
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
    let proj_root = env_var("PROJECT_ROOT").unwrap();
    let r = match t {
        EnumType::Bool => js.new_boolean(err_string(bool::decode(bon))?),
        EnumType::U8 => js.new_u8(err_string(u8::decode(bon))?),
        EnumType::U16 => js.new_u16(err_string(u16::decode(bon))?),
        EnumType::U32 => js.new_u32(err_string(u32::decode(bon))?),
        EnumType::U64 => {
            let arr:[u8; 8] = unsafe{transmute_copy(&u64::decode(bon))};
            js.check_function(format!("Module.modules['{}/pi_utils//math/bigint/util.js'].exports.u64Merge", proj_root));
            js.new_uint8_array(8).from_bytes(&arr);
            let r = js.invoke(1);
            if r.is_none(){
                unsafe { dukc_pop(js.get_vm()) };
                return Err("call function error: Module.modules['pi_utils//math/bigint/util.js'].exports.u64Merge".to_string());
            }
            r
        },
        EnumType::U128 => {
            let r = u128::decode(bon);
            let arr:[u8; 16] = unsafe{transmute_copy(&r)};
            
            js.check_function(format!("Module.modules['{}/pi_utils//math/bigint/util.js'].exports.u128Merge", proj_root));
            js.new_uint8_array(16).from_bytes(&arr);
            let r = js.invoke(1);
            if r.is_none(){
                unsafe { dukc_pop(js.get_vm()) };
                return Err("call function error: Module.modules['pi_utils//math/bigint/util.js'].exports.u128Merge".to_string());
            }
            r
        }
        //todo
        EnumType::U256 => js.new_u64(err_string(u64::decode(bon))?),
        EnumType::Usize => js.new_u64(err_string(u64::decode(bon))?),
        EnumType::I8 => js.new_i8(err_string(i8::decode(bon))?),
        EnumType::I16 => js.new_i16(err_string(i16::decode(bon))?),
        EnumType::I32 => js.new_i32(err_string(i32::decode(bon))?),
        //todo
        EnumType::I64 => js.new_i64(err_string(i64::decode(bon))?),
        //todo
        EnumType::I128 => js.new_i64(err_string(i64::decode(bon))?),
        //TODO
        EnumType::I256 => js.new_i64(err_string(i64::decode(bon))?),
        EnumType::Isize => js.new_i64(err_string(i64::decode(bon))?),
        EnumType::F32 => js.new_f32(err_string(f32::decode(bon))?),
        EnumType::F64 => js.new_f64(err_string(f64::decode(bon))?),
        //TODO
        EnumType::BigI => js.new_i64(err_string(i64::decode(bon))?),
        EnumType::Str => {
            let r = err_string(String::decode(bon))?;
            match js.new_str(r) {
                Err(e) => {
                    return Err(e);
                },
                Ok(v) => v,
            }
        },
        //Bin应该有一个直接从片段new出array_buffer的方法， js未提供 TODO
        EnumType::Bin => {
            let bin = err_string(bon.read_bin())?;
            js.new_array_buffer(bin.len() as u32)
        },
        EnumType::Arr(v_type) => {
            let len = err_string(usize::decode(bon))?;
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
            let len = err_string(usize::decode(bon))?;
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
            if err_string(bon.is_nil())? {
                js.new_undefined()
            }else{
                decode_by_type(js, bon, v_type)?
            }
        },
        EnumType::Enum(v_type) => {
            if err_string(bon.is_nil())? {
                js.new_undefined()
            }else{
                decode_by_enuminfo(js, bon, v_type)?
            }
        },
    };
    Ok(r)
}

pub fn decode_any(js: &Arc<JS>, bon: &mut ReadBuffer) -> Result<JSType, String>{
	let r = err_string(bon.get_type())?;
	let v = match r {
		0 => {
			err_string(bon.is_nil())?;
			js.new_undefined()
		},
		1..3 => js.new_boolean(err_string(bool::decode(bon))?),
		3..42 => js.new_f64(err_string(f64::decode(bon))?),
		42..111 => js.new_str(err_string(String::decode(bon))?)?,
		111..180 => {
			let bin = err_string(bon.read_bin())?;
			js.new_array_buffer(bin.len() as u32)
		},
		180..249 => decode_json(js, bon)?,
		_ => return Err(format!("decode_json fail, ty:{}", r)),
	};
	Ok(v)

	// 0=null
// 1=false
// 2=true
// 3=浮点数0.0，4=浮点数1.0，5=16位浮点数，6=32位浮点数，7=64位浮点数，8=128位浮点数;
// 9=8位负整数，10=16位负整数，11=32位负整数，12=48位负整数，13=64位负整数，14=128位负整数
// 15~35= -1~19
// 36=8位正整数，37=16位正整数，38=32位正整数，39=48位正整数，40=64位正整数，41=128位正整数

// 42-106=0-64长度的UTF8字符串，
// 107=8位长度的UTF8字符串，108=16位长度的UTF8字符串，109=32位长度的UTF8字符串，110=48位长度的UTF8字符串

// 111-175=0-64长度的二进制数据，
// 176=8位长度的二进制数据，177=16位长度的二进制数据，178=32位长度的二进制数据，179=48位长度的二进制数据

// 180-244=0-64长度的容器，包括对象、数组和map、枚举
// 245=8位长度的容器，246=16位长度的容器，247=32位长度的容器，248=48位长度的容器
// 之后的一个4字节的整数表示类型。
// 类型：
// 	0 表示忽略
// 	1 通用对象
// 	2 通用数组
// 	3 通用map
	// if ()
}

pub fn decode_json(js: &Arc<JS>, bon: &mut ReadBuffer) -> Result<JSType, String>{
	let callbackfn = |bb: &mut ReadBuffer, t: u32, mut l: u64| -> Result<JSType, ReadBonErr> {
		let old_head = bb.head;
		l = l - 4; // l包含t的长度（4字节）
		let r;
		if t == 2 {
			r = js.new_array();
			let mut i = 0;
			if ((bb.head - old_head) as u64) < l {
				// 读长度
				match decode_any(js, bb) {
					Ok(v) => v,
					Err(_s) => {
						unsafe { dukc_pop(js.get_vm()) };
						return Err(ReadBonErr::Other("json type error".to_string()));
					},
				};
			}
			loop {
				if ((bb.head - old_head) as u64) < l {
					let mut v = match decode_any(js, bb) {
						Ok(v) => v,
						Err(_s) => {
							unsafe { dukc_pop(js.get_vm()) };
							return Err(ReadBonErr::Other("json type error".to_string()));
						},
					};
					js.set_index(&r, i as u32, &mut v);
					i += 1;
				}else {
					break;
				}
			}
		} else if t == 1 {
			r = js.new_object();
			loop {
				if ((bb.head - old_head) as u64) < l {
					let k = match String::decode(bb) {
						Ok(r) => r,
						Err(_s) => {
							unsafe { dukc_pop(js.get_vm()) };
							return Err(ReadBonErr::Other("json type error".to_string()));
						}
					};
					let mut v = match decode_any(js, bb) {
						Ok(v) => v,
						Err(_s) => {
							unsafe { dukc_pop(js.get_vm()) };
							return Err(ReadBonErr::Other("json type error".to_string()));
						},
					};
					js.set_field(&r, k, &mut v);
				}else {
					break;
				}
			}
		} else {
			return Err(ReadBonErr::Other("json type error".to_string()));
		}
		return Ok(r);
	};
	return err_string(bon.read_container(callbackfn));
}

//将TabKV转化为js中的Json
pub fn decode_by_tabkv(js: &Arc<JS>, tabkv: &TabKV, meta: &TabMeta) -> Result<JSType, String>{
    let obj = js.new_object();
    match js.new_str(tabkv.ware.as_str().to_string()) {
        Err(e) => {
            return Err(e);
        },
        Ok(mut v) => {
            js.set_field(&obj, "ware".to_string(), &mut v);
        },
    }
    match js.new_str(tabkv.tab.as_str().to_string()) {
        Err(e) => {
            return Err(e);
        },
        Ok(mut v) => {
            js.set_field(&obj, "tab".to_string(), &mut v);
        },
    }
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