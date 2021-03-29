use back::{BackendTask, GfxBackend};
use generic_gfx::{GenericGfx, WorkingGenericGfx};
use task_recv::GfxTaskReceiver;

pub mod back;
mod generic_gfx;
pub mod task_recv;
pub mod data_src;

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
    
    /// Update graphics. Some resources will be sent to consumers maybe.
    fn update(&mut self);
}

/// Gfx frontend builder
pub struct GfxBuilder<TaskReceiver: GfxTaskReceiver> {
    tasks: TaskReceiver,
    backend: Box<dyn GfxBackend>,
}

impl<TaskReceiver: GfxTaskReceiver> GfxBuilder<TaskReceiver> {
    /// Builds frontend with specified parameters
    pub fn build(self) -> impl Gfx {
        let gfx = WorkingGenericGfx::new(self.backend, self.tasks);
        GenericGfx::Working(gfx)
    }
}
