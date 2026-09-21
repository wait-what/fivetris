use crate::Game;

impl<'a> Game<'a> {
    /// Internal function!
    pub fn clear_full_lines(&mut self) -> Result<usize, ()> {
        let mut amount = 0;

        loop {
            if self.board.try_clear_line().is_some() {
                amount += 1;
            } else {
                return Ok(amount);
            }
        }
    }
}
