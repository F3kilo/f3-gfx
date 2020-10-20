use crate::back::{StoreTex, TexId};
use crate::deferred_task::{DeferredTask, TaskPusher};
use crate::gfx::Tex;
use crate::read::read_tex;
use crate::res::Remove;
use crate::LoadResult;
use std::path::PathBuf;

pub struct TexRemover(TaskPusher);

impl TexRemover {
    pub fn new(pusher: TaskPusher) -> Self {
        Self(pusher)
    }
}

impl Remove for TexRemover {
    type Resource = TexId;

    fn remove(&mut self, res: Self::Resource) {
        self.0.push(DeferredTask::RemoveTex(res))
    }
}

pub async fn load_async(
    path: PathBuf,
    mut tex_storage: Box<dyn StoreTex>,
    remover: Box<dyn Remove<Resource = TexId>>,
) -> LoadResult<Tex> {
    let data = read_tex::read(path).await?;
    let id = tex_storage.write(data).await?;
    Ok(Tex::new(id, remover))
}

pub async fn remove_async(id: TexId, mut tex_storage: Box<dyn StoreTex>) {
    tex_storage.remove(id).await
}
