use std::collections::VecDeque;
use crate::{Game, Tile, TileLine, rng};

pub struct Garbage {
    queue: VecDeque<usize>,
    seed: u64,
    total_limit: usize,
    turn_limit: usize,
    skip_on_cleared: bool,
}

impl Garbage {
    pub fn new(seed: u64, total_limit: usize, turn_limit: usize, skip_on_cleared: bool) -> Self {
        Self {
            queue: VecDeque::new(),
            seed,
            total_limit,
            turn_limit,
            skip_on_cleared,
        }
    }

    fn next_u64(&mut self) -> u64 {
        let random = rng::next(self.seed);
        self.seed = random;
        random
    }

    pub fn generate_line(&mut self, tile: Tile, width: usize) -> TileLine {
        let mut line = vec![Some(tile); width];

        let random_index = (self.next_u64() as usize) % width;
        line[random_index] = None;

        line
    }

    pub fn get_queue(&self) -> &VecDeque<usize> {
        &self.queue
    }

    pub fn get_total(&self) -> usize {
        self.queue.iter().sum()
    }

    pub fn get_total_limit(&self) -> usize {
        self.total_limit
    }

    pub fn get_turn_limit(&self) -> usize {
        self.turn_limit
    }

    pub fn get_skip_on_cleared(&self) -> bool {
        self.skip_on_cleared
    }
}

impl<'a> Game<'a> {
    /// Internal function!
    /// `Some(n)` if n lines of garbage were applied
    /// Applies from the front (first in)
    pub fn apply_garbage(&mut self) -> Option<Vec<usize>> {
        let mut remaining = self.garbage.turn_limit;
        let mut applied = Vec::new();

        let (width, height) = self.board.get_size();

        while remaining > 0 {
            let Some(mut count) = self.garbage.queue.pop_front() else {
                break;
            };

            let amount = count.min(remaining);
            count -= amount;

            let line = self.garbage.generate_line(self.ruleset.garbage_tile, width);

            for _ in 0..amount {
                self.board.insert_line(height, line.clone()).unwrap();
            }

            remaining -= amount;
            applied.push(amount);

            if count > 0 {
                self.garbage.queue.push_front(count);
                break;
            }
        }

        (!applied.is_empty()).then_some(applied)
    }

    /// `Err(n)` if `n` lines of garbage didn't fit into `total_garbage_limit`
    pub fn queue_garbage(&mut self, amount: usize) -> Result<(), usize> {
        let current_amount = self.garbage.queue.iter().sum::<usize>();
        let to_add = amount.min(self.garbage.total_limit.saturating_sub(current_amount));

        self.garbage.queue.push_back(to_add);

        if to_add < amount {
            Err(amount - to_add)
        } else {
            Ok(())
        }
    }

    /// `Some(n)` if `n` lines of garbage were canceled
    /// Cancels from the front (first in)
    pub fn cancel_garbage(&mut self, amount: usize) -> Option<usize> {
        let mut canceled = 0;
        let mut remaining = amount;

        while remaining > 0 {
            if let Some(front) = self.garbage.queue.front_mut() {
                if *front <= remaining {
                    canceled += *front;
                    remaining -= *front;
                    self.garbage.queue.pop_front();
                } else {
                    canceled += remaining;
                    *front -= remaining;
                    remaining = 0;
                }
            } else {
                break;
            }
        }

        if canceled > 0 {
            Some(canceled)
        } else {
            None
        }
    }
}
