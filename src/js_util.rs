use std::sync::Arc;

use pi_vm::adapter::{JSType, JS};
use pi_lib::sinfo::{StructInfo, EnumType};
use pi_lib::bon::{ReadBuffer, Decode};
use pi_db::db::{TabKV, TabMeta};

pub fn decode_by_sinfo(js: &Arc<JS>, bon: &mut ReadBuffer, sinfo: &StructInfo) -> JSType {
    let name = sinfo.name.as_str();

    match name {
        "" => { //该类型为元组
            let arr = js.new_array();
            for v in sinfo.fields.iter(){
                js.set_index(&arr, v.name.parse().expect("String cannot be converted to digits"), &decode_by_type(js, bon, &v.ftype));
            }
            return arr;
        },
        _ => (),
    };

    let index = match name.find("."){
        Some(v) => v,
        None => panic!("illegal module name, lack '.', modName: {}", name),
    };
    let r = name.split_at(index);// r.0为模块名， r.1为类型名称;
    let type_name = String::from("pi_modules['") + r.0 + ".s']" + ".exports" + r.1;
    let obj = js.new_type_object(type_name);
    if obj.is_undefined(){
        panic!("module is not exist, please make sure the module has been loaded, modName: {}", name)
    }

    for v in sinfo.fields.iter(){
        js.set_field(&obj, String::from(v.name.as_str()), &decode_by_type(js, bon, &v.ftype));
    }
    obj
}

pub fn decode_by_type(js: &Arc<JS>, bon: &mut ReadBuffer, t: &EnumType) -> JSType {
    match t {
        EnumType::Bool => js.new_boolean(bool::decode(bon)),
        EnumType::U8 => js.new_u8(u8::decode(bon)),
        EnumType::U16 => js.new_u16(u16::decode(bon)),
        EnumType::U32 => js.new_u32(u32::decode(bon)),
        //todo
        EnumType::U64 => js.new_u64(u64::decode(bon)),
        //todo
        EnumType::U128 => js.new_u64(u64::decode(bon)),
        //todo
        EnumType::U256 => js.new_u64(u64::decode(bon)),
        EnumType::Usize => js.new_u32(u32::decode(bon)),
        EnumType::I8 => js.new_i8(i8::decode(bon)),
        EnumType::I16 => js.new_i16(i16::decode(bon)),
        EnumType::I32 => js.new_i32(i32::decode(bon)),
        EnumType::I64 => js.new_i64(i64::decode(bon)),
        //todo
        EnumType::I128 => js.new_i64(i64::decode(bon)),
        //TODO
        EnumType::I256 => js.new_i64(i64::decode(bon)),
        EnumType::Isize => js.new_i32(i32::decode(bon)),
        EnumType::F32 => js.new_f32(f32::decode(bon)),
        EnumType::F64 => js.new_f64(f64::decode(bon)),
        //TODO
        EnumType::BigI => js.new_i64(i64::decode(bon)),
        EnumType::Str => js.new_str(String::decode(bon)),
        //Bin应该有一个直接从片段new出array_buffer的方法， js未提供 TODO
        EnumType::Bin => {
            let bin = bon.read_bin();
            js.new_array_buffer(bin.len() as u32)
        },
        //时间戳， 没用， 可能会删除， 因此这里也没有实现
        EnumType::UTC => js.new_i64(i64::decode(bon)),
        EnumType::Arr(v_type) => {
            let arr = js.new_array();
            let len = usize::decode(bon);
            for i in 0..len{
                js.set_index(&arr, i as u32, &decode_by_type(js, bon, v_type));
            }
            arr
        }
        //map暂时使用json代替， 需要更改 TODO
        EnumType::Map(_k_type, _v_type) => {
            js.new_object()
        }
        EnumType::Struct(v_type) => decode_by_sinfo(js, bon, v_type),
    }
}

//将TabKV转化为js中的Json
pub fn decode_by_tabkv(js: &Arc<JS>, tabkv: &TabKV, meta: &TabMeta) -> JSType {
    let obj = js.new_object();
    js.set_field(&obj, "ware".to_string(), &js.new_str(tabkv.ware.as_str().to_string()));
    js.set_field(&obj, "tab".to_string(), &js.new_str(tabkv.tab.as_str().to_string()));
    js.set_field(&obj, "key".to_string(), &decode_by_type(js, &mut ReadBuffer::new(&tabkv.key, 0), &meta.k));
    match &tabkv.value {
        &Some(ref v) => {println!("decode_by_tabkv value-----------------------------------------"); js.set_field(&obj, "value".to_string(), &decode_by_type(js, &mut ReadBuffer::new(&v, 0), &meta.v));},
        None => (),
    }
    obj
}