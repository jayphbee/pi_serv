use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicUsize};

use dashmap::DashMap;

lazy_static! {
    static ref BINARY_MAP: Arc<DashMap<usize, Arc<Vec<u8>>>> = Arc::new(DashMap::new());
    static ref BINARY_ID: AtomicUsize = AtomicUsize::new(0);
}

pub struct Binary {
    id: usize,
    len: usize
}

impl Binary {
    pub fn new(content: Vec<u8>) -> Binary {
        let id = get_next_binary_id();
        let len = content.len();
        BINARY_MAP.insert(id, Arc::new(content));

        Binary {
            id,
            len
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_arc_vec(&self) -> Arc<Vec<u8>> {
        BINARY_MAP.get(&self.id).unwrap().clone()
    }
}

impl Drop for Binary {
    fn drop(&mut self) {
        debug!("Binary id = {:?} dropped", self.id);
        BINARY_MAP.remove(&self.id);
    }
}

fn get_next_binary_id() -> usize {
    BINARY_ID.fetch_add(1, Ordering::SeqCst)
}
