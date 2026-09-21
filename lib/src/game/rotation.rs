use crate::{RotationResult, Game};

impl<'a> Game<'a> {
    /// Internal function!
    pub fn rotate_to(&mut self, to: u8) -> Result<RotationResult, ()> {
        if let Some(active_piece) = &mut self.active_piece {
            let from = active_piece.get_rotation();
            return active_piece.rotate(&self.board.tiles, from, to);
        } else {
            return Err(());
        }
    }

    pub fn rotate_cw(&mut self) -> Result<RotationResult, ()> {
        if let Some(active_piece) = &mut self.active_piece {
            let from = active_piece.get_rotation();
            let to = (from + 1) % 4;
            return active_piece.rotate(&self.board.tiles, from, to);
        } else {
            return Err(());
        }
    }

    pub fn rotate_ccw(&mut self) -> Result<RotationResult, ()> {
        if let Some(active_piece) = &mut self.active_piece {
            let from = active_piece.get_rotation();
            let to = (from + 3) % 4;
            return active_piece.rotate(&self.board.tiles, from, to);
        } else {
            return Err(());
        }
    }

    pub fn rotate_180(&mut self) -> Result<RotationResult, ()> {
        if let Some(active_piece) = &mut self.active_piece {
            let from = active_piece.get_rotation();
            let to = (from + 2) % 4;
            return active_piece.rotate(&self.board.tiles, from, to);
        } else {
            return Err(());
        }
    }
}
