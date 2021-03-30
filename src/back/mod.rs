pub mod resource;
use std::fmt;
use crate::back::resource::mesh::MeshResource;

/// Gfx backend task
#[derive(Debug)]
pub enum BackendTask {
    Resource(ResourceType),
    Present,
}

/// Graphics resource
#[derive(Debug)]
pub enum ResourceType {
    Mesh(MeshResource)
}

/// Gfx backend
pub trait GfxBackend: fmt::Debug + Send {
    fn run_task(&mut self, task: BackendTask);
    fn update(&mut self);
}

/// Trait describe sender of some result
pub trait ResultSetter<Result: Send + 'static>: fmt::Debug + Send {
    fn set(&mut self, result: Result);
}