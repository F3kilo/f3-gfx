use crate::back::resource::task::{ResId};
use crate::res::set_get::Setter;

#[derive(Debug)]
pub struct ReadTask<R: ResId> {
    id: R,
    result_setter: Setter<R::Data>,
}

impl<R: ResId> ReadTask<R> {
    /// Creates new Read task for graphics backend.
    pub fn new(id: R, result_setter: Setter<R::Data>) -> Self {
        Self { id, result_setter }
    }

    /// Takes data source and result setter from `self`.
    pub fn into_inner(self) -> (R, Setter<R::Data>) {
        (self.id, self.result_setter)
    }
}