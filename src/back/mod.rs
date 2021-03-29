pub mod resource;
use crate::back::resource::task::ResourceTask;
use resource::geom::GeometryId;
use std::fmt;

/// Gfx backend task
#[derive(Debug)]
pub enum BackendTask {
    Geometry(ResourceTask<GeometryId>),
    Present,
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