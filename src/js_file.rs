use std::fs::{read_dir, File, OpenOptions, write, rename as std_rename, remove_file as std_remove_file, remove_dir as std_remove_dir};
use std::io::{Write, Read};
use std::path::Path;

use file::file::{AsyncFile, AsyncFileOptions, WriteOptions};
use crate::binary::Binary;





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
 * 同步写文件
*/
pub fn write_file_string_sync(path: String, text: String, file_write_option: FileWriteOptions) -> Result<(), String> {
	match file_write_option {
		FileWriteOptions::OnlyWrite | FileWriteOptions::ReadWrite | FileWriteOptions::OnlyAppend | FileWriteOptions::ReadAppend => {
			let mut open_option = OpenOptions::new();
			open_option.read(true).write(true).append(true).create(true);
			match open_option.open(path) {
				Ok(mut file) => {
					file.write(text.as_bytes()).map(|_| ()).map_err(|e| e.to_string())
				}
				Err(e) => Err(e.to_string())
			}
		}
		FileWriteOptions::TruncateWrite => {
			write(path, text.as_bytes()).map_err(|e| e.to_string())
		}
	}
}

pub fn write_file_buffer_sync(path: String, bytes: &[u8], file_write_option: FileWriteOptions) -> Result<(), String> {
	match file_write_option {
		FileWriteOptions::OnlyWrite | FileWriteOptions::ReadWrite | FileWriteOptions::OnlyAppend | FileWriteOptions::ReadAppend => {
			let mut open_option = OpenOptions::new();
			open_option.read(true).write(true).append(true).create(true).create_new(true);
			match open_option.open(path) {
				Ok(mut file) => {
					file.write(bytes).map(|_| ()).map_err(|e| e.to_string())
				}
				Err(e) => Err(e.to_string())
			}
		}
		FileWriteOptions::TruncateWrite => {
			write(path, &bytes).map_err(|e| e.to_string())
		}
	}
}

/**
 * 异步写文件
*/
pub fn write_file_string(path: String, text: String, file_write_option: FileWriteOptions, call_back: Box<dyn FnOnce(Result<String, String>)>) {
	let option = match file_write_option {
		FileWriteOptions::OnlyWrite => AsyncFileOptions::OnlyWrite(1),
		FileWriteOptions::ReadWrite => AsyncFileOptions::ReadWrite(1),
		FileWriteOptions::TruncateWrite => AsyncFileOptions::TruncateWrite(1),
		FileWriteOptions::ReadAppend => AsyncFileOptions::ReadAppend(1),
		FileWriteOptions::OnlyAppend => AsyncFileOptions::OnlyAppend(1)
	};
	AsyncFile::open(path, option, Box::new(|r: std::io::Result<AsyncFile>| {
		match r {
			Ok(r) => {
				r.write(WriteOptions::None,0, text.into_bytes(), Box::new(|_s: AsyncFile, r: std::io::Result<()>| {
					match r {
						Ok(()) => call_back(Ok("".to_string())),
						Err(e) => call_back(Err(e.to_string())),
					}
				}))
			},
			Err(e) => call_back(Err(e.to_string())),
		}
	}));
}

pub fn write_file_buffer(path: String, bytes: &[u8], file_write_option: FileWriteOptions, call_back: Box<dyn FnOnce(Result<String, String>)>) {
	let option = match file_write_option {
		FileWriteOptions::OnlyWrite => AsyncFileOptions::OnlyWrite(1),
		FileWriteOptions::ReadWrite => AsyncFileOptions::ReadWrite(1),
		FileWriteOptions::TruncateWrite => AsyncFileOptions::TruncateWrite(1),
		FileWriteOptions::ReadAppend => AsyncFileOptions::ReadAppend(1),
		FileWriteOptions::OnlyAppend => AsyncFileOptions::OnlyAppend(1)
	};
	let bytes = bytes.to_vec();
	AsyncFile::open(path, option, Box::new(|r: std::io::Result<AsyncFile>| {
		match r {
			Ok(r) => {
				r.write(WriteOptions::None,0, bytes, Box::new(|_s: AsyncFile, r: std::io::Result<()>| {
					match r {
						Ok(()) => call_back(Ok("".to_string())),
						Err(e) => call_back(Err(e.to_string())),
					}
				}))
			},
			Err(e) => call_back(Err(e.to_string())),
		}
	}));
}

/**
 * 同步改名
*/
pub fn rename_sync(from: String, to: String) -> Result<String, String> {
	std_rename(from, to).map(|r|"".to_string()).map_err(|e| e.to_string())
}

/**
 * 异步改名
*/
pub fn rename(from: String, to: String, call_back: Box<dyn FnOnce(Result<String, String>)>) {
	AsyncFile::rename(from, to, Box::new(|_from, _to, res| {
		match res {
			Ok(()) => call_back(Ok("".to_string())),
			Err(e) => call_back(Err(e.to_string()))
		}
	}));
}

/** 
 * 同步删除文件
*/
pub fn remove_file_sync(path: String) -> Result<String, String> {
	std_remove_file(path).map(|r|"".to_string()).map_err(|e| e.to_string())
}

/** 
 * 异步删除文件
*/
pub fn remove_file(path: String, call_back: Box<dyn FnOnce(Result<String, String>)>) {
	AsyncFile::remove(path, Box::new(|res| {
		match res {
			Ok(()) => call_back(Ok("".to_string())),
			Err(e) => call_back(Err(e.to_string()))
		}
	}))
}

/**
* 同步删除空文件夹
*/
pub fn remove_dir_sync(path: String) -> Result<String, String> {
	std_remove_dir(path).map(|_r| "".to_string()).map_err(|e| e.to_string())
}

/**
* 异步删除空文件夹
*/
pub fn remove_dir(path: String, call_back: Box<dyn FnOnce(Result<String, String>)>) {
	AsyncFile::remove_dir(path, Box::new(|res| {
		match res {
			Ok(()) => call_back(Ok("".to_string())),
			Err(e) => call_back(Err(e.to_string()))
		}
	}))
}

/**
 * 同步读目录里面的所有文件
 */
pub fn walk_dir_sync(path: &str) -> Result<Vec<String>, String> {
	let path = Path::new(path);
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

// pub fn walk_dir(path: &str, call_back: Arc<dyn FnOnce(Result<String, String>)>) {
// 	// TODO
// }

// ======================  use Binary as param =====================
pub fn write_file_buffer_binary(path: String, bin: Binary, file_write_option: FileWriteOptions, call_back: Box<dyn FnOnce(Result<String, String>)>) {
	let option = match file_write_option {
		FileWriteOptions::OnlyWrite => AsyncFileOptions::OnlyWrite(1),
		FileWriteOptions::ReadWrite => AsyncFileOptions::ReadWrite(1),
		FileWriteOptions::TruncateWrite => AsyncFileOptions::TruncateWrite(1),
		FileWriteOptions::ReadAppend => AsyncFileOptions::ReadAppend(1),
		FileWriteOptions::OnlyAppend => AsyncFileOptions::OnlyAppend(1)
	};
	if let Some(bytes) = bin.take() {
		AsyncFile::open(path, option, Box::new(|r: std::io::Result<AsyncFile>| {
			match r {
				Ok(r) => {
					r.write(WriteOptions::None,0, bytes, Box::new(|_s: AsyncFile, r: std::io::Result<()>| {
						match r {
							Ok(()) => call_back(Ok("".to_string())),
							Err(e) => call_back(Err(e.to_string())),
						}
					}))
				},
				Err(e) => call_back(Err(e.to_string())),
			}
		}));
	} else {
		call_back(Err("Binary has been taken!!!".to_string()))
	}
	
}

pub fn read_file_buffer_binary(path: String, call_back: Box<dyn FnOnce(Result<Binary, String>)>) {
	AsyncFile::open(path, AsyncFileOptions::OnlyRead(1), Box::new(|r: std::io::Result<AsyncFile>| {
		match r {
			Ok(r) => {
				let len = r.get_size();
				r.read(0, len as usize, Box::new(|_s: AsyncFile, r: std::io::Result<Vec<u8>>| {
					match r {
						Ok(r) => call_back(Ok(Binary::new(r))),
						Err(e) => call_back(Err(e.to_string())),
					}
				}))
			},
			Err(e) => call_back(Err(e.to_string())),
		}
	}));
}

pub fn is_absolute(path: &str) -> bool {
	Path::new(path).is_absolute()
}

pub fn is_relative(path: &str) -> bool {
	Path::new(path).is_relative()
}

pub fn full_path(path: &str) -> Option<String> {
	let p = Path::new(path);
	match p.canonicalize() {
		Ok(cano) => {
			let cano = cano.to_str().unwrap().split(":").map(|s|s.to_string()).collect::<Vec<String>>();
			// unix 环境
			if cano.len() == 1 {
				Some(cano[0].clone())
			// windows 环境
			} else {
				Some(cano[1].replace("\\","/"))
			}
		}
		Err(_e) => None
	}
}

pub enum FileWriteOptions {
    OnlyWrite,
    OnlyAppend,
    ReadAppend,
    ReadWrite,
    TruncateWrite,
}