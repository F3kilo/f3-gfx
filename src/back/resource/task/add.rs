use crate::back::resource::task::{DynResultSender, ResourceId};
use crate::data_src::DataSource;
use thiserror::Error;

pub type AddResult<Res> = Result<Res, AddError>;
pub type AddResultSender<Res> = DynResultSender<AddResult<Res>>;

#[derive(Debug)]
pub struct AddTask<Resource: ResourceId> {
    data_src: Box<dyn DataSource<Resource::Data>>,
    result_sender: AddResultSender<Resource>,
}

impl<Resource: ResourceId> AddTask<Resource> {
    pub fn new(
        data_src: Box<dyn DataSource<Resource::Data>>,
        result_sender: AddResultSender<Resource>,
    ) -> Self {
        Self {
            data_src,
            result_sender,
        }
    }
}

#[derive(Debug, Error)]
pub enum AddError {}
