use crate::{ActivePiece, Board};

impl<'a> ActivePiece<'a>  {
    #[inline]
    pub fn get_position(&self) -> (i32, i32) {
        self.position
    }

    pub fn move_to(&mut self, new_position: (i32, i32), board: &Board) -> Result<(), ()> {
        let tiles = board.get_tiles();

        if !self.collides_at(tiles, new_position, self.rotation) {
            self.position = new_position;
            self.update_ghost_position(tiles);

            Ok(())
        } else {
            Err(())
        }
    }
}
