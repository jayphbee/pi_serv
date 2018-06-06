use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use pi_vm::task::TaskType;
use pi_vm::pi_vm_impl::{block_reply, block_throw};
use std::sync::Arc;
use pi_vm;

pub fn register(mgr: &BonMgr){
}