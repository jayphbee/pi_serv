use std::path::Path;
use std::fs::read;

pub fn read_file(path: &str) -> Vec<u8>{
    let r = read(Path::new(path));
    let data = r.expect(&(String::from("file is not exist, path:") + path));
    return data;
}