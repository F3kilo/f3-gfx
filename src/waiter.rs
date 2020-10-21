use once_cell::sync::OnceCell;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Getter<T> {
    cell: Arc<OnceCell<T>>,
}

impl<T: Debug + Send + Sync> Getter<T> {
    pub fn new() -> (Self, Setter<T>) {
        let cell = Arc::new(OnceCell::new());
        let setter_cell = cell.clone();
        (Self { cell }, Setter(setter_cell))
    }

    pub fn try_get(&self) -> TakeResult<&T> {
        log::trace!("Trying to take item");
        if let Some(got) = self.cell.get() {
            return Ok(got);
        }
        Err(TakeError::NotReady)
    }

    pub fn try_take(self) -> TakeResult<T> {
        log::trace!("Trying to take item");

        if self.cell.get().is_some() {
            let inner = Arc::try_unwrap(self.cell);
            return match inner {
                Ok(mut cell) => Ok(cell.take().unwrap()), // Can unwrap, because outer 'if'
                Err(arc) => Err(TakeError::CantTake(Self { cell: arc })),
            };
        }

        Err(TakeError::NotReady)
    }
}

pub type TakeResult<T> = Result<T, TakeError<Getter<T>>>;

#[derive(Debug)]
pub enum TakeError<G> {
    NotReady,
    CantTake(G),
}

impl<G: Debug> Error for TakeError<G> {}

impl<G> fmt::Display for TakeError<G> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TakeError::NotReady => write!(f, "Resource is not ready"),
            TakeError::CantTake(_) => write!(f, "Can't take value because another getter exist"),
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
