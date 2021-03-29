use std::fmt;

/// Gfx backend task
#[derive(Debug)]
pub enum BackendTask {
    LoadResource,
    Present,
}

/// Gfx backend
pub trait GfxBackend: fmt::Debug {
    fn run_task(&mut self, task: BackendTask);
}
