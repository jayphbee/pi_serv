use std::path::Path;
use std::fs::{File};
use std::path::PathBuf;
use std::io::Read;
use json::{JsonValue, parse};
use std::fs::read;
use std::sync::Arc;
use std::collections::HashMap;

use pi_db::mgr::{Mgr, Tr};
use pi_db::db::{SResult, TabKV, TabMeta};
use atom::Atom;
use bon::{WriteBuffer, Encode, ReadBuffer, Decode};
use sinfo::{EnumType};

/**
* 同步的读取指定文件的数据
* @param path 文件的路径
* @returns 返回文件的数据
*/
pub fn read_file(path: &str) -> Vec<u8>{
    let r = read(Path::new(path));
    let data = r.expect(&(String::from("file is not exist, path:") + path));
    return data;
}