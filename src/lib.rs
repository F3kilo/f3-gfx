pub mod back;
mod generic_gfx;
pub mod handler;
pub mod res;
pub mod scene;
pub mod task_recv;

use back::{BackendTask, GfxBackend};
use generic_gfx::{GenericGfx, WorkingGenericGfx};
use task_recv::ReceiveTask;
use thiserror::Error;

/// Gfx frontend task.
#[derive(Debug)]
pub enum GfxTask {
    Backend(BackendTask),
    Command(ServiceTask),
}

/// Gfx command task.
#[derive(Debug)]
pub enum ServiceTask {
    ChangeBackend(Box<dyn GfxBackend>),
}

/// Gfx frontend.
pub trait Gfx: Send {
    /// Run enqueued tasks.
    fn run_tasks(&mut self) -> Result<(), GfxError>;

    /// Test if gfx is still working.
    fn is_working(&self) -> bool;

    /// Update graphics. Some resources may be sent to consumers.
    /// Returns `true` if some tasks are NOT finished.
    fn update(&mut self) -> Result<bool, GfxError>;
}

/// Gfx frontend builder
pub struct GfxBuilder<TaskReceiver: ReceiveTask> {
    tasks: TaskReceiver,
    backend: Box<dyn GfxBackend>,
}

impl<TaskReceiver: ReceiveTask> GfxBuilder<TaskReceiver> {
    /// Creates new GfxBuilder with specified task receiver and backend.
    pub fn new(tasks: TaskReceiver, backend: Box<dyn GfxBackend>) -> Self {
        Self { tasks, backend }
    }

    /// Builds frontend with specified parameters.
    pub fn build(self) -> impl Gfx {
        let gfx = WorkingGenericGfx::new(self.backend, self.tasks);
        GenericGfx::Working(gfx)
    }
}

#[derive(Debug, Error)]
#[error("critical graphics error occured: {0}")]
pub struct GfxError(pub String);
