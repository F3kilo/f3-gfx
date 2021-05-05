use f3_gfx::back::present::PresentTask;
use f3_gfx::back::resource::mesh::{MeshResource, StaticMeshData, StaticMeshId};
use f3_gfx::back::resource::task::add::{AddTask};
use f3_gfx::back::resource::task::read::{ReadTask};
use f3_gfx::back::resource::task::remove::RemoveTask;
use f3_gfx::back::resource::task::{ResId, ResourceTask};
use f3_gfx::back::{BackendTask, GfxBackend, ResourceType, TaskError, GfxBackendUpdateError};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::atomic::{AtomicU64, Ordering};

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Default)]
pub struct DummyGfxBack {
    static_mesh_manager: StaticMeshManager,
    running_tasks: Vec<Box<dyn RunningTask>>,
}

impl GfxBackend for DummyGfxBack {
    fn run_task(&mut self, task: BackendTask) {
        log::trace!("Backend receives task: {:?}", task);
        match task {
            BackendTask::Resource(t) => self.start_resource_task(t),
            BackendTask::Present(t) => self.start_present(t),
        }
    }

    fn update(&mut self) -> Result<bool, GfxBackendUpdateError> {
        log::trace!("Polling {} backend tasks.", self.running_tasks.len());
        let remove_indices: Vec<usize> = self
            .running_tasks
            .iter_mut()
            .enumerate()
            .filter_map(|(i, t)| if t.try_finish() { Some(i) } else { None })
            .rev()
            .collect();

        for i in remove_indices {
            self.running_tasks.swap_remove(i);
        }

        log::trace!("Not finished tasks left : {}.", self.running_tasks.len());
        Ok(!self.running_tasks.is_empty())
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
            MeshResource::StaticMesh(t) => {
                t.call(&mut self.static_mesh_manager, &mut self.running_tasks)
            }
        }
    }

    fn start_present(&mut self, task: PresentTask) {
        log::trace!("Starting present task: {:?}", task);
        let (_info, scene) = task.into_inner();
        let present = Box::new(RunTask(move || {
            log::trace!("Presenting scene: {:?}", scene);
            true
        }));

        self.running_tasks.push(present);
    }
}

trait ManagerFnByTask<R: ResId> {
    fn call(self, manager: &mut impl ResourceManager<R>, running: &mut Vec<Box<dyn RunningTask>>);
}

impl<R: ResId> ManagerFnByTask<R> for ResourceTask<R> {
    fn call(self, manager: &mut impl ResourceManager<R>, running: &mut Vec<Box<dyn RunningTask>>) {
        match self {
            ResourceTask::Add(t) => manager.add(t, running),
            ResourceTask::Remove(t) => manager.remove(t),
            ResourceTask::Read(t) => manager.read(t, running),
        }
    }
}

trait ResourceManager<R: ResId> {
    fn add(&mut self, task: AddTask<R>, running: &mut Vec<Box<dyn RunningTask>>);
    fn remove(&mut self, task: RemoveTask<R>);
    fn read(&mut self, task: ReadTask<R>, running: &mut Vec<Box<dyn RunningTask>>);
}

#[derive(Debug, Default)]
struct StaticMeshManager {
    storage: HashMap<StaticMeshId, StaticMeshData>,
}

impl ResourceManager<StaticMeshId> for StaticMeshManager {
    fn add(&mut self, task: AddTask<StaticMeshId>, running: &mut Vec<Box<dyn RunningTask>>) {
        let (data, result_setter) = task.into_inner();
        let id = new_id();
        self.storage.insert(id, data);
        let mut once_res_set = Some(result_setter);
        let set_result_task = Box::new(RunTask(move || {
            log::trace!("Setting add task result: {:?}", id);
            once_res_set.take().unwrap().set_resource(id);
            true
        }));

        log::trace!("Push add task to running task.");
        running.push(set_result_task);
    }

    fn remove(&mut self, task: RemoveTask<StaticMeshId>) {
        log::trace!("Removing static mesh: {:?}", task);
        let id = task.into_inner();
        self.storage.remove(&id);
    }

    fn read(&mut self, task: ReadTask<StaticMeshId>, running: &mut Vec<Box<dyn RunningTask>>) {
        log::trace!("Reading static mesh: {:?}", task);
        let (id, result_setter) = task.into_inner();
        let data = self.storage.get(&id).map(Clone::clone);
        let result = match data {
            Some(d) => Ok(d),
            None => Err(TaskError::BackendBroken),
        };

        let mut once_res_set = Some(result_setter);
        let set_result_task = Box::new(RunTask(move || {
            log::trace!("Setting read result: {:?}", result);
            once_res_set.take().unwrap().set(result.clone());
            true
        }));

        log::trace!("Push read task to running task.");
        running.push(set_result_task);
    }
}

fn new_id<T: ResId + From<u64>>() -> T {
    ID_COUNTER.fetch_add(1, Ordering::SeqCst).into()
}

trait RunningTask: Send + Debug {
    fn try_finish(&mut self) -> bool;
}

struct RunTask<T: FnMut() -> bool + Send>(T);

impl<T: FnMut() -> bool + Send> Debug for RunTask<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Runing task")
    }
}

impl<T: FnMut() -> bool + Send> RunningTask for RunTask<T> {
    fn try_finish(&mut self) -> bool {
        (self.0)()
    }
}
