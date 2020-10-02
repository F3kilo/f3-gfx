use crate::back::TexId;
use crate::task::remove_tex::RemoveTex;
use crate::task::{SyncTaskSender, Task};
use crate::tex::Tex;
use crate::LoadResult;
use log::warn;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

pub struct TexWaiter {
    rx: Option<TexReceiver>,
}

impl TexWaiter {
    pub fn new(recv: Receiver<LoadResult<Tex>>) -> Self {
        Self {
            rx: Some(TexReceiver::new(recv)),
        }
    }

    pub fn try_take(&mut self) -> TakeResult<LoadResult<Tex>> {
        match &mut self.rx {
            Some(rx) => {
                let result = rx.try_recv();
                if result.is_ok() {
                    self.rx = None;
                }
                result
            }
            None => Err(Self::already_taken_error()),
        }
    }

    pub fn wait_ready(&mut self) -> TakeResult<LoadResult<Tex>> {
        match &mut self.rx {
            Some(rx) => {
                let result = rx.wait_ready();
                if result.is_ok() {
                    self.rx = None;
                }
                result
            }
            None => Err(Self::already_taken_error()),
        }
    }

    fn already_taken_error() -> TakeError {
        warn!("Try to take texture, which is already taken.");
        TakeError::AlreadyTaken
    }
}

struct TexReceiver {
    recv: Receiver<LoadResult<Tex>>,
}

impl TexReceiver {
    pub fn new(recv: Receiver<LoadResult<Tex>>) -> Self {
        Self { recv }
    }

    pub fn try_recv(&self) -> TakeResult<LoadResult<Tex>> {
        let received = self.recv.try_recv();
        match received {
            Ok(tex_result) => Ok(tex_result),
            Err(e) => match e {
                TryRecvError::Empty => Err(TakeError::NotReady),
                TryRecvError::Disconnected => Err(Self::not_available_error()),
            },
        }
    }

    pub fn wait_ready(&self) -> TakeResult<LoadResult<Tex>> {
        let received = self.recv.recv();
        match received {
            Ok(tex_result) => Ok(tex_result),
            Err(e) => Err(Self::not_available_error()),
        }
    }

    fn not_available_error() -> TakeError {
        warn!("Try receive loading texture id, but receiver is disconnected.");
        TakeError::NotAvailable
    }
}

impl From<TexReceiver> for Receiver<LoadResult<Tex>> {
    fn from(r: TexReceiver) -> Self {
        r.recv
    }
}

#[derive(Clone)]
pub struct TexRemover {
    tx: SyncTaskSender,
}

impl TexRemover {
    pub fn new(tx: Sender<Box<dyn Task>>) -> Self {
        Self {
            tx: SyncTaskSender::new(tx),
        }
    }

    pub fn remove(&self, id: TexId) {
        let _ = self.tx.send(Box::new(RemoveTex::new(id)));
    }
}

pub type TakeResult<T> = Result<T, TakeError>;

#[derive(Debug)]
pub enum TakeError {
    NotReady,
    NotAvailable,
    AlreadyTaken,
}
