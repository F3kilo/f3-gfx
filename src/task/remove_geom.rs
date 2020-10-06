use crate::back::GeomId;
use crate::gfx::Context;
use crate::res::Remove;
use crate::task::{SyncTaskSender, Task};
use tokio::task::JoinHandle;

#[derive(Debug)]
struct RemoveGeom {
    id: GeomId,
}

impl RemoveGeom {
    pub fn new(id: GeomId) -> Self {
        Self { id }
    }
}

impl Task for RemoveGeom {
    fn start(&mut self, ctx: &mut Context) -> JoinHandle<()> {
        let mut geom_storage = ctx.back.get_geom_storage();
        let id = self.id;
        log::trace!("Removing geometry: {:?}", id);
        ctx.rt.spawn(async move {
            geom_storage.remove(id).await;
        })
    }
}

struct GeomRemover(SyncTaskSender);

impl Remove for GeomRemover {
    type Resource = GeomId;

    fn remove(&mut self, res: Self::Resource) {
        let _ = self.0.send(Box::new(RemoveGeom::new(res)));
    }
}

pub fn remover(task_tx: SyncTaskSender) -> impl Remove<Resource = GeomId> {
    GeomRemover(task_tx)
}
