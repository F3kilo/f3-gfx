use crate::back::resource::task::{ResId};
use crate::res::set_get::Setter;
use crate::res::GfxResource;

#[derive(Debug)]
pub struct AddTask<R: ResId> {
    data: R::Data,
    result_setter: Setter<GfxResource<R>>,
}

impl<R: ResId> AddTask<R> {
    /// Creates new Add task for backend resource.
    pub fn new(data: R::Data, result_setter: Setter<GfxResource<R>>) -> Self {
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
    pub fn into_inner(self) -> (R::Data, Setter<GfxResource<R>>) {
        (self.data, self.result_setter)
    }
}