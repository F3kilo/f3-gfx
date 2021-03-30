use crate::back::resource::task::{DynResultSetter, ResourceId};
use thiserror::Error;

pub type ListResultSetter<Res> = DynResultSetter<Res>;

#[derive(Debug)]
pub struct ListTask<Resource: ResourceId> {
    id: Resource,
    result_setter: ListResultSetter<Resource>,
}

impl<Resource: ResourceId> ListTask<Resource> {
    pub fn new(id: Resource, result_setter: ListResultSetter<Resource>) -> Self {
        Self { id, result_setter }
    }
}

#[derive(Debug, Error)]
pub enum AddError {}
