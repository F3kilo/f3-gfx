use std::sync::atomic::{AtomicU64, Ordering};

pub mod mesh;
pub mod task;

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn new_unique_id() -> u64 {
    ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}
