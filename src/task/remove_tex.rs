use crate::back::TexId;
use crate::gfx::Context;
use crate::res::Remove;
use crate::task::{SyncTaskSender, Task};
use tokio::task::JoinHandle;

#[derive(Debug)]
struct RemoveTex {
    id: TexId,
}

impl RemoveTex {
    pub fn new(id: TexId) -> Self {
        Self { id }
    }
}

impl Task for RemoveTex {
    fn start(&mut self, ctx: &mut Context) -> JoinHandle<()> {
        let mut tex_storage = ctx.back.get_tex_storage();
        let id = self.id;
        log::trace!("Removing texture: {:?}", id);
        ctx.rt.spawn(async move {
            tex_storage.remove(id).await;
        })
    }
}

struct TexRemover(SyncTaskSender);

impl Remove for TexRemover {
    type Resource = TexId;

    fn remove(&mut self, res: Self::Resource) {
        let _ = self.0.send(Box::new(RemoveTex::new(res)));
    }
}

pub fn remover(task_tx: SyncTaskSender) -> impl Remove<Resource = TexId> {
    TexRemover(task_tx)
}
