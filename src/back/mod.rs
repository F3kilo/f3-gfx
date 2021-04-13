pub mod present;
pub mod resource;

use crate::back::present::PresentTask;
use crate::back::resource::mesh::MeshResource;
use std::fmt;
use crate::GfxError;

/// Gfx backend task
#[derive(Debug)]
pub enum BackendTask {
    Resource(ResourceType),
    Present(PresentTask),
}

/// Graphics resource
#[derive(Debug)]
pub enum ResourceType {
    Mesh(MeshResource),
}

/// Gfx backend
pub trait GfxBackend: fmt::Debug + Send {
    /// Starts non-blocking execution of `task`.
    fn run_task(&mut self, task: BackendTask) -> Result<(), GfxError>;

    /// Checks if task is ready and sends it's result.
    /// Returns true if some tasks are NOT finish.
    fn update(&mut self) -> Result<bool, GfxError>;
}

/// Trait describe setter of some result
pub trait ResultSetter<Result: Send + 'static>: fmt::Debug + Send {
    fn set(&mut self, result: Result);
}
