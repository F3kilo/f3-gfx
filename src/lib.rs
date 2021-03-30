use back::{BackendTask, GfxBackend};
use generic_gfx::{GenericGfx, WorkingGenericGfx};
use task_recv::ReceiveTask;

pub mod back;
mod generic_gfx;
pub mod task_recv;
pub mod data_src;
pub mod handler;

/// Gfx frontend task
#[derive(Debug)]
pub enum GfxTask {
    Backend(BackendTask),
    Command(ServiceTask),
}

/// Gfx command task
#[derive(Debug)]
pub enum ServiceTask {
    ChangeBackend(Box<dyn GfxBackend>),
}

/// Gfx frontend
pub trait Gfx {
    /// Run enqueued tasks
    fn run_tasks(&mut self);
    
    /// Test if gfx is still working
    fn is_working(&self) -> bool;
    
    /// Update graphics. Some resources may be sent to consumers.
    fn update(&mut self);
}

/// Gfx frontend builder
pub struct GfxBuilder<TaskReceiver: ReceiveTask> {
    tasks: TaskReceiver,
    backend: Box<dyn GfxBackend>,
}

impl<TaskReceiver: ReceiveTask> GfxBuilder<TaskReceiver> {
    /// Builds frontend with specified parameters
    pub fn build(self) -> impl Gfx {
        let gfx = WorkingGenericGfx::new(self.backend, self.tasks);
        GenericGfx::Working(gfx)
    }
}
