use crate::Game;

impl<'a> Game<'a> {
    /// Internal function!
    pub fn move_by(&mut self, offset: (i8, i8)) -> Result<(), ()> {
        if let Some(active_piece) = &mut self.active_piece {
            let position = active_piece.get_position();

            let new_position = (
                position.0 + offset.0 as i32,
                position.1 + offset.1 as i32
            );

            return active_piece.move_to(new_position, &self.board);
        } else {
            return Err(());
        }
    }

    /// Internal function!
    pub fn das_by(&mut self, direction: (i8, i8)) {
        loop {
            if self.move_by(direction).is_err() {
                break;
            }
        }
    }

    pub fn move_left(&mut self) -> Result<(), ()> {
        self.move_by((-1, 0))
    }

    pub fn move_right(&mut self) -> Result<(), ()> {
        self.move_by((1, 0))
    }

    pub fn move_down(&mut self) -> Result<(), ()> {
        self.move_by((0, 1))
    }

    pub fn das_left(&mut self) {
        self.das_by((-1, 0));
    }

    pub fn das_right(&mut self) {
        self.das_by((1, 0));
    }

    pub fn das_down(&mut self) {
        self.das_by((0, 1));
    }
}
