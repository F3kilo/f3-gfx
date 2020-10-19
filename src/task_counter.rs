use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct TaskCounter {
    counter: Arc<AtomicU64>,
}

impl TaskCounter {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(AtomicU64::default()),
        }
    }

    pub fn count(&self) -> u64 {
        self.counter.load(Ordering::Relaxed)
    }

    pub fn inc(&mut self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
    }

    pub fn dec(&mut self) {
        self.counter.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn wait_all_ready(&self) {
        while self.count() > 0 {
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}
