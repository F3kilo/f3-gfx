use crate::back::resource::task::{ResourceId, ResourceTask};

/// Variants of mesh resource
#[derive(Debug)]
pub enum MeshResource {
    StaticMesh(ResourceTask<StaticMeshId>)
}

/// Unique identifier of static mesh resource
#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub struct StaticMeshId(u64);

/// Data of static mesh
#[derive(Debug)]
pub struct StaticMeshData {
    indices: Vec<u32>,
    vertex_data: Vec<StaticMeshVertex>,
}

/// Single vertex of static mesh
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct StaticMeshVertex {
    position: [f32; 4],
    normal: [f32; 4],
    uv: [f32; 2],
}

impl ResourceId for StaticMeshId {
    type Data = StaticMeshData;
}
