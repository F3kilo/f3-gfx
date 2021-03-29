use crate::back::resource::task::{DynResultSender, ResourceId};
use thiserror::Error;

pub type ListResultSender<Res> = DynResultSender<Res>;

#[derive(Debug)]
pub struct ListTask<Resource: ResourceId> {
    id: Resource,
    result_sender: ListResultSender<Resource>,
}

impl<Resource: ResourceId> ListTask<Resource> {
    pub fn new(id: Resource, result_sender: ListResultSender<Resource>) -> Self {
        Self { id, result_sender }
    }
}

#[derive(Debug, Error)]
pub enum AddError {}
