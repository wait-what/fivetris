use std::{collections::VecDeque, fmt};
use crate::rng;

type BagGenerator<T> = fn(&Vec<T>) -> Result<Vec<usize>, ()>;

#[derive(Debug, Clone)]
pub enum QueueKind<T> {
    BagRng(u64), // Shuffle elements and append to queue
    RandomRng(u64), // Randomly select elements in any order
    Repeating, // Repeat all elements in the same order
    Preset(usize), // All given elements n times, will eventually run out
    Custom(BagGenerator<T>), // Custom generation closure
}

#[derive(Debug, Clone)]
pub struct Queue<'a, T> {
    queue: VecDeque<usize>,
    elements: &'a Vec<T>,
    min_stock: usize,
    kind: QueueKind<T>,
}

impl<'a, T: 'a + Clone> Queue<'a, T> {
    pub fn new(elements: &'a Vec<T>, kind: QueueKind<T>, min_stock: usize) -> Self {
        let mut queue = Self {
            queue: VecDeque::new(),
            elements,
            min_stock,
            kind,
        };

        while queue.fill().is_ok() && queue.queue.len() < queue.min_stock {}

        queue
    }

    fn fill(&mut self) -> Result<(), ()> {
        match &mut self.kind {
            QueueKind::BagRng(seed) => {
                let len = self.elements.len();
                let mut bag = Vec::with_capacity(len);

                for i in 0..len {
                    bag.push(i);
                }

                for i in 0..len {
                    let random = rng::next(*seed);
                    *seed = random;

                    bag.swap(i, random as usize % len);
                };

                self.queue.extend(bag);

                Ok(())
            },
            QueueKind::RandomRng(seed) => {
                let len = self.elements.len();

                let random = rng::next(*seed);
                *seed = random;
                self.queue.push_back(random as usize % len);

                Ok(())
            },
            QueueKind::Repeating => {
                for i in 0..self.elements.len() {
                    self.queue.push_back(i);
                }

                Ok(())
            },
            QueueKind::Preset(repeats) => {
                if *repeats == 0 {
                    return Err(());
                }

                *repeats -= 1;

                for i in 0..self.elements.len() {
                    self.queue.push_back(i);
                }

                Ok(())
            },
            QueueKind::Custom(generator) => {
                let new_bag = generator(&self.elements)?;
                self.queue.extend(new_bag);
                Ok(())
            }
        }
    }

    #[inline]
    pub fn pop_front(&mut self) -> Option<&'a T> {
        if self.queue.len() < self.min_stock {
            let _ = self.fill();
        }

        let index = self.queue.pop_front()?;

        self.elements.get(index)
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&'a T> {
        if index >= self.queue.len() {
            return None;
        }

        let index = self.queue.get(index)?;
        self.elements.get(*index)
    }

    #[inline]
    pub fn elements(&self) -> &Vec<T> {
        self.elements
    }
}

impl<T: fmt::Display> fmt::Display for Queue<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Queue with {} pieces", self.queue.len())
    }
}
