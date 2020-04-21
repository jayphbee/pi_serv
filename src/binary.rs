use std::sync::atomic::{Ordering, AtomicUsize};

use dashmap::DashMap;

lazy_static! {
    static ref BINARY_MAP: DashMap<usize, Vec<u8>> = DashMap::new();
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
        BINARY_MAP.insert(id, content);

        Binary {
            id,
            len
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn take(&self) -> Option<Vec<u8>> {
        BINARY_MAP.remove(&self.id).map(|(_, val)| val)
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
