use crate::{Board, Piece, TileGrid};

mod ghost;
mod collision;
mod movement;
mod map;
mod rotation;
pub use rotation::RotationResult;

#[derive(Debug, Clone)]
pub struct ActivePiece<'a> {
    piece: &'a Piece,
    position: (i32, i32),
    ghost_position: (i32, i32),
    rotation: u8,
}

impl<'a> ActivePiece<'a> {
    pub fn new(piece: &'a Piece, board: &Board, above_board_height: usize) -> Result<Self, ()> {
        let (board_width, board_height) = board.get_size();

        let position = (
            (board_width / 2) as i32 + piece.spawn_offset.0,
            (board_height as i32 - above_board_height as i32 + piece.spawn_offset.1),
        );

        let mut active_piece = Self {
            piece,
            position,
            ghost_position: position,
            rotation: piece.spawn_rotation,
        };

        if active_piece.collides_at(board.get_tiles(), active_piece.position, active_piece.rotation) {
            return Err(());
        }

        active_piece.update_ghost_position(board.get_tiles());

        Ok(active_piece)
    }

    #[inline]
    pub fn get_piece(&self) -> &'a Piece {
        self.piece
    }

    #[inline]
    pub fn get_current_shape(&self) -> &TileGrid {
        &self.piece.shape[self.rotation as usize]
    }
}
