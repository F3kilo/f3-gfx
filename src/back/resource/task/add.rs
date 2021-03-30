use crate::back::resource::task::{DynResultSetter, ResId};
use crate::data_src::DataSource;
use thiserror::Error;

pub type AddResult<R> = Result<R, AddError>;
pub type AddResultSetter<R> = DynResultSetter<AddResult<R>>;

#[derive(Debug)]
pub struct AddTask<R: ResId> {
    data_src: Box<dyn DataSource<R::Data>>,
    result_setter: AddResultSetter<R>,
}

impl<R: ResId> AddTask<R> {
    pub fn new(data_src: Box<dyn DataSource<R::Data>>, result_setter: AddResultSetter<R>) -> Self {
        Self {
            data_src,
            result_setter,
        }
    }
}

#[derive(Debug, Error)]
pub enum AddError {}
