pub mod add;
pub mod read;
pub mod remove;

use crate::back::resource::task::add::AddTask;
use crate::back::resource::task::read::ReadTask;
use crate::back::resource::task::remove::RemoveTask;
use crate::back::{BackendTask, ResultSetter};
use std::fmt;

#[derive(Debug)]
pub enum ResourceTask<R: ResId> {
    Add(AddTask<R>),
    Remove(RemoveTask<R>),
    Read(ReadTask<R>),
}

pub trait ResId: Send + Sync + fmt::Debug + Copy + Clone {
    type Data: Send + fmt::Debug;

    fn new_unique() -> Self;
    
    fn add(task: AddTask<Self>) -> BackendTask;
    fn remove(task: RemoveTask<Self>) -> BackendTask;
    fn read(task: ReadTask<Self>) -> BackendTask;
}

pub type DynResultSetter<R> = Box<dyn ResultSetter<R>>;
