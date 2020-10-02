use crate::back::TexId;
use crate::task::remove_tex::RemoveTex;
use crate::task::SyncTaskSender;
use futures_util::core_reexport::fmt::{Debug, Formatter};
use std::fmt;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Resource<Res: Debug + Copy> {
    inner: Arc<UniqueResource<Res>>,
}

impl<Res: Debug + Copy> Debug for UniqueResource<Res> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.res.fmt(f)
    }
}

impl<Res: Debug + Copy> Resource<Res> {
    pub fn new(res: Res, remover: Box<dyn Remove<Resource = Res>>) -> Self {
        let inner = Arc::new(UniqueResource::new(res, remover));
        Self { inner }
    }
}

struct UniqueResource<Res: Debug + Copy> {
    res: Res,
    remover: Box<dyn Remove<Resource = Res>>,
}

impl<Res: Debug + Copy> UniqueResource<Res> {
    pub fn new(res: Res, remover: Box<dyn Remove<Resource = Res>>) -> Self {
        log::trace!("New unique resource #{:?} created", res);
        Self { res, remover }
    }
}

impl<Res: Debug + Copy> Drop for UniqueResource<Res> {
    fn drop(&mut self) {
        log::trace!("Removing resource #{:?}", self.res);
        self.remover.remove(self.res)
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
