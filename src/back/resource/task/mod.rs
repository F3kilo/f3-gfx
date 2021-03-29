pub mod add;
pub mod list;
pub mod read;
pub mod remove;

use crate::back::resource::task::add::AddTask;
use crate::back::resource::task::list::ListTask;
use crate::back::resource::task::read::ReadTask;
use crate::back::resource::task::remove::RemoveTask;
use crate::back::ResultSender;
use std::fmt;

#[derive(Debug)]
pub enum ResourceTask<Res: ResourceId> {
    Add(AddTask<Res>),
    Remove(RemoveTask<Res>),
    Read(ReadTask<Res>),
    List(ListTask<Res>),
}

pub trait ResourceId: Send + fmt::Debug {
    type Data: Send + fmt::Debug;
}

pub type DynResultSender<Res> = Box<dyn ResultSender<Res>>;
