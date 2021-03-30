use crate::back::resource::task::add::AddTask;
use crate::back::resource::task::list::ListTask;
use crate::back::resource::task::read::ReadTask;
use crate::back::resource::task::remove::RemoveTask;
use crate::back::resource::task::{ResId, ResourceTask};
use crate::back::{BackendTask, ResourceType};

/// Variants of mesh resource
#[derive(Debug)]
pub enum MeshResource {
    StaticMesh(ResourceTask<StaticMeshId>),
}

/// Unique identifier of static mesh resource
#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Debug, Copy, Clone)]
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

    fn list(task: ListTask<Self>) -> BackendTask {
        let mesh_resource = MeshResource::StaticMesh(ResourceTask::List(task));
        BackendTask::Resource(ResourceType::Mesh(mesh_resource))
    }
}
