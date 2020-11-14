#![allow(dead_code)]
#![allow(unused_variables)]

use crate::back::WriteError;
use crate::data_src::TakeError;

pub mod async_tasker;
pub mod back;
pub mod job_stor;
pub mod geom;
pub mod gfx;
pub mod res;
pub mod scene;
pub mod task_counter;
pub mod tex;
pub mod waiter;
pub mod job;
pub mod data_src;

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
    TakeError(TakeError),
    WriteError(WriteError),
}

impl From<TakeError> for LoadError {
    fn from(e: TakeError) -> Self {
        Self::TakeError(e)
    }
}

impl From<WriteError> for LoadError {
    fn from(e: WriteError) -> Self {
        Self::WriteError(e)
    }
}

