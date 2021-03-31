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
    /// Creates new Add task for backend resource.
    pub fn new(data_src: Box<dyn DataSource<R::Data>>, result_setter: AddResultSetter<R>) -> Self {
        Self {
            data_src,
            result_setter,
        }
    }

    /// Takes data source and result setter from `self`.
    pub fn into_inner(self) -> (Box<dyn DataSource<R::Data>>, AddResultSetter<R>) {
        (self.data_src, self.result_setter)
    }
}

/// Error represent some problem in process of adding resource to graphics backend.
#[derive(Debug, Error, Copy, Clone)]
pub enum AddError {}
