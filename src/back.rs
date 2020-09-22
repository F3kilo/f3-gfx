use crate::LoadResult;
use std::sync::mpsc::Sender;

pub trait Backend: Send {
    fn write_tex(&mut self, data: TexData, result_sender: Sender<LoadResult<TexId>>);
    fn remove_tex(&mut self, id: TexId);
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct TexId(u64);

impl From<u64> for TexId {
    fn from(i: u64) -> Self {
        Self(i)
    }
}

pub type WriteResult<T> = Result<T, WriteError>;

pub struct WriteError;

pub struct TexData {}
