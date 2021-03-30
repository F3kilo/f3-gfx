use crate::back::resource::task::{DynResultSetter, ResId};
use thiserror::Error;

pub type ListResultSetter<Res> = DynResultSetter<Res>;

#[derive(Debug)]
pub struct ListTask<R: ResId> {
    id: R,
    result_setter: ListResultSetter<R>,
}

impl<R: ResId> ListTask<R> {
    pub fn new(id: R, result_setter: ListResultSetter<R>) -> Self {
        Self { id, result_setter }
    }
}

#[derive(Debug, Error)]
pub enum AddError {}
