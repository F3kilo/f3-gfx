use crate::back::{GeomId, StoreGeom, GeomData};
use crate::deferred_task::{DeferredTask, TaskPusher};
use crate::gfx::Geom;
use crate::read::read_geom;
use crate::res::Remove;
use crate::LoadResult;
use std::path::PathBuf;

pub struct GeomRemover(TaskPusher);

impl GeomRemover {
    pub fn new(pusher: TaskPusher) -> Self {
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
    geom_storage: Box<dyn StoreGeom>,
    remover: Box<dyn Remove<Resource = GeomId>>,
) -> LoadResult<Geom> {
    let data = read_geom::read(path).await?;
    load_from_data_async(data, geom_storage, remover).await
}

pub async fn load_from_data_async(
    data: GeomData,
    mut geom_storage: Box<dyn StoreGeom>,
    remover: Box<dyn Remove<Resource = GeomId>>,
) -> LoadResult<Geom> {
    let id = geom_storage.write(data).await?;
    Ok(Geom::new(id, remover))
}

pub async fn remove_async(id: GeomId, mut geom_storage: Box<dyn StoreGeom>) {
    geom_storage.remove(id).await
}
