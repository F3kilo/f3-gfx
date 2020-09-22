#![allow(dead_code)]
#![allow(unused_variables)]

use crate::back::{Backend, WriteError};
use crate::link::Link;
use crate::read_tex::ReadError;
use gfx::Tasker;
use std::sync::mpsc;
use std::thread;

pub mod back;
mod gfx;
pub mod link;
mod read_tex;
mod task;
pub mod tex;
pub mod tex_waiter;

pub fn run(back: Box<dyn Backend>) -> Link {
    let (task_tx, task_rx) = mpsc::channel();
    let link = Link::new(task_tx.clone());

    thread::spawn(move || {
        let mut tasker = Tasker::new(back, task_tx, task_rx);
        while tasker.start_next_task() {}
    });

    link
}

pub type LoadResult<T> = Result<T, LoadError>;
pub type WriteResult<T> = Result<T, WriteError>;

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
