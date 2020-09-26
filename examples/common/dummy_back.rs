use crate::common::id_counter;
use async_trait::async_trait;
use f3_gfx::back::{
    Backend, ReadError, ReadResult, StoreResource, StoreTex, TexData, TexId, WriteResult,
};
use futures_util::core_reexport::time::Duration;

pub struct DummyBack {
    tex_storage: TexStorage,
}

impl Default for DummyBack {
    fn default() -> Self {
        Self {
            tex_storage: TexStorage::default(),
        }
    }
}

impl Backend for DummyBack {
    fn get_tex_storage(&mut self) -> Box<dyn StoreTex> {
        Box::new(self.tex_storage.clone())
    }
}

#[derive(Clone, Default)]
struct TexStorage {
    ids: Vec<TexId>,
}

impl StoreTex for TexStorage {}

#[async_trait]
impl StoreResource for TexStorage {
    type Id = TexId;
    type Data = TexData;

    async fn write(&mut self, _data: Self::Data) -> WriteResult<Self::Id> {
        tokio::time::delay_for(Duration::from_millis(200)).await;
        let new_id = id_counter::get_unique_id();
        self.ids.push(new_id);
        Ok(new_id)
    }

    async fn read(&self, id: Self::Id) -> ReadResult<Self::Data> {
        match self.get_pos(id) {
            Some(_) => Ok(TexData {}),
            None => Err(ReadError::NotFound),
        }
    }

    async fn remove(&mut self, id: Self::Id) {
        if let Some(index) = self.get_pos(id) {
            self.ids.swap_remove(index);
        };
        tokio::time::delay_for(Duration::from_millis(200)).await;
    }

    fn contains(&self, id: Self::Id) -> bool {
        self.get_pos(id).is_some()
    }

    fn list(&self) -> Vec<Self::Id> {
        self.ids.clone()
    }
}

impl TexStorage {
    fn get_pos(&self, id: TexId) -> Option<usize> {
        self.ids.iter().position(|i| *i == id)
    }
}
