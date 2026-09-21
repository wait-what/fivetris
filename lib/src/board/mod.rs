use std::collections::VecDeque;
use crate::TileGrid;

mod map;
mod apply_piece;
mod lines;

mod hash;
pub use hash::BoardHash;

mod resize;
pub use resize::Corner;

#[derive(Debug, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub tiles: TileGrid,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: VecDeque::from(vec![vec![None; width]; height]),
        }
    }

    #[inline]
    /// (width, height)
    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.tiles.clear();
        self.tiles.resize(self.height, vec![None; self.width]);
    }

    #[inline]
    pub fn get_tiles(&self) -> &TileGrid {
        &self.tiles
    }
}
