use crate::common::id_counter::get_unique_id;
use f3_gfx::back::GeomId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use subranges::interval::Interval;
use subranges::Subranges;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::device::Device;

pub struct VertexBuffer {
    vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    buffer_intervals: Arc<Mutex<BufferIntervals>>,
}

pub const MAX_VERTEX_COUNT: usize = 10000;

impl VertexBuffer {
    pub fn new(device: Arc<Device>) -> Self {
        let intervals = BufferIntervals::new(MAX_VERTEX_COUNT);
        Self {
            vertex_buffer: Self::create_vertex_buffer(device),
            buffer_intervals: Arc::new(Mutex::new(intervals)),
        }
    }

    pub fn write_vertices(&self, verts: &[Vertex]) -> Option<GeomId> {
        let fill_result = self
            .buffer_intervals
            .lock()
            .unwrap()
            .fill_interval(verts.len() as i64);

        fill_result.map(|(int, id)| {
            let start = int.start() as usize;
            let end = int.end() as usize;
            let slice = &mut self.vertex_buffer.write().unwrap()[start..end];
            slice.copy_from_slice(verts);
            id
        })
    }

    pub fn get_interval(&self, id: GeomId) -> Option<Interval> {
        self.buffer_intervals.lock().unwrap().get_interval(id)
    }

    pub fn remove_vertices(&self, id: GeomId) {
        self.buffer_intervals.lock().unwrap().free_interval(id)
    }

    fn create_vertex_buffer(device: Arc<Device>) -> Arc<CpuAccessibleBuffer<[Vertex]>> {
        let vertices = [Vertex::default(); MAX_VERTEX_COUNT];
        CpuAccessibleBuffer::from_iter(
            device,
            BufferUsage::vertex_buffer_transfer_destination(),
            false,
            vertices.iter().cloned(),
        )
        .unwrap()
    }
}

struct BufferIntervals {
    ranges: Subranges,
    intervals: HashMap<GeomId, Interval>,
}

impl BufferIntervals {
    pub fn new(size: usize) -> Self {
        let ranges = Subranges::new(Interval::new(0, size as i64));
        Self {
            ranges,
            intervals: HashMap::new(),
        }
    }

    pub fn fill_interval(&mut self, len: i64) -> Option<(Interval, GeomId)> {
        self.ranges.take_free_subrange(len).map(|i| {
            let id = get_unique_id();
            self.intervals.insert(id, i);
            (i, id)
        })
    }

    pub fn get_interval(&self, id: GeomId) -> Option<Interval> {
        self.intervals.get(&id).copied()
    }

    pub fn free_interval(&mut self, interval_id: GeomId) {
        let int = self.intervals.remove(&interval_id);
        if let Some(int) = int {
            self.ranges.erase_subrange(int)
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Vertex {
    position: [f32; 3],
}

vulkano::impl_vertex!(Vertex, position);
