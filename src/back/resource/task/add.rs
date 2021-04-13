use crate::back::resource::task::{DynResultSetter, ResId};
use thiserror::Error;

pub type AddResult<R> = Result<R, AddError>;
pub type AddResultSetter<R> = DynResultSetter<AddResult<R>>;

#[derive(Debug)]
pub struct AddTask<R: ResId> {
    data: R::Data,
    result_setter: AddResultSetter<R>,
}

impl<R: ResId> AddTask<R> {
    /// Creates new Add task for backend resource.
    pub fn new(data: R::Data, result_setter: AddResultSetter<R>) -> Self {
        Self {
            data,
            result_setter,
        }
    }

    /// Takes data source and result setter from `self`.
    pub fn into_inner(self) -> (R::Data, AddResultSetter<R>) {
        (self.data, self.result_setter)
    }
}

/// Error represent some problem in process of adding resource to graphics backend.
#[derive(Debug, Error, Clone)]
pub enum AddError {
    #[error("graphics backend has not enough space for resource")]
    SpaceExhausted,
    #[error("can't add resource to graphics backend")]
    CantAdd(String)
}
