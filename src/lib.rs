#![allow(dead_code)]
#![allow(unused_variables)]

use crate::back::WriteError;
use crate::read::ReadError;

pub mod async_tasker;
pub mod back;
pub mod deferred_task;
pub mod geom;
pub mod gfx;
pub mod read;
pub mod res;
pub mod scene;
pub mod task_counter;
pub mod tex;
pub mod waiter;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub type LoadResult<T> = Result<T, LoadError>;
pub type WriteResult<T> = Result<T, WriteError>;

#[derive(Debug, Clone)]
pub enum LoadError {
    ReadError(ReadError),
    WriteError(WriteError),
}

impl From<ReadError> for LoadError {
    fn from(e: ReadError) -> Self {
        Self::ReadError(e)
    }
}

impl From<WriteError> for LoadError {
    fn from(e: WriteError) -> Self {
        Self::WriteError(e)
    }
}
