use crate::common::id_counter;
use async_trait::async_trait;
use f3_gfx::back::{
    Backend, GeomData, GeomId, Present, PresentInfo, ReadError, ReadResult, Render, RenderInfo,
    RenderResult, StoreGeom, StoreResource, StoreTex, TexData, TexId, WriteResult,
};
use f3_gfx::scene::Scene;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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

    fn get_renderer(&mut self) -> Box<dyn Render> {
        Box::new(Renderer::new(self.tex_storage.clone()))
    }

    fn get_presenter(&mut self) -> Box<dyn Present> {
        Box::new(Presenter {})
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
        GeomData {
            vertices: vec![],
            indices: vec![],
        }
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
        let ids_lock = self.ids.lock().unwrap();
        let d = id.get_data();
        match ids_lock.iter().position(|i| *i == id) {
            Some(_) => Ok(d),
            None => Err(ReadError::NotFound),
        }
    }

    async fn remove(&mut self, id: Self::Id) {
        {
            let mut ids_lock = self.ids.lock().unwrap();
            if let Some(index) = ids_lock.iter().position(|i| *i == id) {
                log::trace!("Remove {:?} from tex storage", id);
                ids_lock.swap_remove(index);
                return;
            };
        }

        log::trace!(
            "Try to remove {:?} from tex storage, but it wasn't found",
            id
        );
        tokio::time::delay_for(Duration::from_millis(200)).await;
    }

    fn contains(&self, id: Self::Id) -> bool {
        let ids_lock = self.ids.lock().unwrap();
        ids_lock.contains(&id)
    }

    fn list(&self) -> Vec<Self::Id> {
        self.ids.lock().unwrap().clone()
    }
}

struct Renderer {
    tex_storage: Storage<TexId>,
}

impl Renderer {
    pub fn new(tex_storage: Storage<TexId>) -> Self {
        Self { tex_storage }
    }
}

#[async_trait]
impl Render for Renderer {
    async fn render(&mut self, scene: &Scene, _render_info: RenderInfo) -> RenderResult {
        for item in scene.iter() {
            log::trace!("Rendering item: {:?}", item);
        }

        let d = TexData {};
        let tex = self.tex_storage.write(d).await.unwrap();
        Ok(tex)
    }
}

struct Presenter {}

#[async_trait]
impl Present for Presenter {
    async fn present(&mut self, scene: &Scene, _present_info: PresentInfo) {
        log::trace!("Presenting scene: {:?}", scene)
    }
}
