use crate::back::resource::new_unique_id;
use crate::back::resource::task::add::AddTask;
use crate::back::resource::task::read::ReadTask;
use crate::back::resource::task::remove::RemoveTask;
use crate::back::resource::task::{ResId, ResourceTask};
use crate::back::{BackendTask, ResourceType};
use raw_window_handle::HasRawWindowHandle;
use std::fmt::Debug;
use std::sync::Arc;

/// Unique identifier of static mesh resource.
#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Debug, Copy, Clone)]
pub struct WindowId(u64);

impl From<u64> for WindowId {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl ResId for WindowId {
    type Data = Arc<dyn WindowHandle>;

    fn new_unique() -> Self {
        Self(new_unique_id())
    }

    fn to_raw(&self) -> u64 {
        self.0
    }

    fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    fn add(task: AddTask<Self>) -> BackendTask {
        BackendTask::Resource(ResourceType::Window(ResourceTask::Add(task)))
    }

    fn remove(task: RemoveTask<Self>) -> BackendTask {
        BackendTask::Resource(ResourceType::Window(ResourceTask::Remove(task)))
    }

    fn read(task: ReadTask<Self>) -> BackendTask {
        BackendTask::Resource(ResourceType::Window(ResourceTask::Read(task)))
    }
}

pub trait WindowHandle: Send + Sync + Debug {
    fn raw_window_handle(&self) -> &dyn HasRawWindowHandle;
}

impl<T> WindowHandle for T where T: HasRawWindowHandle + Send + Sync + Debug {
    fn raw_window_handle(&self) -> &dyn HasRawWindowHandle {
        self
    }
}
