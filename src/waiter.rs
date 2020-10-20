use once_cell::sync::OnceCell;
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;
use std::{fmt, thread};
use tokio::time::Duration;

#[derive(Clone, Debug)]
pub struct Wait<T> {
    recv: Arc<OnceCell<T>>,
}

impl<T: Debug + Clone + Send + Sync> Wait<T> {
    pub fn new() -> (Self, Setter<T>) {
        let cell = Arc::new(OnceCell::new());
        (Self { recv: cell.clone() }, Setter(cell))
    }

    pub fn try_take(&self) -> TakeResult<T> {
        log::trace!("Trying to take item");
        if let Some(got) = self.recv.get() {
            return Ok(got.clone());
        }
        Err(TakeError::NotReady)
    }

    pub fn wait(&self) -> T {
        log::trace!("Waiting for item");
        loop {
            thread::sleep(Duration::from_millis(5));
            if let Some(got) = self.recv.get() {
                return got.clone();
            }
        }
    }

    // todo: wait with timeout and delay
}

fn already_taken_error() -> TakeError {
    log::warn!("Try to take already taken resource");
    TakeError::AlreadyTaken
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

#[derive(Debug)]
pub struct Setter<T>(Arc<OnceCell<T>>);

impl<T: Send + Sync> Setter<T> {
    pub fn set(&mut self, val: T) {
        if let Err(e) = self.0.set(val) {
            log::warn!("Try to set value to already full Setter")
        }
    }
}
