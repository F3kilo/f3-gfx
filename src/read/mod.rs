use std::error::Error;
use std::{fmt, io};

pub mod read_geom;
pub mod read_tex;

pub type ReadResult<T> = Result<T, ReadError>;

#[derive(Debug, Clone)]
pub struct ReadError(String);

impl Error for ReadError {}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Read failed: {}", self.0)
    }
}

impl From<io::Error> for ReadError {
    fn from(e: io::Error) -> Self {
        ktx2_reader::error::ReadError::IoError(e).into()
    }
}
