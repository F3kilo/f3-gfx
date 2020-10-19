#![allow(dead_code)]
#![allow(unused_variables)]

use crate::back::WriteError;
use crate::read_tex::ReadError;

pub mod back;
pub mod gfx;
mod read_tex;
pub mod res;
pub mod scene;
pub mod task_counter;
pub mod deferred_task;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub type LoadResult<T> = Result<T, LoadError>;
pub type WriteResult<T> = Result<T, WriteError>;

#[derive(Debug)]
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
