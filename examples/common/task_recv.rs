use std::sync::mpsc;
use f3_gfx::GfxTask;
use f3_gfx::task_recv::{ReceiveTask, ReceiveTaskError};
use std::sync::mpsc::TryRecvError;


#[derive(Debug)]
pub struct TaskReceiver(mpsc::Receiver<GfxTask>);

impl TaskReceiver {
    pub fn new(rx: mpsc::Receiver<GfxTask>) -> Self {
        Self(rx)
    }
}

impl ReceiveTask for TaskReceiver {
    fn pop(&mut self) -> Result<Option<GfxTask>, ReceiveTaskError> {
        match self.0.try_recv() {
            Ok(t) => Ok(Some(t)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(ReceiveTaskError::LostAllInputChannels),
        }
    }
}