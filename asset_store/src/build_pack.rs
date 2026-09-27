use std::{collections::{HashMap, BTreeMap}, fs, io};
use image::{GenericImage, ImageError};
use crate::{Texture, AssetStore};
use rectangle_pack::{
    RectToInsert,
    GroupedRectsToPlace,
    TargetBin,
    pack_rects,
};

const MIN_TEXTURE_SIZE: u32 = 16;
const MAX_TEXTURE_SIZE: u32 = 2048;
const MARGIN: u32 = 1;

/// Start at a 16x16 texture at n=1. Double the size (n=2, n=3, ...) until we reach the maximum texture size.
/// If maximum texture size is reached, add another 16x16 texture and continue doubling it.
fn create_texture_sizes(mut n: usize, min_texture_size: u32, max_texture_size: u32) -> Vec<(u32, u32)> {
    let mut sizes = vec![min_texture_size];

    while n > 0 {
        let last = sizes.len() - 1;
        let size = sizes[last];

        if size < max_texture_size {
            sizes[last] = (size * 2).min(max_texture_size);
        } else {
            sizes.push(min_texture_size);
        }

        n -= 1;
    }

    sizes.into_iter().map(|size| (size, size)).collect()
}

#[derive(Debug)]
pub enum BuildPackError {
    ImageError(ImageError),
    IoError(io::Error),
}

impl AssetStore {
    pub fn new_from(base_path: &str) -> Result<Self, BuildPackError> {
        let paths = {
            fn read_dir_recursive(base: &str, path: &str, paths: &mut Vec<String>) -> io::Result<()> {
                for entry in fs::read_dir(path)? {
                    let entry = entry?;
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        read_dir_recursive(base, entry_path.to_str().unwrap(), paths)?;
                    } else {
                        let rel = entry_path
                            .strip_prefix(base)
                            .unwrap_or(&entry_path)
                            .to_str()
                            .unwrap()
                            .to_string();
                        paths.push(rel);
                    }
                }

                Ok(())
            }

            let mut paths = Vec::new();
            read_dir_recursive(base_path, base_path, &mut paths).map_err(BuildPackError::IoError)?;

            paths
        };

        let images: Vec<_> = paths.iter().filter(|path| {
            path.ends_with(".webp")
        }).collect();

        let mut rects_to_place: GroupedRectsToPlace<String, ()> = GroupedRectsToPlace::new();
        for image_path in images {
            let dimensions = image::image_dimensions(
                format!("{base_path}/{image_path}")
            ).map_err(BuildPackError::ImageError)?;

            rects_to_place.push_rect(
                image_path.clone(),
                None,
                RectToInsert::new(dimensions.0 + MARGIN * 2, dimensions.1 + MARGIN * 2, 1)
            );
        }

        let mut texture_sizes;
        let mut n = 1;
        let packed_rects = loop {
            let mut target_bins = BTreeMap::new();

            texture_sizes = create_texture_sizes(n, MIN_TEXTURE_SIZE, MAX_TEXTURE_SIZE);
            for (i, (width, height)) in texture_sizes.iter().enumerate() {
                target_bins.insert(i, TargetBin::new(*width, *height, 1));
            }

            let packed_rects = pack_rects(
                &rects_to_place,
                &mut target_bins,
                &rectangle_pack::volume_heuristic,
                &rectangle_pack::contains_smallest_box,
            );

            match packed_rects {
                Ok(packed_rects) => break packed_rects,
                Err(_) => n += 1,
            }
        };

        let mut texture_positions: HashMap<String, Texture> = HashMap::new();
        for (name, (atlas_idx, location)) in packed_rects.packed_locations() {
            texture_positions.insert(
                name.clone().replace(".webp", ""),
                Texture {
                    atlas_idx: *atlas_idx,
                    x: location.x(),
                    y: location.y(),
                    width: location.width(),
                    height: location.height(),
                }
            );
        }

        let mut atlases = Vec::new();
        for texture_size in &texture_sizes {
            let atlas = image::DynamicImage::new_rgba8(texture_size.0, texture_size.1);
            atlases.push(atlas);
        }

        for (name, texture_position) in &texture_positions {
            let name = format!("{base_path}/{}.webp", name);
            let texture = image::open(name).map_err(BuildPackError::ImageError)?;

            atlases[texture_position.atlas_idx]
                .copy_from(&texture, texture_position.x + MARGIN, texture_position.y + MARGIN)
                .map_err(BuildPackError::ImageError)?;
        }

        Ok(AssetStore {
            atlases,
            texture_positions,
            audio: (),
        })
    }
}
