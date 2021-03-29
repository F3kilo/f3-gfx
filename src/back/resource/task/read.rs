use crate::back::resource::task::{DynResultSender, ResourceId};
use thiserror::Error;

pub type ReadResult<Res> = Result<Res, ReadError>;
pub type ReadResultSender<Res> = DynResultSender<ReadResult<Res>>;

#[derive(Debug)]
pub struct ReadTask<Resource: ResourceId> {
    id: Resource,
    result_sender: ReadResultSender<Resource::Data>,
}

impl<Resource: ResourceId> ReadTask<Resource> {
    pub fn new(id: Resource, result_sender: ReadResultSender<Resource::Data>) -> Self {
        Self { id, result_sender }
    }
}

#[derive(Debug, Error)]
pub enum ReadError {}
