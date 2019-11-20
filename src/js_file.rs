use std::fs::File;
use std::io::Read;

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