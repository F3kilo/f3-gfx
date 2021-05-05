pub mod back;
pub mod handler;
pub mod res;
pub mod scene;
mod task_channel;

use back::{BackendTask, GfxBackend};
use crate::task_channel::TasksChannel;

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
    pub fn update(&mut self) {
        self.backend.poll_tasks();
    }

    fn run_service_task(&mut self, task: ServiceTask) {
        todo!()
    }
}
