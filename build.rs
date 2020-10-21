use std::env;
use std::fs::{copy, create_dir_all};

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

            if !lib_path.exists() {
                //指定路径的库文件不存在，则复制
                if let Err(e) = copy(&origin_dir, target_dir.join(lib_name)) {
                    eprintln!("==> Build debug failed, origin path: {:?}, target path: {:?}, reason: {:?}", &origin_dir, target_dir, e);
                }
            }
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

            if !lib_path.exists() {
                //指定路径的库文件不存在，则复制
                if let Err(e) = copy(&origin_dir, target_dir.join(lib_name)) {
                    eprintln!("==> Build debug failed, origin path: {:?}, target path: {:?}, reason: {:?}", &origin_dir, target_dir, e);
                }
            }
        }
    }
}
