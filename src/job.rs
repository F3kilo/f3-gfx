use crate::async_tasker::AsyncTasker;
use crate::back::Backend;
use std::{mem, fmt};

pub trait Job: Send + fmt::Display {
    fn start(&mut self, tasker: &mut AsyncTasker, back: &mut Box<dyn Backend>);
}

pub struct OnceData<Data> {
    data: Option<Data>,
}

impl<Data> OnceData<Data> {
    pub fn new(data: Data) -> Self {
        Self { data: Some(data) }
    }

    pub fn take(&mut self) -> Data {
        mem::replace(&mut self.data, None).expect("Try to take data twice from OnceData")
    }
}

impl<T> From<T> for OnceData<T> {
    fn from(d: T) -> Self {
        OnceData::new(d)
    }
}
