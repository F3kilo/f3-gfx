use crate::back::TexData;
use log::{trace, warn};
use std::error::Error;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{fmt, io};

pub async fn read(path: PathBuf) -> ReadResult<TexData> {
    trace!("Start reading texture file: {:?}", path);
    if let Some(ext) = path.extension() {
        let str = ext.to_string_lossy().into_owned();

        return match str.deref() {
            "ktx2" => read_ktx2(path).await.map(|t| TexData {}),
            ext => Err(extension_not_supported(&path)),
        };
    }
    Err(extension_not_specified(&path))
}

async fn read_ktx2(path: PathBuf) -> ReadResult<TexData> {
    trace!("Start reading .ktx2 file");
    let file = tokio::fs::File::open(&path).await?;
    let mut reader = ktx2_reader::Reader::new(file).await?;
    let _head = reader.header();
    let _regions = reader.regions_description();
    let _data = reader.read_data().await?;
    Ok(TexData {})
}

fn extension_not_supported(path: &Path) -> ReadError {
    warn!(
        "Try read texture file with unsupported extension: {:?}",
        path
    );
    ReadError(format!("Extension is not supported: {:?}", path))
}

fn extension_not_specified(path: &Path) -> ReadError {
    warn!("Try read texture file without extension: {:?}", path);
    ReadError(format!("Extension is not specified: {:?}", path))
}

pub type ReadResult<T> = Result<T, ReadError>;

#[derive(Debug, Clone)]
pub struct ReadError(String);

impl Error for ReadError {}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Read failed: {}", self.0)
    }
}

impl From<ktx2_reader::error::ReadError> for ReadError {
    fn from(e: ktx2_reader::error::ReadError) -> Self {
        Self(format!("Can't read .ktx2 texture: {}", e))
    }
}

impl From<io::Error> for ReadError {
    fn from(e: io::Error) -> Self {
        ktx2_reader::error::ReadError::IoError(e).into()
    }
}
