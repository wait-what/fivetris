use crate::{Board, Tile};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Board {
    /// If resizing to a smaller size, the tiles will be removed from the opposite of `primary_corner`
    /// If resizing to a larger size, the tiles will be added to the opposite of `primary_corner`
    pub fn resize(&mut self, width: usize, height: usize, _primary_corner: Corner, _fill_with: Option<Tile>) -> Result<(), ()> {
        if width == self.width && height == self.height {
            return Ok(()); // No resize needed
        }

        if width == 0 || height == 0 {
            return Err(()); // Invalid size
        }

        todo!("board resize");
    }
}
