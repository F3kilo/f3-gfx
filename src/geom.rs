use crate::back::{GeomId, StoreGeom};
use crate::deferred_task::{DeferredTask, DeferredTaskPusher};
use crate::gfx::Geom;
use crate::read::read_geom;
use crate::res::Remove;
use crate::LoadResult;
use std::path::PathBuf;

pub struct GeomRemover(DeferredTaskPusher);

impl GeomRemover {
    pub fn new(pusher: DeferredTaskPusher) -> Self {
        Self(pusher)
    }
}

impl Remove for GeomRemover {
    type Resource = GeomId;

    fn remove(&mut self, res: Self::Resource) {
        self.0.push(DeferredTask::RemoveGeom(res))
    }
}

pub async fn load_async(
    path: PathBuf,
    mut geom_storage: Box<dyn StoreGeom>,
    remover: Box<dyn Remove<Resource = GeomId>>,
) -> LoadResult<Geom> {
    let data = read_geom::read(path).await?;
    let id = geom_storage.write(data).await?;
    Ok(Geom::new(id, remover))
}

pub async fn remove_async(id: GeomId, mut geom_storage: Box<dyn StoreGeom>) {
    geom_storage.remove(id).await
}
