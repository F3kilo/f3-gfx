use crate::back::resource::task::add::AddTask;
use crate::back::resource::task::read::ReadTask;
use crate::back::resource::task::remove::RemoveTask;
use crate::back::resource::task::{ResId, ResourceTask};
use crate::back::{BackendTask, ResourceType};
use std::mem;

/// Variants of mesh resource.
#[derive(Debug)]
pub enum MeshResource {
    StaticMesh(ResourceTask<StaticMeshId>),
}

/// Provides information about mesh data sizes.
pub trait MeshData {
    fn one_vertex_size() -> usize;
    fn vertex_count(&self) -> usize;

    fn vertices_size(&self) -> usize {
        Self::one_vertex_size() * self.vertex_count()
    }

    fn one_index_size() -> usize;
    fn index_count(&self) -> usize;

    fn indices_size(&self) -> usize {
        Self::one_index_size() * self.index_count()
    }
}

/// Unique identifier of static mesh resource.
#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Debug, Copy, Clone)]
pub struct StaticMeshId(u64);

impl From<u64> for StaticMeshId {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

pub type MeshIndex = u32;

/// Data of static mesh.
#[derive(Debug, Clone)]
pub struct StaticMeshData {
    pub indices: Vec<MeshIndex>,
    pub vertices: Vec<StaticMeshVertex>,
}

impl MeshData for StaticMeshData {
    fn one_vertex_size() -> usize {
        mem::size_of::<StaticMeshVertex>()
    }

    fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    fn one_index_size() -> usize {
        mem::size_of::<MeshIndex>()
    }

    fn index_count(&self) -> usize {
        self.indices.len()
    }
}

/// Single vertex of static mesh.
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct StaticMeshVertex {
    position: [f32; 4],
    normal: [f32; 4],
    uv: [f32; 2],
}

impl ResId for StaticMeshId {
    type Data = StaticMeshData;

    fn add(task: AddTask<Self>) -> BackendTask {
        let mesh_resource = MeshResource::StaticMesh(ResourceTask::Add(task));
        BackendTask::Resource(ResourceType::Mesh(mesh_resource))
    }

    fn remove(task: RemoveTask<Self>) -> BackendTask {
        let mesh_resource = MeshResource::StaticMesh(ResourceTask::Remove(task));
        BackendTask::Resource(ResourceType::Mesh(mesh_resource))
    }

    fn read(task: ReadTask<Self>) -> BackendTask {
        let mesh_resource = MeshResource::StaticMesh(ResourceTask::Read(task));
        BackendTask::Resource(ResourceType::Mesh(mesh_resource))
    }
}

#[cfg(test)]
mod tests {
    use crate::back::resource::mesh::{MeshData, StaticMeshData};

    fn static_mesh_example() -> StaticMeshData {
        StaticMeshData {
            vertices: vec![Default::default(); 4],
            indices: vec![0; 6],
        }
    }

    #[test]
    fn static_mesh_sizes() {
        let data = static_mesh_example();
        let vertex_size = StaticMeshData::one_vertex_size();
        let index_size = StaticMeshData::one_index_size();
        assert_eq!(vertex_size, 40);
        assert_eq!(index_size, 4);
        assert_eq!(data.vertex_count(), 4);
        assert_eq!(data.index_count(), 6);
        assert_eq!(data.vertices_size(), data.vertices.len() * vertex_size);
        assert_eq!(data.indices_size(), data.indices.len() * index_size);
    }
}
