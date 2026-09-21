use super::Board;
use crate::ActivePiece;

pub type BoardHash = Vec<u8>;

// TODO: optimize

impl Board {
    pub fn hash(&self) -> BoardHash {
        let mut hash = vec![0; self.width * self.height / 8 + 1];

        for y in 0..self.height {
            for x in 0..self.width {
                if self.tiles[y][x].is_some() {
                    let index = y * self.width + x;
                    hash[index / 8] |= 1 << (index % 8); // set index-th byte to 1
                }
            }
        }

        hash
    }

    /// Board hash including the active piece, as if it were placed at its current position
    pub fn hash_with_piece(&self, piece: &ActivePiece) -> BoardHash {
        let mut hash = self.hash();

        let shape = piece.get_current_shape();
        let position = piece.get_position();

        for x in 0..shape[0].len() {
            for y in 0..shape.len() {
                if let Some(_) = &shape[y][x] {
                    let board_x = position.0 + x as i32;
                    let board_y = position.1 + y as i32;

                    if board_x < 0 || board_x as usize >= self.width || board_y < 0 || board_y as usize >= self.height {
                        continue;
                    }

                    let index = board_y as usize * self.width + board_x as usize;
                    hash[index / 8] |= 1 << (index % 8); // set index-th byte to 1
                }
            }
        }

        hash
    }
}
