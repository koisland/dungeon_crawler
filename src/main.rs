use macroquad::prelude::*;
use std::time::{Duration, Instant};

mod enemy;
mod map;
mod player;
mod ray;
mod screen;
mod state;
mod textures;
mod tiles;

use crate::{screen::Screen, state::GameState, textures::Textures};

const WIDTH: usize = 1024;
const HEIGHT: usize = 512;

// https://github.com/not-fl3/macroquad/issues/380#issuecomment-4320788661
fn window_conf() -> Conf {
    Conf {
        window_title: "tinyraycaster".to_owned(),
        window_resizable: false,
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> eyre::Result<()> {
    let mut gs = GameState::new("data/map.txt", "data/state.tsv")?;
    let textures = Textures::new("data/textures.tsv", 32)?;
    let mut screen = Screen::new(WIDTH, HEIGHT, gs.map.w * 10, gs.map.h * 10);

    // https://github.com/not-fl3/macroquad/issues/380#issuecomment-4775299639
    let fps_target = 60.0;
    let frame_dur = Duration::from_secs_f64(1.0 / fps_target);
    let mut next_tick = Instant::now();

    loop {
        // Limit FPS
        next_tick += frame_dur;
        let now = Instant::now();
        if next_tick > now {
            spin_sleep::sleep(next_tick - now); // spin_sleep crate; might be more reliable than std::thread::sleep
        } else {
            next_tick = now; // catch up after a slow frame
        }

        // Exit
        if is_key_down(KeyCode::Escape) || is_quit_requested() {
            break Ok(());
        }
        // Update state
        gs.update_all();

        // Render frame
        screen.render(&mut gs, &textures)?;

        // FPS
        if is_key_down(KeyCode::F) {
            draw_text(
                format!(
                    "{:.4} s/{:.0} fps",
                    get_frame_time(),
                    1.0 / (get_frame_time() as f32)
                )
                .as_str(),
                0.0,
                20.0,
                30.0,
                GREEN,
            );
        }

        next_frame().await;
    }
}
