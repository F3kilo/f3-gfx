pub mod add;
pub mod list;
pub mod read;
pub mod remove;

use crate::back::resource::task::add::AddTask;
use crate::back::resource::task::list::ListTask;
use crate::back::resource::task::read::ReadTask;
use crate::back::resource::task::remove::RemoveTask;
use crate::back::{ResultSetter, BackendTask};
use std::fmt;

#[derive(Debug)]
pub enum ResourceTask<Res: ResourceId> {
    Add(AddTask<Res>),
    Remove(RemoveTask<Res>),
    Read(ReadTask<Res>),
    List(ListTask<Res>),
}

pub trait ResourceId: Send + Sync + fmt::Debug + Copy + Clone {
    type Data: Send + fmt::Debug;
    
    fn add(task: AddTask<Self>) -> BackendTask;
    fn remove(task: RemoveTask<Self>) -> BackendTask;
    fn read(task: ReadTask<Self>) -> BackendTask;
    fn list(task: ListTask<Self>) -> BackendTask;
}

pub type DynResultSetter<Res> = Box<dyn ResultSetter<Res>>;
