use crate::common::id_counter::get_unique_id;
use crate::common::vulkano_back::cpu_buf::CpuBuffer;
use f3_gfx::back::{GeomData, GeomId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use subranges::interval::Interval;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::device::Device;

type VertId = u64;
type IndexId = u64;

pub const MAX_VERTEX_COUNT: usize = 10000;
pub const MAX_INDEX_COUNT: usize = 10000;

pub struct GeomBuffer {
    verts: CpuBuffer<ColVert>,
    indices: CpuBuffer<u32>,
    ids: Arc<Mutex<HashMap<GeomId, (VertId, IndexId)>>>,
}

impl GeomBuffer {
    pub fn new(device: Arc<Device>) -> Self {
        let vert_buf = Self::create_vertex_buffer(device.clone());
        let inds_buf = Self::create_index_buffer(device);
        let verts = CpuBuffer::new(vert_buf);
        let indices = CpuBuffer::new(inds_buf);
        Self {
            verts,
            indices,
            ids: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn write(&self, data: GeomData) -> Option<GeomId> {
        let (v_data, i_data) = (data.vertices, data.indices);
        let v_data: Vec<ColVert> = v_data.into_iter().map(|v| v.into()).collect();
        let v_id = self.verts.write(v_data.as_slice());
        if let Some(v_id) = v_id {
            let i_id = self.indices.write(i_data.as_slice());
            if let Some(i_id) = i_id {
                let g_id = get_unique_id();
                self.ids.lock().unwrap().insert(g_id, (v_id, i_id));
                return Some(g_id);
            }
            self.verts.remove(v_id);
        };
        None
    }

    pub fn get_geom_info(&self, id: GeomId) -> Option<GeomInfo> {
        if let Some((v_id, i_id)) = self.ids.lock().unwrap().get(&id) {
            let v_interval = self.verts.get_interval(*v_id).unwrap();
            let i_interval = self.indices.get_interval(*i_id).unwrap();
            return Some(GeomInfo {
                vertices: v_interval,
                indices: i_interval,
            });
        }
        None
    }

    pub fn get_geom_data(&self, id: GeomId) -> Option<GeomData> {
        if let Some((v_id, i_id)) = self.ids.lock().unwrap().get(&id) {
            let v_data = self.verts.get_data(*v_id).unwrap();
            let i_data = self.indices.get_data(*i_id).unwrap();
            return Some(GeomData {
                vertices: v_data.into_iter().map(|v| v.into()).collect(),
                indices: i_data,
            });
        }
        None
    }

    pub fn remove(&self, id: GeomId) {
        let ids = self.ids.lock().unwrap().remove(&id);
        if let Some((v_id, i_id)) = ids {
            self.verts.remove(v_id);
            self.indices.remove(i_id);
        }
    }

    fn create_vertex_buffer(device: Arc<Device>) -> Arc<CpuAccessibleBuffer<[ColVert]>> {
        let vertices = [ColVert::default(); MAX_VERTEX_COUNT];
        CpuAccessibleBuffer::from_iter(
            device,
            BufferUsage::vertex_buffer(),
            false,
            vertices.iter().copied(),
        )
        .unwrap()
    }

    fn create_index_buffer(device: Arc<Device>) -> Arc<CpuAccessibleBuffer<[u32]>> {
        let vertices = [0; MAX_INDEX_COUNT];
        CpuAccessibleBuffer::from_iter(
            device,
            BufferUsage::index_buffer(),
            false,
            vertices.iter().copied(),
        )
        .unwrap()
    }
}

pub struct GeomInfo {
    vertices: Interval,
    indices: Interval,
}

#[derive(Default, Copy, Clone)]
pub struct ColVert {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

vulkano::impl_vertex!(ColVert, position, color);

impl From<f3_gfx::back::ColVert> for ColVert {
    fn from(v: f3_gfx::back::ColVert) -> Self {
        ColVert {
            position: v.position,
            color: v.color,
        }
    }
}

impl From<ColVert> for f3_gfx::back::ColVert {
    fn from(v: ColVert) -> Self {
        f3_gfx::back::ColVert {
            position: v.position,
            color: v.color,
        }
    }
}
