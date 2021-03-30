use crate::GfxTask;
use std::error::Error;
use std::fmt;
use crate::back::resource::task::ResourceId;
use crate::back::resource::task::add::AddTask;
use crate::data_src::DataSource;

/// Wrapper for SendTask object. Provides RAII wrappers around resource ids.
#[derive(Debug, Clone)]
pub struct GfxHandler<TaskSender: SendTask> {
    task_sender: TaskSender,
}

impl<TaskSender: SendTask> GfxHandler<TaskSender> {
    pub fn add_resource<T: ResourceId>(&mut self, data_src: Box<dyn DataSource<T::Data>>) -> UniqueResource<T> {

        AddTask::new()
    }

    pub fn get_resource_data(&mut self) {}

}

pub trait SendTask: Clone + Send {
    fn send(&mut self, task: GfxTask) -> Result<(), SendTaskError>;
}

#[derive(Debug)]
pub struct SendTaskError {
    not_sent_task: GfxTask,
}

impl SendTaskError {
    pub fn new(not_sent_task: GfxTask) -> Self {
        Self { not_sent_task }
    }

    pub fn into_not_sent_task(self) -> GfxTask {
        self.not_sent_task
    }
}

impl Error for SendTaskError {}

impl fmt::Display for SendTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "can't send task: receiver dropped")
    }
}

pub struct UniqueResource<Resource: ResourceId, TaskSender: SendTask> {
    id: Resource,
    task_sender: TaskSender
}