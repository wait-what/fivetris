use crate::{ActivePiece, TileGrid};

impl<'a> ActivePiece<'a> {
    pub fn update_ghost_position(&mut self, board: &TileGrid) {
        let mut position = self.position;
        let mut ghost_position = position;

        while !self.collides_at(board, ghost_position, self.rotation) {
            position = ghost_position;
            ghost_position.1 += 1; // Move down
        }

        self.ghost_position = position; // Set ghost position to the last valid position
    }

    #[inline]
    pub fn get_ghost_position(&self) -> (i32, i32) {
        self.ghost_position
    }
}
