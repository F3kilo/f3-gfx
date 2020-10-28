use crate::common::id_counter::get_unique_id;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use subranges::interval::Interval;
use subranges::Subranges;
use vulkano::buffer::{CpuAccessibleBuffer, TypedBufferAccess};

#[derive(Clone)]
pub struct CpuBuffer<T> {
    buffer: Arc<CpuAccessibleBuffer<[T]>>,
    buffer_intervals: Arc<Mutex<BufferIntervals>>,
}

impl<T: Copy + Send + Sync + 'static> CpuBuffer<T> {
    pub fn new(buffer: Arc<CpuAccessibleBuffer<[T]>>) -> Self {
        let intervals = BufferIntervals::new(buffer.len());
        Self {
            buffer,
            buffer_intervals: Arc::new(Mutex::new(intervals)),
        }
    }

    pub fn write(&self, data: &[T]) -> Option<u64> {
        let fill_result = self
            .buffer_intervals
            .lock()
            .unwrap()
            .fill_interval(data.len() as i64);

        fill_result.map(|(int, id)| {
            let start = int.start() as usize;
            let end = int.end() as usize;
            let slice = &mut self.buffer.write().unwrap()[start..end];
            slice.copy_from_slice(data);
            id
        })
    }

    pub fn get_interval(&self, id: u64) -> Option<Interval> {
        self.buffer_intervals.lock().unwrap().get_interval(id)
    }

    pub fn get_data(&self, id: u64) -> Option<Vec<T>> {
        let buffer = self.buffer.read().unwrap();
        let interval = self.get_interval(id);
        interval.map(|i| {
            let start = i.start() as usize;
            let end = i.end() as usize;
            buffer[start..end].to_vec()
        })
    }
    
    pub fn remove(&self, id: u64) {
        self.buffer_intervals.lock().unwrap().free_interval(id)
    }
}

struct BufferIntervals {
    ranges: Subranges,
    intervals: HashMap<u64, Interval>,
}

impl BufferIntervals {
    pub fn new(size: usize) -> Self {
        let ranges = Subranges::new(Interval::new(0, size as i64));
        Self {
            ranges,
            intervals: HashMap::new(),
        }
    }

    pub fn fill_interval(&mut self, len: i64) -> Option<(Interval, u64)> {
        self.ranges.take_free_subrange(len).map(|i| {
            let id = get_unique_id();
            self.intervals.insert(id, i);
            (i, id)
        })
    }

    pub fn get_interval(&self, id: u64) -> Option<Interval> {
        self.intervals.get(&id).copied()
    }

    pub fn free_interval(&mut self, interval_id: u64) {
        let int = self.intervals.remove(&interval_id);
        if let Some(int) = int {
            self.ranges.erase_subrange(int)
        }
    }
}
