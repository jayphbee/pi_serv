use std::collections::HashMap;
use depend::{Depend, RcFileDes};
use std::path::Path;
use std::fs::read;
use std::convert::From;

//use pi_base::util::now_millisecond;

pub struct Loader;

impl Loader {
	pub fn list_with_depend(dirs: &[String], dp: &Depend) -> Vec<String>{
		let mut mod_names: Vec<String> = Vec::new();
		for dir in dirs.iter(){
			//let dir = &dp.get_path(&dir);
            //println!("list_with_depend----------------------------{}", dir);
			let f = dp.get(dir);
			if f.is_none() {
				panic!("找不到文件或目录：{}", dp.get_path(dir));
			}
			Loader::list_dir(f.unwrap(), &mut mod_names);
		}

		let r = dp.depend(mod_names);
        r
	}

	pub fn load_dir<F>(dirs: &[String], dp: &Depend, mut success: F)where F: FnMut(HashMap<String, Vec<u8>>){
		let file_list = Loader::list_with_depend(dirs, dp);
		let mut file_map: HashMap<String,Vec<u8>> = HashMap::new();
		for v in file_list.into_iter(){
			let path = dp.get_path(&v);
			let r = read(Path::new(&path));
			let data = r.expect((String::from("文件不存在！,path:") + &path).as_str());
			//file_map.insert(p, modify_code(&path, data));
            file_map.insert(v, data);
		}
		success(file_map);
	}

	pub fn load_dir_sync(dirs: &[String], dp: &Depend) -> HashMap<String, Vec<u8>>{
		let file_list = Loader::list_with_depend(dirs, dp);
		let mut file_map: HashMap<String, Vec<u8>> = HashMap::new();
		for v in file_list.into_iter(){
			let path = dp.get_path(&v);
            //println!("----------p:{}, path:{}", &p, &path);
            if path.ends_with(".js") {
                let r = read(Path::new(&path));
                let data = r.expect((String::from("文件不存在！,path:") + &path).as_str());
                //file_map.insert(p, modify_code(&path, data));
                file_map.insert(v, data);
            }
		}
		file_map
	}

	// pub fn load_dir_async<F>(dirs: &[&str], dp: Depend, file_map: &mut HashMap<String, Rc<Vec<u8>>>, mut success: F) where F: FnMut(&Vec<String>, &HashMap<String, Rc<Vec<u8>>>){
	// 	let mut mod_names: Vec<String> = Vec::new();
	// 	for dir in dirs.iter(){
	// 		let depend = dp.get(dir);
	// 		match depend {
	// 			Some(v) => match v.borrow().children {
	// 				Some(_) => Loader::list_dir(v, &mut mod_names),
	// 				None => {
	// 					mod_names.push(String::from(v.borrow().path.as_ref()));
	// 				},
	// 			},
	// 			None => {
	// 				continue;
	// 			},
	// 		}
	// 	}

	// 	let file_list = Loader::list_with_depend(dirs, dp);

	// 	let mut map = HashMap::new();
	// 	mod_names.clear();
	// 	for v in file_list.iter(){
	// 		let path = String::from(v.borrow().path.as_ref());
	// 		if path.ends_with("js") ||path.ends_with("sjs") {
	// 			mod_names.push(String::from(path.as_str()));
	// 		}

	// 		if file_map.contains_key(&path){
	// 			let v = file_map.get(&path).unwrap();
	// 			map.insert(String::from(path.as_str()), v.clone());
	// 		}else{
	// 			let r = read(Path::new(&path));
	// 			let data = Rc::new(r.expect("文件不存在！"));
	// 			file_map.insert(String::from(path.as_str()), data.clone());
	// 			map.insert(String::from(path.as_str()), data);
	// 		}
	// 	}

	// 	success(&mod_names, &map);
	// }

    pub fn list(dirs: &[String], dp: &Depend) -> Vec<String>{
        let mut mod_names: Vec<String> = Vec::new();
        for dir in dirs.iter(){
            //let dir = dp.get_path(&dir);
            let f = dp.get(&dir);
            if f.is_none() {
                panic!("找不到文件：{}", dir);
            }

            Loader::list_dir(f.unwrap(), &mut mod_names);
        }
        mod_names
    }

	pub fn list_dir(info: &RcFileDes, mod_names: &mut Vec<String>){
		let info_ref = info.borrow();
		let children = info_ref.children.as_ref(); 
		match children{
			Some(v) => {
				for child in v.values(){
					Loader::list_dir(child, mod_names);
				}
			},
			None => {
				mod_names.push(<String as From<&str>>::from(info_ref.path.as_ref()));
			},
		}
	}
}