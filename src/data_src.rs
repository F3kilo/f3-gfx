use std::error::Error;
use std::fmt;
use tokio::task::{JoinError, JoinHandle};

#[async_trait::async_trait]
pub trait DataSource: Send + 'static {
    type Data;

    async fn take_data(self) -> TakeResult<Self::Data>;
}

#[async_trait::async_trait]
impl<T: Send + 'static> DataSource for T {
    type Data = T;

    async fn take_data(self) -> TakeResult<Self::Data> {
        Ok(self)
    }
}

pub type TakeResult<T> = Result<T, TakeError>;

#[derive(Debug)]
pub enum TakeError {
    NotAvailable,
    NotComplete,
}

impl Error for TakeError {}

impl fmt::Display for TakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reason = match self {
            TakeError::NotAvailable => "data is not available",
            TakeError::NotComplete => "loading not complete due to system error",
        };
        write!(f, "Can't take data: {}", reason)
    }
}

impl From<JoinError> for TakeError {
    fn from(e: JoinError) -> Self {
        log::error!("Some data loading async task was interrupted: {}", e);
        Self::NotComplete
    }
}

#[async_trait::async_trait]
pub trait JoinData {
    type Data;

    async fn join_data(self) -> TakeResult<Self::Data>;
}

#[async_trait::async_trait]
impl<U: Send + 'static> JoinData for JoinHandle<TakeResult<U>> {
    type Data = U;

    async fn join_data(self) -> TakeResult<Self::Data> {
        let join_result = self.await?;
        join_result
    }
}
