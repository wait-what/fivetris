use crate::{Board, Tile, TileKind, ActivePiece};

impl Board {
    pub fn apply_piece(&mut self, piece: &ActivePiece) {
        let shape = &piece.get_current_shape();
        let position = piece.get_position();

        for y in 0..shape.len() {
            for x in 0..shape[y].len() {
                if shape[y][x].is_none() {
                    continue;
                }

                let tile = shape[y][x].as_ref().unwrap().clone();

                let board_x = (position.0 + x as i32) as usize;
                let board_y = (position.1 + y as i32) as usize;

                self.tiles[board_y][board_x] = Some(Tile {
                    color: tile.color,
                    kind: TileKind::Locked { garbage: false },
                });
            }
        }
    }
}
