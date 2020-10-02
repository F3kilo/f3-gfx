use async_trait::async_trait;

pub trait Backend: Send {
    fn get_tex_storage(&mut self) -> Box<dyn StoreTex>;
    fn get_geom_storage(&mut self) -> Box<dyn StoreGeom>;
}

#[async_trait]
pub trait StoreResource: Send {
    type Id;
    type Data;

    async fn write(&mut self, data: Self::Data) -> WriteResult<Self::Id>;
    async fn read(&self, id: Self::Id) -> ReadResult<Self::Data>;
    async fn remove(&mut self, id: Self::Id);

    fn contains(&self, id: Self::Id) -> bool;
    fn list(&self) -> Vec<Self::Id>;
}

pub trait StoreTex: StoreResource<Id = TexId, Data = TexData> {}
pub trait StoreGeom: StoreResource<Id = GeomId, Data = GeomData> {}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct TexId(u64);

impl From<u64> for TexId {
    fn from(i: u64) -> Self {
        Self(i)
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct GeomId(u64);

impl From<u64> for GeomId {
    fn from(i: u64) -> Self {
        Self(i)
    }
}

pub type ReadResult<T> = Result<T, ReadError>;

pub enum ReadError {
    NotFound,
    CantRead(&'static str),
}

pub type WriteResult<T> = Result<T, WriteError>;

#[derive(Debug)]
pub struct WriteError;

pub struct TexData {}
pub struct GeomData {}
