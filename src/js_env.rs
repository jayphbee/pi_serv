use std::env;

pub struct Args(env::Args);

// 命令行参数迭代器
impl Args {
	pub fn next(&mut self) -> Option<String> {
		self.0.next()
	}
}

// 环境变量迭代器
pub struct EnvVars(env::Vars);

impl EnvVars {
	pub fn next(&mut self) -> Option<(String, String)> {
		self.0.next()
	}
}

pub fn args() -> Args {
	Args(env::args())
}

/**
 * 返回当前工作目录
 * 错误
 * 	如果当前工作目录值无效，则返回Err， 可能的情况：
 * 		1.当前目录不存在
 * 		2.没有足够的权限来访问当前目录
 */
pub fn current_dir() -> Result<String, String> {
	match env::current_dir() {
		Ok(r) => match r.to_str() {
			Some(r) => Ok(r.to_string()),
			None => Err("current_dir to_string fail".to_string()),
		},
		Err(e) => Err(e.to_string()),
	}
}

/**
 * 将当前工作目录更改为指定路径
 * 如果操作失败，则返回Err
 */
pub fn set_current_dir(path: &str) -> Result<(), String>{
	match env::set_current_dir(path) {
		Ok(_) => Ok(()),
		Err(e) => Err(e.to_string()),
	}
}

/**
 * 将当前正在运行的进程的环境变量k设置为v
 * 如果key为空，包含ASCII等号'='或NUL字符'\0'，或者该值包含NUL字符，则此函数可能会恐慌。
 */
pub fn set_env_var(k: &str, v: &str){
	env::set_var(k, v)
}

/**
 * 返回当前正在运行的可执行文件的完整文件系统路径
 * 平台特定的行为: 如果可执行文件是通过符号链接调用的，则某些平台将返回符号链接的路径，而其他平台将返回符号链接目标的路径
 * 错误： 获取当前可执行文件的路径是特定于平台的操作，该操作可能由于多种原因而失败。一些错误可以包括但不限于文件系统操作失败或常规syscall错误。
 */
pub fn current_exe() -> Result<String, String> {
	match env::current_exe() {
		Ok(r) => match r.to_str() {
			Some(r) => Ok(r.to_string()),
			None => Err("current_dir to_string fail".to_string()),
		},
		Err(e) => Err(e.to_string()),
	}
}

/**
 * 当前进程中获取指定环境变量
 * @param key , 环境变量的key
 * @return Error（环境变量不存在 环境变量无效的unicode） or Ok(String)
 * 如果key为空，包含ASCII等号'='或NUL字符'\0'，或者该值包含NUL字符，则此函数可能会恐慌。
 */
pub fn env_var(key: &str) ->  Result<String, env::VarError> {
	env::var(key)
}


/**
 * 返回当前进程的所有环境变量的字符串（变量，值）对的迭代器
 * 返回的迭代器包含此调用时进程环境变量的快照。之后对环境变量的修改将不会反映在返回的迭代器中。
 * 错误： 进行迭代时，如果环境中的任何键或值都不是有效的unicode，则返回的迭代器将发生混乱
 */
pub fn env_vars() -> EnvVars {
	EnvVars(env::vars())
}
