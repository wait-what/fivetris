use std::{collections::HashMap, io::{self, Read}};
use image::{ImageFormat, ImageError};
use nanoserde::{DeJson, DeJsonErr};
use crate::AssetStore;
use tar::Archive;
use crabz2::Crabz2Reader as DecompressorReader;


#[derive(Debug)]
pub enum DeserializePackError {
    AtlasCountError,
    AtlasSizeError,
    JsonError(DeJsonErr),
    ImageError(ImageError),
    IoError(io::Error),
}

impl AssetStore {
    pub fn deserialize<T: Read>(bytes: T) -> Result<Self, DeserializePackError> {
        let mut asset_store: AssetStore = AssetStore {
            atlases: Vec::new(),
            texture_positions: HashMap::new(),
            audio: (),
        };

        let buf = DecompressorReader::new(bytes);
        let mut archive = Archive::new(buf);

        archive.entries()
            .map_err(DeserializePackError::IoError)?
            .filter_map(|entry| entry.ok())
            .for_each(|mut entry| {
                let path = entry.path().unwrap();
                let path_str = path.to_str().unwrap();

                if path_str == "texture_positions.json" {
                    let mut buf = String::new();
                    entry.read_to_string(&mut buf).unwrap();

                    asset_store.texture_positions = DeJson::deserialize_json(&buf)
                        .map_err(DeserializePackError::JsonError)
                        .unwrap();
                } else if path_str.starts_with("atlases/") && path_str.ends_with(".webp") {
                    let mut buf = Vec::new();
                    entry.read_to_end(&mut buf).unwrap();

                    let atlas = image::load_from_memory_with_format(&buf, ImageFormat::WebP)
                        .map_err(DeserializePackError::ImageError)
                        .unwrap();

                    asset_store.atlases.push(atlas);
                }
            });

        for texture_position in asset_store.texture_positions.values() {
            if texture_position.atlas_idx >= asset_store.atlases.len() {
                return Err(DeserializePackError::AtlasCountError);
            }

            let atlas = &asset_store.atlases[texture_position.atlas_idx];
            if texture_position.x + texture_position.width > atlas.width() ||
               texture_position.y + texture_position.height > atlas.height() {
                return Err(DeserializePackError::AtlasSizeError);
            }
        }

        Ok(asset_store)
    }
}
