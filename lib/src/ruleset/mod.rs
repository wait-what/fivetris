use std::collections::HashMap;
use crate::{Tile, TileGrid};

#[cfg(feature = "ruleset_builtin")]
pub mod builtin;

mod deserialize;

pub type KickTable = Vec<(i32, i32)>;

#[derive(Clone, Debug)]
pub struct Piece {
    pub name: String,
    pub shape: [TileGrid; 4],
    pub kick_tables: HashMap<(u8, u8), KickTable>,
    pub spawn_offset: (i32, i32),
    pub spawn_rotation: u8,
    pub spin_rules: (), // TODO
}

#[derive(Clone, Debug)]
pub struct Ruleset {
    pub pieces: Vec<Piece>,
    pub garbage_tile: Tile,
}
