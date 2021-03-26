#![allow(dead_code)]
#![allow(unused_variables)]

use crate::back::WriteError;
use crate::data_src::TakeDataError;
use thiserror::Error;

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
pub mod render;
pub mod present;


pub type LoadResult<T> = Result<T, LoadError>;
pub type WriteResult<T> = Result<T, WriteError>;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("Can't take data from provided source: {0}")]
    TakeError(#[from]TakeDataError),
    #[error("Can't write data to backend: {0}")]
    WriteError(#[from]WriteError),
}
