use f3_gfx::back::resource::mesh::{MeshResource, StaticMeshData, StaticMeshId};
use f3_gfx::back::resource::task::add::{AddResult, AddTask};
use f3_gfx::back::resource::task::read::{ReadError, ReadTask};
use f3_gfx::back::resource::task::remove::RemoveTask;
use f3_gfx::back::resource::task::{ResId, ResourceTask};
use f3_gfx::back::{BackendTask, GfxBackend, ResourceType};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

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
        let (mut data_src, mut result_setter) = task.into_inner();
        let data = futures::executor::block_on(data_src.take_data())
            .expect("Can't take data from data source");
        let id = new_id();
        self.storage.insert(id, data);
        let result = AddResult::Ok(id);
        log::trace!("Setting add result: {:?}", result);
        result_setter.set(result)
    }

    fn remove(&mut self, task: RemoveTask<StaticMeshId>) {
        log::trace!("Removing static mesh: {:?}", task);
        let id = task.into_inner();
        self.storage.remove(&id);
    }

    fn read(&mut self, task: ReadTask<StaticMeshId>) {
        log::trace!("Reading static mesh: {:?}", task);
        let (id, mut result_setter) = task.into_inner();
        let data = self.storage.get(&id).map(Clone::clone);
        let result = match data {
            Some(d) => Ok(d),
            None => Err(ReadError::NotFound),
        };
        log::trace!("Setting read result: {:?}", result);
        result_setter.set(result);
    }
}

fn new_id<T: ResId + From<u64>>() -> T {
    ID_COUNTER.fetch_add(1, Ordering::SeqCst).into()
}
