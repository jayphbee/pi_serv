use crate::FILES_ASYNC_RUNTIME;
use async_file::file::{AsyncFile, AsyncFileOptions};
use clap::ArgMatches;
use vm_builtin::ContextHandle;
use vm_core::vm;

use std::env;
use std::path::{Path, PathBuf};
use std::fs::read_to_string;

pub async fn init_js(init_vm: vm::Vm, handle: ContextHandle, matches: ArgMatches<'static>) {
    let init_exec_path = matches.value_of("init-file").unwrap().to_string();
    let projs = match matches.values_of("projects") {
        Some(p) => p
            .map(|s| s.to_string().replace("\\", "/"))
            .collect::<Vec<String>>(),
        None => vec![],
    };
    let current_dir = env::current_dir().unwrap();
    let current_dir_parent = current_dir.parent().unwrap().to_str().unwrap();

    let path = Path::new(&init_exec_path)
        .iter()
        .filter_map(|x| if x == "." || x == ".." { None } else { Some(x) })
        .map(|x| x.to_str().unwrap())
        .collect::<Vec<&str>>();

    let root: PathBuf = [vec![current_dir_parent], path].concat().iter().collect();
    let project_root = root
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .replace("\\", "/");

    env::set_var("PROJECTS", &projs.as_slice().join(" "));

    let cur_dir = env::current_dir();

    info!("current dir: {:?}, projects: {:?}, init-file path: {:?}", cur_dir, projs, init_exec_path);

    // 如果没有出现 -p 参数
    if matches.occurrences_of("projects") == 0 {
        env::set_var("PROJECT_ROOT", cur_dir.unwrap().to_str().unwrap());
    } else {
        env::set_var("PROJECT_ROOT", &project_root);
    }

    // let source = match AsyncFile::open(
    //     FILES_ASYNC_RUNTIME.clone(),
    //     init_exec_path.clone(),
    //     AsyncFileOptions::OnlyRead,
    // )
    // .await
    // {
    //     Ok(file) => match file.read(0, file.get_size() as usize).await {
    //         Ok(f) => f,
    //         Err(e) => {
    //             panic!("read init-file path: {:?}, error: {:?}", &init_exec_path, e);
    //         }
    //     },
    //     Err(e) => {
    //         panic!(
    //             "open init-file failed path: {:?}, error: {:?}",
    //             &init_exec_path, e
    //         );
    //     }
    // };

    // let source = match String::from_utf8(source) {
    //     Ok(s) => s,
    //     Err(e) => {
    //         panic!("init-file is not valid utf8 string, error: {:?}", e);
    //     }
    // };

    // TODO: 以后改为异步的
    let source = read_to_string(&init_exec_path).unwrap();

    if let Err(e) = init_vm
        .execute(handle, &init_exec_path, source.as_str())
        .await
    {
        panic!(
            "load init-file failed path: {:?}, error: {:?}",
            init_exec_path, e
        );
    }
}
