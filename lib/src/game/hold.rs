use crate::{Game, ActivePiece, NextPieceError};

#[derive(Copy, Clone, Debug)]
pub enum HoldError {
    OutOfHolds,
    QueueEmpty,
    SpawnCollision,
}

impl<'a> Game<'a> {
    pub fn hold(&mut self) -> Result<(), HoldError> {
        let (hold_piece, hold_count) = self.hold_piece;

        if hold_count >= self.holds_allowed {
            return Err(HoldError::OutOfHolds);
        }

        let active_piece = match self.active_piece.take() {
            Some(piece) => piece,
            None => return Err(HoldError::QueueEmpty),
        };


        self.hold_piece = (Some(active_piece.get_piece()), hold_count + 1);

        if let Some(hold_piece) = hold_piece {
            self.active_piece = match ActivePiece::new(hold_piece, &self.board, self.above_board_height) {
                Ok(active_piece) => Some(active_piece),
                Err(_) => return Err(HoldError::SpawnCollision),
            };
        } else {
            return self.next_piece().map_err(|err| match err {
                NextPieceError::QueueEmpty => HoldError::QueueEmpty,
                NextPieceError::SpawnCollision => HoldError::SpawnCollision,
            });
        }

        Ok(())
    }

    /// Internal function!
    pub fn reset_hold_count(&mut self) {
        self.hold_piece.1 = 0;
    }
}
