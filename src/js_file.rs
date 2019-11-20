use std::fs::{read_dir, File};
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use file::file::{AsyncFile, AsyncFileOptions};





// pub enum AsyncFileOptions {
//     OnlyRead(u8),
//     OnlyWrite(u8),
//     OnlyAppend(u8), // m
//     ReadAppend(u8),
//     ReadWrite(u8),
//     TruncateWrite(u8),
// }


/**
 * 异步读文件， 返回buffer
 * 32位系统， 读文件长度不能超过u32的最大值
 */
pub fn read_file_buffer(path: String, call_back: Box<dyn FnOnce(Result<Vec<u8>, String>)>) {
	AsyncFile::open(path, AsyncFileOptions::OnlyRead(1), Box::new(|r: std::io::Result<AsyncFile>| {
		match r {
			Ok(r) => {
				let len = r.get_size();
				r.read(0, len as usize, Box::new(|_s: AsyncFile, r: std::io::Result<Vec<u8>>| {
					match r {
						Ok(r) => call_back(Ok(r)),
						Err(e) => call_back(Err(e.to_string())),
					}
				}))
			},
			Err(e) => call_back(Err(e.to_string())),
		}
	}));
}

/**
 * 异步读文件， 返回字符串（目前仅支持utf8编码）
 * 32位系统， 读文件长度不能超过u32的最大值
 */
pub fn read_file_string(path: String, call_back: Box<dyn FnOnce(Result<String, String>)>) {
	AsyncFile::open(path, AsyncFileOptions::OnlyRead(1), Box::new(|r: std::io::Result<AsyncFile>| {
		match r {
			Ok(r) => {
				let len = r.get_size();
				r.read(0, len as usize, Box::new(|_s: AsyncFile, r: std::io::Result<Vec<u8>>| {
					match r {
						Ok(r) => match String::from_utf8(r) {
							Ok(r) => call_back(Ok(r)),
							Err(e) => call_back(Err(e.to_string())),
						},
						Err(e) => call_back(Err(e.to_string())),
					}
				}));
			},
			Err(e) => call_back(Err(e.to_string())),
		}
	}));
}

/**
 * 同步读文件, 返回buffer
 */
pub fn read_file_buffer_sync(path: &str) -> Result<Vec<u8>, String> {
	let mut file = match File::open(path) {
		Ok(f) => f,
		Err(e) => return Err(e.to_string()),
	};
	let mut data = Vec::new();
	if let Err(e) = file.read(&mut data) {
		return Err(e.to_string());
	}
	Ok(data)
}

/**
 * 同步读文件, 返回字符串（目前仅支持utf8编码）
 */
pub fn read_file_string_sync(path: &str) -> Result<String, String> {
	let mut file = match File::open(path) {
		Ok(f) => f,
		Err(e) => return Err(e.to_string()),
	};
	let mut data = String::new();
	if let Err(e) = file.read_to_string(&mut data) {
		return Err(e.to_string());
	}
	Ok(data)
}

/**
 * 同步读目录里面的所有文件
 */
pub fn walk_dir_sync(path: &Path) -> Result<Vec<String>, String> {
	if !path.is_dir() {
		return Ok(vec![]);
	}

	let mut stack = vec![];
	stack.push(path.to_str().unwrap().to_string());
	let mut res = vec![];

	loop {
		if let Some(dir) = stack.pop() {
			match read_dir(dir) {
				Ok(entries) => {
					for entry in entries {
						if let Ok(e) = entry {
							if e.path().is_file() {
								res.push(e.path().to_str().unwrap().replace("\\", "/").to_string());
							} else if e.path().is_dir() {
								let p = e.path().to_str().unwrap().replace("\\", "/").to_string();
								stack.push(p);
							}
						}
					}
				}
				Err(_e) => {}
			}
		} else {
			break;
		}
	}

	Ok(res)
}

pub fn walk_dir(path: &Path, call_back: Arc<Box<dyn FnOnce(Result<String, String>)>>) {
	// TODO
}