use crate::back::TexId;
use crate::gfx::Context;
use crate::task::Task;
use tokio::task::JoinHandle;

pub struct RemoveTex {
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
        ctx.rt.spawn(async move {
            let result = tex_storage.remove(id).await;
        })
    }
}
