use crate::{QueueKind, Piece};

pub struct GameOptions {
    /// Board `(width, height)`, not including `above_board_height`
    pub board_size: (usize, usize),
    /// "Secret" height above the board
    pub above_board_height: usize,
    /// Amount of holds allowed per turn. Resets on piece lock
    pub holds_allowed: usize,
    pub queue_kind: QueueKind<Piece>,
    /// Minimum amoutn of pieces to keep in the queue at all times (for previews)
    pub queue_min_stock: usize,
    pub garbage_seed: u64,
    /// Maximum amount of garbage that can be queued at once
    pub garbage_limit_total: usize,
    pub garbage_limit_turn: usize,
    /// Don't push garbage if a turn ends with a line clear
    pub garbage_skip_on_cleared: bool,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            board_size: (10, 20),
            above_board_height: 20,
            holds_allowed: 1,
            queue_kind: QueueKind::BagRng(123),
            queue_min_stock: 14,
            garbage_seed: 123,
            garbage_limit_total: 24,
            garbage_limit_turn: 8,
            garbage_skip_on_cleared: true,
        }
    }
}
