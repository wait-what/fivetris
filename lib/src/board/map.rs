use crate::{Board, TileInto};

impl Board {
    /// ```rs
    /// let mut buf = vec![' '; 10 * 20];
    /// board.map(&mut buf, |t| { if t.is_some() { Some('#') } else { Some(' ') } }, (10, 20));
    /// active_piece.map(&mut buf, |t| { if t.is_some() { Some('@') } else { None } }, (10, 20));
    /// ```
    /// Returns Err if the buffer is too small
    pub fn map<T>(&self, buf: &mut [T], f: TileInto<T>) -> Result<(), ()> {
        if buf.len() < self.width * self.height {
            return Err(());
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                if let Some(new_value) = f(&self.tiles[y][x], false) {
                    buf[index] = new_value;
                }
            }
        }

        Ok(())
    }
}
