use crate::back::TexId;
use crate::task::remove_tex::RemoveTex;
use crate::task::SyncTaskSender;
use futures_util::core_reexport::fmt::{Debug, Formatter};
use std::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub struct Tex {
    inner: Arc<UniqueTex>,
}

impl Debug for Tex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.inner.id.fmt(f)
    }
}

impl Tex {
    pub fn new(id: TexId, remover: Box<dyn Remove<Resource = TexId>>) -> Self {
        let inner = Arc::new(UniqueTex::new(id, remover));
        Self { inner }
    }
}

struct UniqueTex {
    id: TexId,
    remover: Box<dyn Remove<Resource = TexId>>,
}

impl UniqueTex {
    pub fn new(id: TexId, remover: Box<dyn Remove<Resource = TexId>>) -> Self {
        log::trace!("New unique tex #{:?} created", id);
        Self { id, remover }
    }
}

impl Drop for UniqueTex {
    fn drop(&mut self) {
        log::trace!("Removing tex #{:?}", self.id);
        self.remover.remove(self.id)
    }
}

pub trait Remove: Send + Sync {
    type Resource;

    fn remove(&mut self, res: Self::Resource);
}

impl Remove for SyncTaskSender {
    type Resource = TexId;

    fn remove(&mut self, res: Self::Resource) {
        let remove_task = RemoveTex::new(res);
        let _ = self.send(Box::new(remove_task));
    }
}
