use crate::back::resource::task::ResId;

#[derive(Debug)]
pub struct RemoveTask<R: ResId> {
    id: R,
}

impl<R: ResId> RemoveTask<R> {
    pub fn new(id: R) -> Self {
        Self { id }
    }
}
