use crate::common::id_counter;
use async_trait::async_trait;
use f3_gfx::back::{
    Backend, GeomData, GeomId, ReadError, ReadResult, StoreGeom, StoreResource, StoreTex, TexData,
    TexId, WriteResult,
};
use futures_util::core_reexport::fmt::Debug;
use futures_util::core_reexport::time::Duration;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct DummyBack {
    tex_storage: Storage<TexId>,
    geom_storage: Storage<GeomId>,
}

impl Backend for DummyBack {
    fn get_tex_storage(&mut self) -> Box<dyn StoreTex> {
        Box::new(self.tex_storage.clone())
    }

    fn get_geom_storage(&mut self) -> Box<dyn StoreGeom> {
        Box::new(self.geom_storage.clone())
    }
}

impl StoreTex for Storage<TexId> {}
impl ResId for TexId {
    type Data = TexData;

    fn get_data(&self) -> Self::Data {
        TexData {}
    }
}

impl StoreGeom for Storage<GeomId> {}
impl ResId for GeomId {
    type Data = GeomData;

    fn get_data(&self) -> Self::Data {
        GeomData {}
    }
}

#[derive(Clone)]
struct Storage<T> {
    ids: Arc<Mutex<Vec<T>>>,
}

impl<T> Default for Storage<T> {
    fn default() -> Self {
        Self {
            ids: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl<T: ResId> Storage<T> {
    fn get_pos(&self, id: T) -> Option<usize> {
        self.ids.lock().unwrap().iter().position(|i| *i == id)
    }
}

trait ResId: From<u64> + Debug + Eq + PartialEq {
    type Data: Send + Sync;

    fn get_data(&self) -> Self::Data;
}

#[async_trait]
impl<T: ResId + Send + Copy> StoreResource for Storage<T> {
    type Id = T;
    type Data = T::Data;

    async fn write(&mut self, _data: Self::Data) -> WriteResult<Self::Id> {
        tokio::time::delay_for(Duration::from_millis(200)).await;
        let new_id = id_counter::get_unique_id();
        log::trace!("Add {:?} to tex storage", new_id);
        self.ids.lock().unwrap().push(new_id);
        Ok(new_id)
    }

    async fn read(&self, id: Self::Id) -> ReadResult<Self::Data> {
        let d = id.get_data();
        match self.get_pos(id) {
            Some(_) => Ok(d),
            None => Err(ReadError::NotFound),
        }
    }

    async fn remove(&mut self, id: Self::Id) {
        if let Some(index) = self.get_pos(id) {
            log::trace!("Remove {:?} from tex storage", id);
            self.ids.lock().unwrap().swap_remove(index);
            tokio::time::delay_for(Duration::from_millis(200)).await;
            return;
        };
        log::trace!(
            "Try to remove {:?} from tex storage, but it wasn't found",
            id
        );
    }

    fn contains(&self, id: Self::Id) -> bool {
        self.get_pos(id).is_some()
    }

    fn list(&self) -> Vec<Self::Id> {
        self.ids.lock().unwrap().clone()
    }
}
