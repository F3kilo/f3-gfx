use crate::back::resource::task::{DynResultSetter, ResourceId};
use thiserror::Error;

pub type ReadResult<Res> = Result<Res, ReadError>;
pub type ReadResultSetter<Res> = DynResultSetter<ReadResult<Res>>;

#[derive(Debug)]
pub struct ReadTask<Resource: ResourceId> {
    id: Resource,
    result_setter: ReadResultSetter<Resource::Data>,
}

impl<Resource: ResourceId> ReadTask<Resource> {
    pub fn new(id: Resource, result_setter: ReadResultSetter<Resource::Data>) -> Self {
        Self { id, result_setter }
    }
}

#[derive(Debug, Error)]
pub enum ReadError {}
