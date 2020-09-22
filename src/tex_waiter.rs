use crate::back::TexId;
use crate::task::Task;
use crate::tex::Tex;
use crate::LoadResult;
use log::warn;
use std::mem;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

pub struct TexWaiter {
    rx: Option<TexReceiver>,
    remover: TexRemover,
}

impl TexWaiter {
    pub fn new(recv: Receiver<LoadResult<TexId>>, unloader: TexRemover) -> Self {
        Self {
            rx: Some(TexReceiver::new(recv)),
            remover: unloader,
        }
    }

    pub fn try_take(&mut self) -> TakeResult<LoadResult<Tex>> {
        match &mut self.rx {
            Some(rx) => {
                let result = rx.try_recv();
                if result.is_ok() {
                    self.rx = None;
                }
                self.id_to_tex(result)
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
                self.id_to_tex(result)
            }
            None => Err(Self::already_taken_error()),
        }
    }

    fn already_taken_error() -> TakeError {
        warn!("Try to take texture, which is already taken.");
        TakeError::AlreadyTaken
    }

    fn id_to_tex(&self, result: TakeResult<LoadResult<TexId>>) -> TakeResult<LoadResult<Tex>> {
        result.map(|load_result| load_result.map(|id| Tex::new(id, self.remover.clone())))
    }
}

impl Drop for TexWaiter {
    fn drop(&mut self) {
        let taken = self.try_take();
        match taken {
            Ok(tex_result) => {}
            Err(_) => {
                let recv = mem::replace(&mut self.rx, None);
                if let Some(recv) = recv {
                    self.remover.remove_later(recv.into())
                }
            }
        }
    }
}

struct TexReceiver {
    recv: Receiver<LoadResult<TexId>>,
}

impl TexReceiver {
    pub fn new(recv: Receiver<LoadResult<TexId>>) -> Self {
        Self { recv }
    }

    pub fn try_recv(&self) -> TakeResult<LoadResult<TexId>> {
        let received = self.recv.try_recv();
        match received {
            Ok(tex_result) => Ok(tex_result),
            Err(e) => match e {
                TryRecvError::Empty => Err(TakeError::NotReady),
                TryRecvError::Disconnected => Err(Self::not_available_error()),
            },
        }
    }

    pub fn wait_ready(&self) -> TakeResult<LoadResult<TexId>> {
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

impl From<TexReceiver> for Receiver<LoadResult<TexId>> {
    fn from(r: TexReceiver) -> Self {
        r.recv
    }
}

#[derive(Clone)]
pub struct TexRemover {
    tx: Sender<Task>,
}

impl TexRemover {
    pub fn new(tx: Sender<Task>) -> Self {
        Self { tx }
    }

    pub fn remove(&self, id: TexId) {
        let _ = self.tx.send(Task::RemoveTex(id));
    }

    pub fn remove_later(&self, recv: Receiver<LoadResult<TexId>>) {
        let _ = self.tx.send(Task::RemoveTexLater(recv));
    }
}

pub type TakeResult<T> = Result<T, TakeError>;

pub enum TakeError {
    NotReady,
    NotAvailable,
    AlreadyTaken,
}
