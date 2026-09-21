use lib::{Game, GameOptions, Ruleset, Tile, TileKind, builtin::SRS};

const COLOR_TERMINAL: bool = true;

fn game_to_string(game: &Game) -> String {
    let board_size = game.get_board().get_size();
    let (width, height) = (board_size.0, board_size.1);

    let mut buf = vec![String::from(" "); width * height];

    fn tile_to_string(tile: &Tile, ghost: bool) -> String {
        let escape = match tile.color {
            (0x80, 0x80, 0x80) => "\x1b[38;5;8m",
            (0x00, 0xFF, 0xFF) => "\x1b[38;5;14m",
            (0xFF, 0x80, 0x00) => "\x1b[38;5;208m",
            (0x00, 0x00, 0xFF) => "\x1b[38;5;12m",
            (0x00, 0xFF, 0x00) => "\x1b[38;5;10m",
            (0xFF, 0x00, 0x00) => "\x1b[38;5;9m",
            (0xFF, 0x00, 0xFF) => "\x1b[38;5;13m",
            (0xFF, 0xFF, 0x00) => "\x1b[38;5;11m",
            _ => "\x1b[39m",
        };

        let character = match (tile.kind, ghost) {
            (TileKind::Locked { garbage: true }, _) => "░",
            (TileKind::Locked { garbage: false }, _) => "▓",
            (TileKind::Active { pivot: true }, false) => "▣",
            (TileKind::Active { pivot: false }, false) => "■",
            (TileKind::Active { pivot: _ }, true) => "◇",
            (TileKind::Permanent, _) => "█",
        };

        if COLOR_TERMINAL {
            format!("{}{}\x1b[39m", escape, character)
        } else {
            character.to_string()
        }
    }

    game.get_board().map(&mut buf, |t, _| {
        if let Some(tile) = t {
            Some(tile_to_string(tile, false))
        } else {
            Some(String::from(" "))
        }
    }).unwrap();

    game.get_active_piece().as_ref().unwrap().map(&mut buf, |t, ghost| {
        if let Some(tile) = t {
            Some(tile_to_string(tile, ghost))
        } else {
            None
        }
    }, board_size).unwrap();

    let mut out = String::new();

    for y in 0..height {
        if y < game.get_above_board_height() - 4{
            continue;
        }

        out.push_str(&format!("{:02} ", y));

        for x in 0..width {
            out.push_str(&buf[x + y * width]);
        }
        out.push('\n');
    }

    out.push_str(&format!("   0123456789"));

    out
}

fn main() {
    let ruleset = Ruleset::from_json(SRS).unwrap();
    let mut game = Game::new(&ruleset, &GameOptions::default()).unwrap();

    // TKI:
    game.move_right().unwrap();
    game.move_right().unwrap();
    game.rotate_180().unwrap();
    game.hard_drop().unwrap();

    game.das_right();
    game.hard_drop().unwrap();

    game.das_left();
    game.das_down();
    game.das_right();
    game.hard_drop().unwrap();

    game.rotate_ccw().unwrap();
    game.hard_drop().unwrap();

    game.hold().unwrap();

    game.rotate_cw().unwrap();
    game.das_left();
    game.hard_drop().unwrap();

    game.move_right().unwrap();
    game.hard_drop().unwrap();

    game.hold().unwrap();

    // TSD:
    game.rotate_ccw().unwrap();
    game.move_left().unwrap();
    game.move_left().unwrap();
    game.das_down();
    game.rotate_ccw().unwrap();

    println!("{}", game_to_string(&game));

    game.hard_drop().unwrap();

    println!("{}", game_to_string(&game));

    // garbage:
    game.queue_garbage(3).unwrap();
    game.queue_garbage(1).unwrap();
    game.queue_garbage(5).unwrap();
    game.das_right();
    game.hard_drop().unwrap();

    println!("{}", game_to_string(&game));
    println!("Garbage queue: {:?}\n", game.get_garbage().get_queue());

    game.rotate_ccw().unwrap();
    game.move_left().unwrap();
    game.move_left().unwrap();
    game.hard_drop().unwrap();

    game.hold().unwrap();

    println!("{}", game_to_string(&game));

}
