use crate::async_tasker::AsyncTasker;
use crate::back::Backend;
use std::fmt::Debug;
use std::mem;

pub trait Job: Send + Debug {
    fn start(&mut self, tasker: &mut AsyncTasker, back: &mut Box<dyn Backend>);
}

#[derive(Debug)]
pub struct OnceData<Data: Debug> {
    data: Option<Data>,
}

impl<Data: Debug> OnceData<Data> {
    pub fn new(data: Data) -> Self {
        Self { data: Some(data) }
    }

    pub fn take(&mut self) -> Data {
        mem::replace(&mut self.data, None).expect("Try to take data twice from OnceData")
    }
}

impl<T: Debug> From<T> for OnceData<T> {
    fn from(d: T) -> Self {
        OnceData::new(d)
    }
}
