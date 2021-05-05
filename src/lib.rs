pub mod back;
pub mod handler;
pub mod res;
pub mod scene;
mod task_channel;

use crate::task_channel::TasksChannel;
use back::{BackendTask, GfxBackend};
use crate::handler::GfxHandler;
use crate::back::GfxBackendUpdateError;

/// Gfx frontend task.
#[derive(Debug)]
pub enum GfxTask {
    Backend(BackendTask),
    Service(ServiceTask),
}

/// Gfx command task.
#[derive(Debug)]
pub enum ServiceTask {
    ChangeBackend(Box<dyn GfxBackend>),
}

/// Gfx frontend.
pub struct Gfx {
    tasks_channel: TasksChannel,
    backend: Box<dyn GfxBackend>,
}

impl Gfx {
    /// Creates new graphics frontend.
    pub fn new(backend: Box<dyn GfxBackend>) -> Self {
        Self {
            backend,
            tasks_channel: TasksChannel::default(),
        }
    }

    pub fn create_handler(&self) -> GfxHandler {
        GfxHandler::new(self.tasks_channel.sender())
    }

    /// Run enqueued tasks.
    pub fn run_tasks(&mut self) {
        while let Some(task) = self.tasks_channel.pop() {
            match task {
                GfxTask::Backend(t) => self.backend.run_task(t),
                GfxTask::Service(t) => self.run_service_task(t),
            }
        }
    }

    /// Update graphics. Some resources may be sent to consumers.
    pub fn update(&mut self) -> Result<(), GfxBackendUpdateError> {
        self.backend.update()?;
        Ok(())
    }

    fn run_service_task(&mut self, _task: ServiceTask) {
        todo!()
    }
}
