use std::error::Error;
use std::fmt;

#[async_trait::async_trait]
pub trait DataSource: Send + 'static {
    type Data;

    async fn take_data(self) -> TakeDataResult<Self::Data>;
}

#[async_trait::async_trait]
impl<T: Send + 'static> DataSource for T {
    type Data = T;

    async fn take_data(self) -> TakeDataResult<Self::Data> {
        Ok(self)
    }
}

pub type TakeDataResult<T> = Result<T, TakeDataError>;

#[derive(Debug)]
pub enum TakeDataError {
    NotAvailable,
}

impl Error for TakeDataError {}

impl fmt::Display for TakeDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reason = match self {
            TakeDataError::NotAvailable => "data is not available",
        };
        write!(f, "Can't take data: {}", reason)
    }
}

