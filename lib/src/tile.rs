use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
pub enum TileKind {
    Locked { garbage: bool },
    Active { pivot: bool },
    Permanent,
}

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub color: (u8, u8, u8),
    pub kind: TileKind,
}

pub type TileInto<T> = fn(tile: &Option<Tile>, ghost: bool) -> Option<T>;

pub type TileLine = Vec<Option<Tile>>;
pub type TileGrid = VecDeque<TileLine>;
