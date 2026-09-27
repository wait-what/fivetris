use std::collections::HashMap;
use image::DynamicImage;
use nanoserde::{SerJson, DeJson};

mod deserialize;
pub use deserialize::DeserializePackError;

mod serialize;
pub use serialize::SerializePackError;

mod build_pack;
pub use build_pack::BuildPackError;

#[derive(SerJson, DeJson, Debug, Clone)]
pub struct Texture {
    pub atlas_idx: usize,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub struct AssetStore {
    pub atlases: Vec<DynamicImage>,
    pub texture_positions: HashMap<String, Texture>,
    pub audio: (),
}
