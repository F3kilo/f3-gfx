use crate::back::resource::task::{DynResultSetter, ResId};
use thiserror::Error;

pub type ReadResult<Res> = Result<Res, ReadError>;
pub type ReadResultSetter<Res> = DynResultSetter<ReadResult<Res>>;

#[derive(Debug)]
pub struct ReadTask<R: ResId> {
    id: R,
    result_setter: ReadResultSetter<R::Data>,
}

impl<R: ResId> ReadTask<R> {
    /// Creates new Read task for graphics backend.
    pub fn new(id: R, result_setter: ReadResultSetter<R::Data>) -> Self {
        Self { id, result_setter }
    }

    /// Takes data source and result setter from `self`.
    pub fn into_inner(self) -> (R, ReadResultSetter<R::Data>) {
        (self.id, self.result_setter)
    }
}

/// Error represent some problem in process of reading resource data from graphics backend.
#[derive(Debug, Error)]
pub enum ReadError {}
