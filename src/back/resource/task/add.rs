use crate::back::resource::task::{DynResultSetter, ResId};

pub type AddResultSetter<R> = DynResultSetter<R>;

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

    /// Returns task data reference.
    pub fn data(&self) -> &R::Data {
        &self.data
    }

    /// Takes data source and result setter from `self`.
    pub fn into_inner(self) -> (R::Data, AddResultSetter<R>) {
        (self.data, self.result_setter)
    }
}