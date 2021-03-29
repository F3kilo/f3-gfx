use std::fmt;
use thiserror::Error;

#[async_trait::async_trait]
pub trait DataSource<Data: fmt::Debug>: Send + 'static + fmt::Debug {
    async fn take_data(&mut self) -> TakeDataResult<Data>;
}

pub type TakeDataResult<T> = Result<T, TakeDataError>;

#[derive(Debug, Error)]
pub enum TakeDataError {
    #[error("Data is not available from source: {0}")]
    NotAvailable(String),
}
