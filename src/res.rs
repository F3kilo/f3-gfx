use crate::back::resource::task::remove::RemoveTask;
use crate::back::resource::task::ResId;
use crate::handler::TaskSender;
use crate::GfxTask;
use std::sync::{Arc, Mutex};

/// Clonable RAII graphics resource reference.
#[derive(Debug, Clone)]
pub struct GfxResource<R: ResId> {
    inner: Arc<UniqueGfxResource<R>>,
}

impl<R: ResId> GfxResource<R> {
    /// Creates new graphics resource.
    pub fn new(id: R, task_sender: TaskSender) -> Self {
        Self {
            inner: Arc::new(UniqueGfxResource::new(id, task_sender)),
        }
    }

    /// Get id of resource. Using raw id is dangerous.
    pub fn id(&self) -> R {
        self.inner.id()
    }
}

/// RAII graphics resource.
#[derive(Debug)]
struct UniqueGfxResource<R: ResId> {
    id: R,
    task_sender: Mutex<TaskSender>,
}

impl<R: ResId> UniqueGfxResource<R> {
    pub fn new(id: R, task_sender: TaskSender) -> Self {
        Self {
            id,
            task_sender: Mutex::new(task_sender),
        }
    }

    pub fn id(&self) -> R {
        self.id
    }
}

impl<R: ResId> Drop for UniqueGfxResource<R> {
    fn drop(&mut self) {
        let remove_task = RemoveTask::new(self.id);
        self.task_sender
            .get_mut()
            .expect("Try to drop unique resource twice")
            .send(GfxTask::Backend(R::remove(remove_task)));
    }
}
