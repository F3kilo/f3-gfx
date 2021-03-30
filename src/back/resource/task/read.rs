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
    pub fn new(id: R, result_setter: ReadResultSetter<R::Data>) -> Self {
        Self { id, result_setter }
    }
}

#[derive(Debug, Error)]
pub enum ReadError {}
