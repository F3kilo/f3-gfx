pub mod resource;
use std::fmt;
use crate::back::resource::mesh::MeshResource;

/// Gfx backend task
#[derive(Debug)]
pub enum BackendTask {
    Resource(Resource),
    Present,
}

/// Graphics resource
#[derive(Debug)]
pub enum Resource {
    Mesh(MeshResource)
}

/// Gfx backend
pub trait GfxBackend: fmt::Debug {
    fn run_task(&mut self, task: BackendTask);
    fn update(&mut self);
}

/// Trait describe sender of some result
pub trait ResultSender<Result: Send>: fmt::Debug {
    fn send(&mut self);
}