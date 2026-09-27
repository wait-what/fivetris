use std::io;
use image::ImageError;
use nanoserde::DeJsonErr;
use crate::AssetStore;

#[derive(Debug)]
pub enum DeserializePackError {
    JsonError(DeJsonErr),
    ImageError(ImageError),
    TarError(io::Error),
}

impl AssetStore {
    pub fn deserialize(bytes: &[u8]) -> Result<Self, DeserializePackError> {
        todo!();
    }
}
