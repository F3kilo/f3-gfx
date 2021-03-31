use crate::back::resource::task::ResId;

#[derive(Debug)]
pub struct RemoveTask<R: ResId> {
    id: R,
}

impl<R: ResId> RemoveTask<R> {
    /// Creates new Remove task for graphics backend.
    pub fn new(id: R) -> Self {
        Self { id }
    }

    /// Takes resource idenyifier from `self`.
    pub fn into_inner(self) -> R {
        self.id
    }
}
