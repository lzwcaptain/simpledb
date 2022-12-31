use std::sync::{Arc, Mutex};


pub(crate) const PAGE_SIZE: usize = 4096;

#[derive(Clone)]
pub(crate) struct Page {
    pub(crate) data: Arc<Mutex<[u8; PAGE_SIZE]>>,
}

impl Page {
    pub(crate) fn get_data(&mut self) -> Arc<Mutex<[u8; 4096]>> {
        self.data.clone()
    }
}