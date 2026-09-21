use crate::{TileGrid, Tile, TileKind, Piece, Ruleset};
use nanoserde::{DeJson, DeJsonErr};
use std::collections::{BTreeMap, VecDeque};

const ROTATIONS: [(u8, u8); 12] = [
    (0, 1),
    (1, 0),
    (1, 2),
    (2, 1),
    (2, 3),
    (3, 2),
    (3, 0),
    (0, 3),
    (0, 2),
    (2, 0),
    (1, 3),
    (3, 1),
];

#[derive(DeJson, Debug)]
struct RulesetJson {
    kick_tables: BTreeMap<String, [Vec<(i32, i32)>; 12]>,
    garbage_color: String,
    pieces: BTreeMap<String, PieceJson>,
}

#[derive(DeJson, Debug)]
struct PieceJson {
    shape: [Vec<Vec<i8>>; 4],
    color: String,
    kick_table: String,
    spawn_offset: (i32, i32),
    spawn_rotation: u8,
}

fn parse_color(color: &str) -> Result<(u8, u8, u8), ()> {
    let color = color.trim_start_matches('#');

    let color = u32::from_str_radix(color, 16).unwrap_or(0xFFFFFFFF);
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;

    Ok((r, g, b))
}

impl Ruleset {
    pub fn from_json(json: &str) -> Result<Self, DeJsonErr> {
        let json = RulesetJson::deserialize_json(json)?;

        let mut pieces: Vec<Piece> = Vec::new();
        for (name, piece) in json.pieces {
            let piece = Piece {
                name: name.clone(),
                shape: piece.shape.map(|shape| {
                    let (width, height) = (shape[0].len(), shape.len());
                    let mut grid: TileGrid = VecDeque::from(vec![vec![None; width]; height]);

                    for (y, row) in shape.iter().enumerate() {
                        for (x, &value) in row.iter().enumerate() {
                            if value & 1 != 0 {
                                grid[y][x] = Some(Tile {
                                    color: parse_color(&piece.color).unwrap(),
                                    kind: TileKind::Active {
                                        pivot: value & 2 != 0,
                                    },
                                });
                            };
                        }
                    }

                    grid
                }),
                kick_tables: {
                    let kick_tables = json.kick_tables.get(&piece.kick_table).unwrap();

                    kick_tables
                        .iter()
                        .enumerate()
                        .map(|(index, kick_table)| {
                            let (from, to) = ROTATIONS[index];
                            ((from, to), kick_table.clone())
                        })
                        .collect()
                },
                spawn_offset: piece.spawn_offset,
                spawn_rotation: piece.spawn_rotation as u8,
                spin_rules: (),
            };

            pieces.push(piece);
        }

        let garbage_tile = Tile {
            color: parse_color(&json.garbage_color).unwrap(),
            kind: TileKind::Locked { garbage: true },
        };

        Ok(Self { pieces, garbage_tile })
    }
}
