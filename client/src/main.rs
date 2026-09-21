use macroquad::prelude::*;
use std::time::Instant;

// pub mod util;
// pub mod input;

// fn window_conf() -> Conf {
//     Conf {
//         window_title: "Five-tris".to_owned(),
//         window_width: 1280,
//         window_height: 720,
//         ..Default::default()
//     }
// }

use lib::{Game, GameOptions, Ruleset, builtin};

use std::time::Duration;

const DAS: Duration = Duration::from_millis(92);
const GRAVITY: Duration = Duration::from_millis(600);
const AUTOLOCK: Duration = Duration::from_millis(800);

struct State<'a> {
    game: Game<'a>,
    start: Instant,
    lines: u32,
    das_start: Option<Instant>,
    das_key: Option<KeyCode>,
    gravity_start: Option<Instant>,
    autolock_start: Option<Instant>,
}

// #[macroquad::main(window_conf)]
#[macroquad::main("Five-tris")]
async fn main() {
    println!("meow!");

    let ruleset: Ruleset = Ruleset::from_json(&builtin::SRS).unwrap();

    let mut state = State {
        game: Game::new(&ruleset, &GameOptions::default()).unwrap(),
        start: Instant::now(),
        lines: 0,
        das_start: None,
        das_key: None,
        gravity_start: None,
        autolock_start: None,
    };

    loop {
        if state.gravity_start.is_none() {
            state.gravity_start = Some(Instant::now());
        } else if state.gravity_start.unwrap().elapsed() >= GRAVITY {
            if state.game.move_down().is_err() {
                if state.autolock_start.is_none() {
                    state.autolock_start = Some(Instant::now());
                } else if state.autolock_start.unwrap().elapsed() >= AUTOLOCK {
                    match state.game.lock_piece() {
                        Ok(Some(count)) => {
                            state.lines += count as u32;
                        },
                        _ => {},
                    }
                    let _ = state.game.move_left();
                    let _ = state.game.move_right();
                    state.autolock_start = None;
                }
            }
            state.gravity_start = Some(Instant::now());
        }

        clear_background(BLACK);

        if is_key_pressed(KeyCode::R) {
            state.game = Game::new(&ruleset, &GameOptions::default()).unwrap();
            state.start = Instant::now();
            state.lines = 0;
        }
        if is_key_pressed(KeyCode::Q) {
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            while state.game.move_down().is_ok() {};
            match state.game.lock_piece() {
                Ok(Some(count)) => {
                    state.lines += count as u32;
                },
                _ => {},
            }
        }

        let left_pressed = is_key_pressed(KeyCode::L) || is_key_pressed(KeyCode::Left);
        let right_pressed = is_key_pressed(KeyCode::Apostrophe) || is_key_pressed(KeyCode::Right);
        let left_down = is_key_down(KeyCode::L) || is_key_down(KeyCode::Left);
        let right_down = is_key_down(KeyCode::Apostrophe) || is_key_down(KeyCode::Right);

        // A newly pressed direction takes precedence, including when both
        // directions are held simultaneously.
        if left_pressed {
            let _ = state.game.move_left();
            state.das_key = Some(KeyCode::Left);
            state.das_start = Some(Instant::now());
        }

        if right_pressed {
            let _ = state.game.move_right();
            state.das_key = Some(KeyCode::Right);
            state.das_start = Some(Instant::now());
        }

        // If the active DAS key is released, transfer control to the other
        // direction if it is still held, starting a fresh DAS delay.
        match state.das_key {
            Some(KeyCode::Left) if !left_down => {
                if right_down {
                    state.das_key = Some(KeyCode::Right);
                    state.das_start = Some(Instant::now());
                } else {
                    state.das_key = None;
                    state.das_start = None;
                }
            },
            Some(KeyCode::Right) if !right_down => {
                if left_down {
                    state.das_key = Some(KeyCode::Left);
                    state.das_start = Some(Instant::now());
                } else {
                    state.das_key = None;
                    state.das_start = None;
                }
            },
            None => {
                if left_down {
                    state.das_key = Some(KeyCode::Left);
                    state.das_start = Some(Instant::now());
                } else if right_down {
                    state.das_key = Some(KeyCode::Right);
                    state.das_start = Some(Instant::now());
                }
            },
            _ => {},
        }

        // DAS only starts after the active direction has been held for 100ms.
        if let Some(das_key) = state.das_key {
            if state.das_start.unwrap().elapsed() >= DAS {
                match das_key {
                    KeyCode::Left if left_down => {
                        state.game.das_left();
                    },
                    KeyCode::Right if right_down => {
                        state.game.das_right();
                    },
                    _ => {},
                }
            }
        }

        if is_key_pressed(KeyCode::Semicolon) || is_key_pressed(KeyCode::Down) {
            state.gravity_start = Some(Instant::now());
        }

        if is_key_down(KeyCode::Semicolon) || is_key_pressed(KeyCode::Down) {
            state.game.das_down();
        }

        if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::LeftControl) || is_key_pressed(KeyCode::Z) {
            let _ = state.game.rotate_ccw();
        }
        if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::C) {
            let _ = state.game.rotate_180();
        }
        if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::Up) {
            let _ = state.game.rotate_cw();
        }
        if is_key_pressed(KeyCode::LeftShift) {
            let _ = state.game.hold();
        }

        let grid_width = 1 + 4 + 1 + 10 + 1 + 4 + 1;
        let grid_height = 3 + 20 + 1;
        let max_grid_dimension = if grid_width < grid_height {
            grid_height
        } else {
            grid_width
        };

        let min_screen_dimension = if screen_width() > screen_height() {
            screen_height()
        } else {
            screen_width()
        };

        let tile_size = min_screen_dimension / max_grid_dimension as f32;

        let (board_width, board_height) = state.game.get_board().get_size();

        let mut board: Vec<Color> = vec![Color::from_rgba(0, 0, 0, 255); board_width * board_height];

        state.game.get_board().map(&mut board, |t, _| {
            match t {
                Some(t) => Some(Color::from_rgba(
                    t.color.0.saturating_sub(30),
                    t.color.1.saturating_sub(30),
                    t.color.2.saturating_sub(30),
                    255,
                )),
                None => None,
            }
        }).unwrap();

        if let Some(active_piece) = state.game.get_active_piece() {
            active_piece.map(&mut board, |t, ghost| {
                match t {
                    Some(t) => Some(Color::from_rgba(
                        if ghost { 160 } else { t.color.0.saturating_add(60) },
                        if ghost { 160 } else { t.color.1.saturating_add(60) },
                        if ghost { 160 } else { t.color.2.saturating_add(60) },
                        if ghost { 160 } else { 255 },
                    )),
                    None => None,
                }
            }, (board_width, board_height)).unwrap();
        }

        // draw board
        let board_origin = (
            1 + 4 + 1,
            1,
        );
        for y in 17..40 {
            for x in 0..10 {
                let screen_x = x + board_origin.0;
                let screen_y = y - 17 + board_origin.1;

                draw_rectangle(
                    screen_x as f32 * tile_size,
                    screen_y as f32 * tile_size,
                    tile_size,
                    tile_size,
                    board[x + y * board_width],
                );
            }
        }

        // draw hold piece
        let hold_origin = (1, 4);
        if let Some(hold_piece) = state.game.get_hold_piece().0 {
            let shape = & hold_piece.shape[0];
            for y in 0..shape.len() {
                for x in 0..shape[y].len() {
                    if let Some(tile) = &shape[y][x] {
                        draw_rectangle(
                            (x + hold_origin.0) as f32 * tile_size,
                            (y + hold_origin.1) as f32 * tile_size,
                            tile_size,
                            tile_size,
                            Color::from_rgba(tile.color.0, tile.color.1, tile.color.2, 255),
                        );
                    }
                }
            }
        }

        // draw stats
        let stats_origin = (1, 10);
        draw_text(
            &format!("Lines\n{}", state.lines),
            stats_origin.0 as f32 * tile_size,
            stats_origin.1 as f32 * tile_size,
            40.0,
            WHITE,
        );
        draw_text(
            &format!("Time\n{:.1}s", state.start.elapsed().as_secs_f32()),
            stats_origin.0 as f32 * tile_size,
            (stats_origin.1 + 1) as f32 * tile_size,
            40.0,
            WHITE,
        );
        draw_text(
            &format!("LPM\n{:.1}l/s", state.lines as f32 / (state.start.elapsed().as_secs_f32() / 60.0)),
            stats_origin.0 as f32 * tile_size,
            (stats_origin.1 + 2) as f32 * tile_size,
            40.0,
            WHITE,
        );


        // draw queue
        let queue_origin = (17, 4);
        for i in 0..5 {
            let piece = state.game.get_queue().get(i).unwrap();
            let shape = &piece.shape[0];

            for y in 0..shape.len() {
                for x in 0..shape[y].len() {
                    if let Some(tile) = &shape[y][x] {
                        draw_rectangle(
                            (x + queue_origin.0) as f32 * tile_size,
                            (y as u32 + queue_origin.1 + i as u32 * 4) as f32 * tile_size,
                            tile_size,
                            tile_size,
                            Color::from_rgba(tile.color.0, tile.color.1, tile.color.2, 255),
                        );
                    }
                }
            }
        }


        // draw gridlines
        for y in 4..=grid_height {
            draw_line(
                tile_size * 6.0,
                y as f32 * tile_size,
                (grid_width - 6) as f32 * tile_size,
                y as f32 * tile_size,
                1.0,
                GRAY,
            );
        }
        for x in 6..(grid_width - 5) {
            draw_line(
                x as f32 * tile_size,
                4.0 * tile_size,
                x as f32 * tile_size,
                (grid_height) as f32 * tile_size,
                1.0,
                GRAY,
            );
        }

        next_frame().await
    }
}
