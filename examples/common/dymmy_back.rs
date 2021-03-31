use f3_gfx::back::resource::mesh::{MeshResource, StaticMeshData, StaticMeshId};
use f3_gfx::back::resource::task::add::AddTask;
use f3_gfx::back::resource::task::read::ReadTask;
use f3_gfx::back::resource::task::remove::RemoveTask;
use f3_gfx::back::resource::task::{ResId, ResourceTask};
use f3_gfx::back::{BackendTask, GfxBackend, ResourceType};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct DummyGfxBack {
    static_mesh_manager: StaticMeshManager,
}

impl GfxBackend for DummyGfxBack {
    fn run_task(&mut self, task: BackendTask) {
        log::trace!("Backend receives task: {:?}", task);
        match task {
            BackendTask::Resource(t) => self.start_resource_task(t),
            BackendTask::Present => log::trace!("Backend receives present task."),
        }
    }

    fn poll_tasks(&mut self) {
        log::trace!("Polling backend tasks");
    }
}

impl DummyGfxBack {
    fn start_resource_task(&mut self, task: ResourceType) {
        log::trace!("Starting resource task: {:?}", task);
        match task {
            ResourceType::Mesh(t) => self.start_mesh_resource_task(t),
        }
    }

    fn start_mesh_resource_task(&mut self, task: MeshResource) {
        log::trace!("Starting mesh resource task: {:?}", task);
        match task {
            MeshResource::StaticMesh(t) => t.call(&mut self.static_mesh_manager),
        }
    }
}

trait ManagerFnByTask<R: ResId> {
    fn call(self, manager: &mut impl ResourceManager<R>);
}

impl<R: ResId> ManagerFnByTask<R> for ResourceTask<R> {
    fn call(self, manager: &mut impl ResourceManager<R>) {
        match self {
            ResourceTask::Add(t) => manager.add(t),
            ResourceTask::Remove(t) => manager.remove(t),
            ResourceTask::Read(t) => manager.read(t),
            ResourceTask::List(_) => {}
        }
    }
}

trait ResourceManager<R: ResId> {
    fn add(&mut self, task: AddTask<R>);
    fn remove(&mut self, task: RemoveTask<R>);
    fn read(&mut self, task: ReadTask<R>);
}

#[derive(Debug, Default)]
struct StaticMeshManager {
    storage: HashMap<StaticMeshId, StaticMeshData>,
}

impl ResourceManager<StaticMeshId> for StaticMeshManager {
    fn add(&mut self, task: AddTask<StaticMeshId>) {
        log::trace!("Adding static mesh: {:?}", task);
    }

    fn remove(&mut self, task: RemoveTask<StaticMeshId>) {
        log::trace!("Removing static mesh: {:?}", task);
    }

    fn read(&mut self, task: ReadTask<StaticMeshId>) {
        log::trace!("Reading static mesh: {:?}", task);
    }
}
