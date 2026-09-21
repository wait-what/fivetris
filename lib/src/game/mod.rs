use crate::{ActivePiece, Board, Queue, Ruleset, Piece};

mod clear;
mod garbage;
mod hold;
mod movement;
mod next;
mod options;
mod rotation;

pub use garbage::Garbage;
pub use hold::HoldError;
pub use next::NextPieceError;
pub use options::GameOptions;

pub struct Game<'a> {
    ruleset: &'a Ruleset,
    queue: Queue<'a, Piece>,
    active_piece: Option<ActivePiece<'a>>,
    hold_piece: (Option<&'a Piece>, usize),
    holds_allowed: usize,
    board: Board,
    above_board_height: usize,
    garbage: Garbage,
}

impl<'a> Game<'a> {
    pub fn new(ruleset: &'a Ruleset, options: &GameOptions) -> Result<Self, NextPieceError> {
        let (width, height) = options.board_size;

        let mut game = Self {
            queue: Queue::new(&ruleset.pieces, options.queue_kind.clone(), options.queue_min_stock),
            ruleset,
            active_piece: None,
            hold_piece: (None, 0),
            holds_allowed: options.holds_allowed,
            board: Board::new(width, height + options.above_board_height),
            above_board_height: options.above_board_height,
            garbage: Garbage::new(options.garbage_seed, options.garbage_limit_total, options.garbage_limit_turn, options.garbage_skip_on_cleared),
        };

        match game.next_piece() {
            Ok(_) => Ok(game),
            Err(err) => Err(err),
        }
    }

    pub fn get_hold_piece(&self) -> (Option<&'a Piece>, usize) {
        self.hold_piece
    }

    pub fn get_active_piece(&self) -> Option<&ActivePiece<'a>> {
        self.active_piece.as_ref()
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_above_board_height(&self) -> usize {
        self.above_board_height
    }

    pub fn get_queue(&self) -> &Queue<'a, Piece> {
        &self.queue
    }

    pub fn get_garbage(&self) -> &Garbage {
        &self.garbage
    }
}
