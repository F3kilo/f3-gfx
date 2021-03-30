use crate::back::resource::task::{DynResultSetter, ResourceId};
use crate::data_src::DataSource;
use thiserror::Error;

pub type AddResult<Res> = Result<Res, AddError>;
pub type AddResultSetter<Res> = DynResultSetter<AddResult<Res>>;

#[derive(Debug)]
pub struct AddTask<Resource: ResourceId> {
    data_src: Box<dyn DataSource<Resource::Data>>,
    result_setter: AddResultSetter<Resource>,
}

impl<Resource: ResourceId> AddTask<Resource> {
    pub fn new(
        data_src: Box<dyn DataSource<Resource::Data>>,
        result_setter: AddResultSetter<Resource>,
    ) -> Self {
        Self {
            data_src,
            result_setter,
        }
    }
}

#[derive(Debug, Error)]
pub enum AddError {}
