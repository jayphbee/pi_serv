use atom;
use pi_vm::adapter::{JSType, JS};
use pi_vm::bonmgr::{jstype_ptr, ptr_jstype, BonMgr, CallResult, FnMeta, StructMeta};
use std::convert::From;
use std::mem::transmute;
use std::sync::Arc;
use From;

fn call_1574906633(js: Arc<JS>, v: Vec<JSType>) -> Option<CallResult> {
    let param_error = "param error in from";

    let jst0 = &v[0];
    if !jst0.is_string() {
        return Some(CallResult::Err(String::from(param_error)));
    }
    let jst0 = jst0.get_str();

    let result = atom::Atom::from(jst0);
    let mut result = js.new_str((*result).clone());

    Some(CallResult::Ok)
}
pub fn register(mgr: &BonMgr) {
    mgr.regist_fun_meta(FnMeta::CallArg(call_1574906633), 1574906633);
}
