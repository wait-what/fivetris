use crate::{ActivePiece, TileGrid};

pub enum RotationResult {
    Kick(usize),
    /// TODO:
    Spin(usize),
}

impl<'a> ActivePiece<'a> {
    #[inline]
    pub fn get_rotation(&self) -> u8 {
        self.rotation
    }

    pub fn rotate(&mut self, board: &TileGrid, from: u8, to: u8) -> Result<RotationResult, ()> {
        let kick_table = self.piece.kick_tables.get(&(from, to)).unwrap();

        for (index, kick) in kick_table.iter().enumerate() {
            let new_position = (
                (self.position.0 + kick.0),
                (self.position.1 + kick.1)
            );

            if !self.collides_at(board, new_position, to) {
                self.position = new_position;
                self.rotation = to;

                self.update_ghost_position(board);

                return Ok(RotationResult::Kick(index));
            }
        }

        return Err(());
    }
}
