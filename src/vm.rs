//use std::sync::{Arc};
use depend::{Depend, RcFileDes, Built};
use jsloader::Loader;
use std::collections::HashMap;
use std::fs::write;
use pi_vm::adapter::JSTemplate;

pub struct Factory;

impl Factory {
    pub fn creat_vm(paths: &[String], dp: &Depend, file_map: &HashMap<String, Vec<u8>>) -> JSTemplate{
        let list = Loader::list_with_depend(paths, dp);
        let mut js_code = String::from("");
        for des in list.iter(){
            println!("des{}", des.borrow().path);
            let path = String::from(des.borrow().path.as_ref());
            let point_i = path.rfind(".");
            if path.ends_with(".js") || path.ends_with(".sjs"){
                let u8arr = file_map.get(&path).unwrap().as_slice();
                let s = String::from_utf8_lossy(u8arr);
                if s.starts_with("_$define("){
                    let p = path.get(0..point_i.unwrap()).unwrap();
                    let mut v = Vec::from(u8arr);
                    let end = v.iter().position(|&x| x == 44).unwrap() - 1;
                    v.splice(10..end, p.as_bytes().iter().cloned());
                     js_code = js_code + "\n" + &String::from_utf8_lossy(v.as_slice());
                }else{
                    js_code = js_code + "\n" + &s;
                }
            }
        }
        let jc = js_code.clone();
        let jc: Vec<&str> = jc.split("\n").collect();
        let mut jsstr = String::from("");
        for i in 1..jc.len() + 1{
            jsstr = jsstr + "\n" + i.to_string().as_str() + jc[i-1];
        }

        //js_code = js_code + "\nconsole.log(self._$modWait);";
        //js_code = js_code + "\nself._$build();";
        

        //println!("---{}", &js_code);
        write("./core.js", &js_code);
        let js = JSTemplate::new("test_vm_run_performance.js".to_string(), js_code);
        let js = js.unwrap();
        let copy = js.clone().unwrap();
        copy.run();
        js
        // let js = JSTemplate::new(js_code);
        // let js = js.unwrap();
        // let copy = Arc::new(js.clone().unwrap());
        // println!("c------------------------------------------");
        // copy.run();
        // js
    }

    pub fn depend(dp: &Depend, paths: &[String]) -> Vec<RcFileDes>{
        let mut set: Vec<RcFileDes> = Vec::new();
        let mut temp: HashMap<String, bool> = HashMap::new();

        let mut p_chain = Vec::new();
        Factory::depend_temp(dp, paths, &mut temp, &mut set, &mut p_chain);
        set
    }

    fn depend_temp(dp: &Depend, paths: &[String], temp:&mut HashMap<String, bool>, set: &mut Vec<RcFileDes>, p_chain: &mut Vec<String>){
        let gd = |f: RcFileDes, arr: &mut Vec<RcFileDes>, temp: &mut HashMap<String, bool>|{
            if temp.contains_key(&String::from(f.borrow().path.as_ref())){
                return;
            }
            arr.push(f.clone());
            temp.insert(String::from(f.borrow().path.as_ref()), true);
        };

        for i in 0..paths.len(){
            let path = paths[i].as_str();
            if is_exist(p_chain, path){
                continue;
            }
            p_chain.push(String::from(path));
            let mut f = dp.get(path);
            if f.is_none(){
                //println!("{}", path);
                let l = path.len();
                let mut p = String::from(path);
                p.splice(l-2..l, "sjs");
                f = dp.get(&p);
            }
            if f.is_some(){
                let f = f.unwrap().clone();
                let f_ref = f.borrow();
                if f_ref.depend.is_some(){
                    let depend = f_ref.depend.as_ref().unwrap();
                    let js_depend = depend.get("js");
                    //let sjs_depend = depend.get("sjs");

                    // if sjs_depend.is_some(){
                    //     let depend = sjs_depend.unwrap();
                    //     let depend_path: Vec<String> = depend.iter().map(|e|{Built::relative_path(&(e.clone() + ".sjs"), path)}).collect();
                    //     Factory::depend_temp(dp, depend_path.as_slice(), temp, set, p_chain);
                    // }

                    if js_depend.is_some(){
                        let depend = js_depend.unwrap();
                        let depend_path: Vec<String> = depend.iter().map(|e|{Built::relative_path(&(e.clone() + ".js"), path)}).collect();
                        Factory::depend_temp(dp, depend_path.as_slice(), temp, set, p_chain); 
                    }
                }

                gd(f.clone(), set, temp);
            }else{
                panic!("依赖列表中不存在该文件{}", paths[i]);
            }
            let l = p_chain.len();
            p_chain.remove(l - 1);
        }
    }
}

fn is_exist(v: &Vec<String>, s: &str) -> bool{
    for vv in v.iter(){
        if vv == s{
            return true
        }
    }
    false
}