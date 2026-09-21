use crate::{ActivePiece, TileInto};

impl<'a> ActivePiece<'a> {
    /// ```rs
    /// let mut buf = vec![' '; 10 * 20];
    /// board.map(&mut buf, |t| { if t.is_some() { Some('#') } else { Some(' ') } }, (10, 20));
    /// active_piece.map(&mut buf, |t| { if t.is_some() { Some('@') } else { None } }, (10, 20));
    /// ```
    /// Returns Err if the buffer is too small
    pub fn map<T>(&self, buf: &mut [T], f: TileInto<T>, board_size: (usize, usize)) -> Result<(), ()> {
        let (width, height) = board_size;

        if buf.len() < width * height {
            return Err(());
        }

        let shape = self.get_current_shape();

        for (position, is_ghost) in [(self.ghost_position, true), (self.position, false)] {
            for (y, row) in shape.iter().enumerate() {
                for (x, tile) in row.iter().enumerate() {
                    let board_x = (position.0 + x as i32) as usize;
                    let board_y = (position.1 + y as i32) as usize;

                    if board_x < width && board_y < height {
                        let index = board_y * width + board_x;
                        if let Some(new_value) = f(tile, is_ghost) {
                            buf[index] = new_value;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
