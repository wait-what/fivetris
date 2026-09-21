use crate::{Board, TileLine};

impl Board {
    pub fn try_clear_line(&mut self) -> Option<(usize, TileLine)> {
        let mut cleared_line = None;

        for y in 0..self.height {
            let mut line_filled = true;

            for x in 0..self.width {
                if self.tiles[y][x].is_none() {
                    line_filled = false;
                    break;
                }
            }

            if line_filled {
                cleared_line = Some((y, self.tiles.remove(y).unwrap()));
                self.tiles.push_front(vec![None; self.width]);
                break;
            }
        }

        cleared_line
    }

    /// Ok(top_most_line)
    /// `Err(())` if position > height
    pub fn insert_line(&mut self, position: usize, line: TileLine) -> Result<TileLine, ()> {
        if position > self.height {
            return Err(());
        }

        self.tiles.insert(position, line);

        Ok(self.tiles.pop_front().unwrap()) // todo??
    }
}
