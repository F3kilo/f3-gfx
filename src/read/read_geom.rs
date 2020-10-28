use crate::back::GeomData;
use crate::read::{ReadError, ReadResult};
use log::{trace, warn};
use std::ops::Deref;
use std::path::{Path, PathBuf};

pub async fn read(path: PathBuf) -> ReadResult<GeomData> {
    trace!("Start reading geometry file: {:?}", path);
    if let Some(ext) = path.extension() {
        let str = ext.to_string_lossy().into_owned();

        return match str.deref() {
            "fbx" => Ok(GeomData {
                vertices: vec![],
                indices: vec![],
            }), // TODO: read fbx geometry data
            ext => Err(extension_not_supported(&path)),
        };
    }
    Err(extension_not_specified(&path))
}

fn extension_not_supported(path: &Path) -> ReadError {
    warn!(
        "Try read geometry file with unsupported extension: {:?}",
        path
    );
    ReadError(format!("Extension is not supported: {:?}", path))
}

fn extension_not_specified(path: &Path) -> ReadError {
    warn!("Try read geometry file without extension: {:?}", path);
    ReadError(format!("Extension is not specified: {:?}", path))
}
