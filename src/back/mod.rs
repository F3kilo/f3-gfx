pub mod present;
pub mod resource;

use crate::back::present::PresentTask;
use crate::back::resource::mesh::MeshResource;
use std::fmt;
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum GfxBackendUpdateError {
    #[error("critical gfx backend error: {0}")]
    CriticalError(String)
}

/// Gfx backend
pub trait GfxBackend: fmt::Debug + Send {
    /// Starts non-blocking execution of `task`.
    fn run_task(&mut self, task: BackendTask);

    /// Checks if some of tasks is ready and sends it's result.
    /// Returns true if some tasks are NOT finish.
    fn update(&mut self) -> Result<bool, GfxBackendUpdateError>;
}

/// Error represents that task can't be complete.
#[derive(Debug, Error, Clone)]
pub enum TaskError {
    #[error("need to free some resources to complete task")]
    NotEnoughResources,
    #[error("backend can't complete tasks anymore")]
    BackendBroken,
}

pub type TaskResult<R> = Result<R, TaskError>;