use once_cell::sync::OnceCell;
use std::fmt::Debug;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug)]
pub struct Getter<T> {
    cell: Arc<OnceCell<T>>,
}

impl<T: Debug + Send + Sync> Getter<T> {
    pub fn new() -> (Self, Setter<T>) {
        let cell = Arc::new(OnceCell::new());
        let setter_cell = cell.clone();
        (Self { cell }, Setter(setter_cell))
    }

    pub fn try_get(&self) -> GetResult<&T> {
        log::trace!("Trying to get Getter item.");
        if let Some(got) = self.cell.get() {
            return Ok(got);
        }
        Err(GetError::NotReady)
    }

    pub fn try_take(self) -> Result<T, Self> {
        log::trace!("Trying to take Getter item.");

        match Arc::try_unwrap(self.cell) {
            Ok(mut inner) => Ok(inner
                .take()
                .expect("Getter is not ready, but Setter was destroyed.")),
            Err(not_unwrapped) => Err(Self {
                cell: not_unwrapped,
            }),
        }
    }
}

pub type GetResult<T> = Result<T, GetError>;

#[derive(Debug, Error)]
pub enum GetError {
    #[error("Getter value is not ready")]
    NotReady,
}

#[derive(Debug)]
pub struct Setter<T>(Arc<OnceCell<T>>);

impl<T: Send + Sync> Setter<T> {
    pub fn set(&mut self, val: T) {
        if let Err(e) = self.0.set(val) {
            log::warn!("Try to set value to already full Setter.")
        }
    }
}
