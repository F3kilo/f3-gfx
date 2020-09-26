use std::sync::atomic::{AtomicU64, Ordering};

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn get_unique_id<T: From<u64>>() -> T {
    let next_id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    next_id.into()
}
