use futures_util::core_reexport::fmt::Debug;
use std::error::Error;
use std::fmt;
use std::sync::mpsc::{Receiver, RecvError, TryRecvError};

pub struct ReceiveOnce<Recv> {
    recv: Option<Recv>,
}

impl<Recv: Receive> ReceiveOnce<Recv> {
    pub fn new(rx: Recv) -> Self {
        Self { recv: Some(rx) }
    }

    pub fn try_take(&mut self) -> TakeResult<Recv::Item> {
        if let Some(r) = &mut self.recv {
            let received = r.try_take()?;
            log::trace!("Taking resource: {:?}", received);
            self.recv = None;
            return Ok(received);
        }
        Err(already_taken_error())
    }

    pub fn wait(&mut self) -> TakeResult<Recv::Item> {
        if let Some(r) = &mut self.recv {
            let received = r.wait()?;
            log::trace!("Taking resource after wait: {:?}", received);
            self.recv = None;
            return Ok(received);
        }
        Err(already_taken_error())
    }
}

fn already_taken_error() -> TakeError {
    log::warn!("Try to take already taken resource");
    TakeError::AlreadyTaken
}

pub trait Receive {
    type Item: Debug;

    fn try_take(&mut self) -> TakeResult<Self::Item>;
    fn wait(&mut self) -> TakeResult<Self::Item>;
}

impl<T: Debug> Receive for Receiver<T> {
    type Item = T;

    fn try_take(&mut self) -> TakeResult<Self::Item> {
        Ok(self.try_recv()?)
    }

    fn wait(&mut self) -> TakeResult<Self::Item> {
        Ok(self.recv()?)
    }
}

pub type TakeResult<T> = Result<T, TakeError>;

#[derive(Debug)]
pub enum TakeError {
    NotReady,
    NotAvailable,
    AlreadyTaken,
}

impl Error for TakeError {}

impl fmt::Display for TakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TakeError::NotReady => write!(f, "Resource is not ready"),
            TakeError::NotAvailable => write!(f, "Resource is not available"),
            TakeError::AlreadyTaken => write!(f, "Resource is already taken"),
        }
    }
}

impl From<TryRecvError> for TakeError {
    fn from(e: TryRecvError) -> Self {
        match e {
            TryRecvError::Empty => Self::NotReady,
            TryRecvError::Disconnected => Self::NotAvailable,
        }
    }
}

impl From<RecvError> for TakeError {
    fn from(e: RecvError) -> Self {
        Self::NotAvailable
    }
}
