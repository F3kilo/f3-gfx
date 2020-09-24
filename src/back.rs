use async_trait::async_trait;

pub trait Backend: Send {
    fn get_tex_storage(&mut self) -> Box<dyn StoreTex>;
}

#[async_trait]
pub trait StoreResource: Send {
    type Id;
    type Data;

    async fn write(&mut self, data: Self::Data) -> WriteResult<Self::Id>;
    async fn read(&self, data: Self::Data) -> ReadResult<Self::Data>;
    async fn remove(&mut self, id: Self::Id) -> RemoveResult;

    fn list(&self) -> Vec<Self::Id>;
}

pub trait StoreTex: StoreResource<Id = TexId, Data = TexData> {}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct TexId(u64);

impl From<u64> for TexId {
    fn from(i: u64) -> Self {
        Self(i)
    }
}

pub type ReadResult<T> = Result<T, ReadError>;

pub struct ReadError;

pub type WriteResult<T> = Result<T, WriteError>;

pub struct WriteError;

pub enum RemoveResult {
    Success,
    Fail,
}

pub struct TexData {}
