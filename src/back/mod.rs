pub mod present;
pub mod resource;

use crate::back::present::PresentTask;
use crate::back::resource::mesh::MeshResource;
use std::fmt;
use crate::GfxError;
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
    fn set(&mut self, result: TaskResult<Result>);
}

/// Error represents that task can't be complete.
#[derive(Debug, Error, Clone)]
pub enum TaskError {
    #[error("graphics backend has not enough resources to complete task")]
    NotEnoughResources,
    #[error("unexpected graphics backend error")]
    BackendError,
}

pub type TaskResult<R> = Result<R, TaskError>;