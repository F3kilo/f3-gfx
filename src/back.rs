pub trait Backend: Send {
    fn unload_tex(&mut self, id: TexId);
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct TexId(u64);

impl From<u64> for TexId {
    fn from(i: u64) -> Self {
        Self(i)
    }
}


pub type LoadResult<T> = Result<T, LoadError>;

pub struct LoadError;
