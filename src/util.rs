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
use pi_lib::atom::Atom;
use pi_lib::bon::{WriteBuffer, Encode, ReadBuffer, Decode};
use pi_lib::sinfo::{EnumType};

use depend::FileDes;

pub fn read_file(path: &str) -> Vec<u8>{
    let r = read(Path::new(path));
    let data = r.expect(&(String::from("file is not exist, path:") + path));
    return data;
}

pub fn read_file_str(path: &PathBuf) -> String{
	let mut file = match File::open(path) {
		Ok(f) => f,
		Err(e) => panic!(format!("no such file {:?} exception:{}", path, e))
	};
	let mut str_val = String::new();
	match file.read_to_string(&mut str_val) {
		Ok(_) => str_val,
		Err(e) => panic!("Error Reading file: {}", e)
	}
}

pub fn read_file_list(path: &PathBuf) -> Vec<FileDes>{
	let content = read_file_str(path);
	let content = unsafe{content.slice_unchecked(11, content.len() - 14)};
	parse_file_list(content)
}

pub fn read_file_map(path: &PathBuf) -> HashMap<Atom, FileDes>{
    let mut map = HashMap::new();
    let list = read_file_list(path);
    for v in list.into_iter(){
        map.insert(Atom::from(v.path.as_str()), v);
    }
	map
}

pub fn parse_file_list(s: &str) -> Vec<FileDes>{
	let r = parse(s).expect(format!("???????????????json parse err, json:{}--", s).as_str());
	match r {
		JsonValue::Array(mut v) => {
			let mut arr = Vec::new();
			for _ in 0..v.len() {
				arr.push(FileDes::from(v.pop().unwrap()));
			}
			arr
		},
		_ => {panic!("???????array??????????Vec<FileDes>");},
	}
}

fn encode_depend(tr: &Tr, arr:&Vec<FileDes>){
    let mut items = Vec::new();
    let ware = Atom::from("memory");
    let tab = Atom::from("_$depend");
    for v in arr.iter(){
        let mut value = WriteBuffer::new();
        v.encode(&mut value);
        let mut key = WriteBuffer::new();
        v.path.encode(&mut key);
        items.push(TabKV{
            index: 0,
            tab: tab.clone(),
            ware: ware.clone(),
            value: Some(Arc::new(value.unwrap())),
            key: Arc::new(key.unwrap()),
        })
    }
    tr.modify(items, None, false, Arc::new(move |r: SResult<()>|{
        r.expect("encode_depend fail");
    }));
}

//创建depend表， 并初始化depend表的数据
pub fn store_depend(mgr: &Mgr, arr: &Vec<FileDes>){
    let tr = mgr.transaction(true);
    let tr1 = tr.clone();
    match tr.alter(&Atom::from("memory"), &Atom::from("_$depend"), Some(Arc::new(TabMeta::new(EnumType::Str, EnumType::Bin))), Arc::new(move |_r: SResult<()>|{})) {
        Some(Ok(_)) => {
            encode_depend(&tr1, arr);
            tr.prepare(Arc::new(move |_r: SResult<()>|{}));
            tr.commit(Arc::new(move |_r: SResult<()>|{}));
        },
        _ => panic!("create_depend fail"),
    };
}

//从mgr中读depend
pub fn read_depend(mgr: &Mgr) -> HashMap<Atom, FileDes>{
    let mut map = HashMap::new();
    let tr = mgr.transaction(false);
    let mut it = tr.iter(&Atom::from("memory"), &Atom::from("_$depend"), None, false, None, Arc::new(|_|{})).unwrap().expect("");
    loop {
        match it.next(Arc::new(|_|{})) {
            Some(temp) => {
                let temp = temp.expect("");
                match temp {
                    Some(v) => {
                        let mut bb = ReadBuffer::new(v.1.as_slice(), 0);
                        let file_des = FileDes::decode(&mut bb).unwrap();
                        map.insert(Atom::from(file_des.path.as_str()), file_des);
                    },
                    None => break,
                };
            },
            None => panic!("read_depend fail"),
        }
    };
    map
}