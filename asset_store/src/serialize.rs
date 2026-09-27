use std::io::{self, Write};
use image::ImageError;
use nanoserde::SerJson;
use tar::{Builder as TarBuilder, Header as TarHeader};
use crabz2::{Crabz2Writer as CompressorWriter, Level as CompressionLevel};
use crate::AssetStore;

#[derive(Debug)]
pub enum SerializePackError {
    ImageError(ImageError),
    IoError(io::Error),
}

impl AssetStore {
    fn encode_atlases(&self) -> Result<Vec<Vec<u8>>, SerializePackError> {
        let mut atlases: Vec<Vec<u8>> = Vec::new();

        for atlas in &self.atlases {
            let mut buf = io::Cursor::new(Vec::new());
            atlas.write_to(&mut buf, image::ImageFormat::WebP).map_err(SerializePackError::ImageError)?;
            atlases.push(buf.into_inner());
        }

        Ok(atlases)
    }

    fn add_file<T: Write>(pack: &mut TarBuilder<T>, path: &str, data: &[u8]) -> Result<(), SerializePackError> {
        let mut header = TarHeader::new_gnu();
        header.set_size(data.len() as u64);
        header.set_mode(0o644);

        pack.append_data(&mut header, path, data).map_err(SerializePackError::IoError)?;

        Ok(())
    }

    pub fn serialize<T: Write>(&self, buf: &mut T) -> Result<(), SerializePackError> {
        let buf = CompressorWriter::new(buf, CompressionLevel::new(3).unwrap());
        let mut pack = TarBuilder::new(buf);

        let atlases = self.encode_atlases()?;
        for (i, atlas) in atlases.iter().enumerate() {
            let path = format!("atlases/{}.webp", i);
            Self::add_file(&mut pack, &path, atlas)?;
        }

        let texture_positions_json = self.texture_positions.serialize_json();
        Self::add_file(&mut pack, "texture_positions.json", texture_positions_json.as_bytes())?;

        pack.finish().map_err(SerializePackError::IoError)?;

        pack
            .into_inner()
            .map_err(SerializePackError::IoError)?
            .finish()
            .map_err(SerializePackError::IoError)?;

        Ok(())
    }
}
