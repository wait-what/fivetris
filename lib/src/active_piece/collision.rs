use crate::{ActivePiece, TileGrid};

impl<'a> ActivePiece<'a> {
    pub fn collides_at(&self, board: &TileGrid, (x, y): (i32, i32), rotation: u8) -> bool {
        let shape = &self.piece.shape[rotation as usize];

        for shape_y in 0..shape.len() {
            for shape_x in 0..shape[shape_y].len() {
                if let Some(_) = &shape[shape_y][shape_x] {
                    let board_x = (x + shape_x as i32) as usize;
                    let board_y = (y + shape_y as i32) as usize;

                    // Check if the tile is out of bounds or collides with an existing tile
                    if board_x >= board[0].len() || board_y >= board.len() || board[board_y][board_x].is_some() {
                        return true; // Collision detected
                    }
                }
            }
        }

        false
    }
}
