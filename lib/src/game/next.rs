use crate::{Game, ActivePiece};

#[derive(Copy, Clone, Debug)]
pub enum NextPieceError {
    QueueEmpty,
    SpawnCollision,
}

impl<'a> Game<'a> {
    /// Internal function!
    pub fn next_piece(&mut self) -> Result<(), NextPieceError> {
        let next_piece = match self.queue.pop_front() {
            Some(piece) => piece,
            None => return Err(NextPieceError::QueueEmpty),
        };

        self.active_piece = match ActivePiece::new(next_piece, &self.board, self.above_board_height) {
            Ok(active_piece) => Some(active_piece),
            Err(_) => return Err(NextPieceError::SpawnCollision),
        };

        Ok(())
    }

    pub fn lock_piece(&mut self) -> Result<Option<usize>, NextPieceError> {
        if let Some(active_piece) = &mut self.active_piece {
            self.board.apply_piece(active_piece);
            self.reset_hold_count();

            let clear_count = self.clear_full_lines().unwrap();
            if clear_count == 0 || !self.garbage.get_skip_on_cleared() {
                 self.apply_garbage();
            }

            match self.next_piece() {
                Ok(_) => Ok(Some(clear_count)),
                Err(err) => Err(err),
            }
        } else {
            return Err(NextPieceError::QueueEmpty);
        }
    }

    pub fn hard_drop(&mut self) -> Result<Option<usize>, NextPieceError> {
        self.das_down();
        self.lock_piece()
    }
}
