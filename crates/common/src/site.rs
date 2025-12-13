use core::sync::atomic::AtomicUsize;

use typed_index_collections::TiVec;

use crate::impl_idx;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Set the counter to a specific ID, should be used after serialization
pub fn set_counter(value: usize) {
    COUNTER.store(value, core::sync::atomic::Ordering::SeqCst);
}

pub fn get_counter() -> usize {
    COUNTER.load(core::sync::atomic::Ordering::SeqCst)
}

pub type Sites = TiVec<SiteIdx, Site>;

impl_idx!(SiteIdx);

#[derive(Clone, Copy, Debug)]
pub struct Site {
    pub x: i32,
    pub y: i32,
    pub id: usize,
}

impl Site {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            id: COUNTER.fetch_add(1, core::sync::atomic::Ordering::SeqCst),
        }
    }
}
