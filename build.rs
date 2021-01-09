use std::env;
use std::fs::{copy, create_dir_all};
use std::path::PathBuf;
use std::sync::mpsc::channel;

use js_proxy_gen::{generate_proxy_crate, parse_crates, spawn};

/*
* 默认的代理库路径
*/
#[cfg(target_os = "windows")]
const DEFAULT_PI_JS_PROXY_CRATE_PATH: &str = r#"..\pi_serv_ext"#;
#[cfg(target_os = "linux")]
const DEFAULT_PI_JS_PROXY_CRATE_PATH: &str = "../pi_serv_ext";

/*
* 默认的代理库版本
*/
const DEFAULT_PI_JS_PROXY_CRATE_VERSION: &str = "0.1.0";

/*
* 默认的代理库Rust版本
*/
const DEFAULT_PI_JS_PROXY_CRATE_EDITION: &str = "2018";

/*
* 默认的分析模式为顺序
*/
const DEFAULT_PI_JS_PROXY_PARSE_MODE: bool = false;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let opt_level = env::var_os("OPT_LEVEL").unwrap();

    if opt_level == "0" {
        //debug版本
        let origin_dir = if cfg!(windows) {
            current_dir
                .join("lib")
                .join("rusty_v8")
                .join("win")
                .join("rusty_v8.lib")
        } else {
            current_dir
                .join("lib")
                .join("rusty_v8")
                .join("linux")
                .join("debug")
                .join("librusty_v8.a")
        };

        let lib_name = origin_dir.file_name().unwrap();
        let target_dir = current_dir
            .join("target")
            .join("debug")
            .join("gn_out")
            .join("obj");

        if let Err(e) = create_dir_all(&target_dir) {
            eprintln!(
                "==> Build debug failed, target path: {:?}, reason: {:?}",
                target_dir, e
            );
        } else {
            //创建目录成功
            let lib_path = target_dir.join(lib_name);

            // if !lib_path.exists() {
            //指定路径的库文件不存在，则复制
            if let Err(e) = copy(&origin_dir, target_dir.join(lib_name)) {
                eprintln!(
                    "==> Build debug failed, origin path: {:?}, target path: {:?}, reason: {:?}",
                    &origin_dir, target_dir, e
                );
            }
            // }
        }
    } else {
        //release版本
        let origin_dir = if cfg!(windows) {
            current_dir
                .join("lib")
                .join("rusty_v8")
                .join("win")
                .join("rusty_v8.lib")
        } else {
            current_dir
                .join("lib")
                .join("rusty_v8")
                .join("linux")
                .join("release")
                .join("librusty_v8.a")
        };

        let lib_name = origin_dir.file_name().unwrap();
        let target_dir = current_dir
            .join("target")
            .join("release")
            .join("gn_out")
            .join("obj");

        if let Err(e) = create_dir_all(&target_dir) {
            eprintln!(
                "==> Build debug failed, target path: {:?}, reason: {:?}",
                target_dir, e
            );
        } else {
            //创建目录成功
            let lib_path = target_dir.join(lib_name);

            // if !lib_path.exists() {
            //指定路径的库文件不存在，则复制
            if let Err(e) = copy(&origin_dir, target_dir.join(lib_name)) {
                eprintln!(
                    "==> Build debug failed, origin path: {:?}, target path: {:?}, reason: {:?}",
                    &origin_dir, target_dir, e
                );
            }
            // }
        }
    }

    //构建代理代码
    let ext_crates = if let Some(ext_crates_var) = env::var_os("PI_JS_PROXY_EXT_CRATES") {
        ext_crates_var
            .to_str()
            .unwrap()
            .split(';')
            .map(|path| PathBuf::from(path))
            .collect::<Vec<PathBuf>>()
    } else {
        panic!("Require set for PI_JS_PROXY_EXT_CRATES");
    };
    let crate_path = if let Some(crate_path_var) = env::var_os("PI_JS_PROXY_CRATE_PATH") {
        let crate_path = crate_path_var.to_str().unwrap();
        PathBuf::from(crate_path)
    } else {
        PathBuf::from(DEFAULT_PI_JS_PROXY_CRATE_PATH)
    };
    let crate_version = if let Some(crate_version_var) = env::var_os("PI_JS_PROXY_CRATE_VERSION") {
        crate_version_var.to_str().unwrap().to_string()
    } else {
        DEFAULT_PI_JS_PROXY_CRATE_VERSION.to_string()
    };
    let crate_edition = if let Some(crate_edition_var) = env::var_os("PI_JS_PROXY_CRATE_EDITION") {
        crate_edition_var.to_str().unwrap().to_string()
    } else {
        DEFAULT_PI_JS_PROXY_CRATE_EDITION.to_string()
    };
    let ts_path = if let Some(ts_path_var) = env::var_os("PI_JS_PROXY_TS_PATH") {
        let ts_path = ts_path_var.to_str().unwrap();
        PathBuf::from(ts_path)
    } else {
        panic!("Require set for PI_JS_PROXY_TS_PATH");
    };
    let is_concurrent =
        if let Some(is_concurrent_var) = env::var_os("PI_JS_PROXY_PARSE_IS_CONCURRENT") {
            is_concurrent_var
                .to_str()
                .unwrap()
                .parse::<bool>()
                .unwrap_or(DEFAULT_PI_JS_PROXY_PARSE_MODE)
        } else {
            DEFAULT_PI_JS_PROXY_PARSE_MODE
        };

    let (sender, receiver) = channel();
    spawn(async move {
        match parse_crates(ext_crates, is_concurrent).await {
            Err(e) => panic!("Parse ext crates failed, reason: {:#?}", e),
            Ok(crates) => {
                if let Err(e) = generate_proxy_crate(
                    crate_path,
                    ts_path,
                    crate_version.as_str(),
                    crate_edition.as_str(),
                    is_concurrent,
                    crates,
                )
                .await
                {
                    panic!("Generate proxy crate failed, reason: {:#?}", e);
                } else {
                    sender.send(true);
                }
            }
        }
    });

    if let Err(e) = receiver.recv() {
        panic!("Generate proxy crate failed, reaosn: {:?}", e);
    }
}
