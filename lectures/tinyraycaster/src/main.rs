use macroquad::prelude::*;
use std::{
    f32::consts::PI,
    time::{Duration, Instant},
};

mod enemy;
mod map;
mod player;
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
        sample_count: 0,
        platform: miniquad::conf::Platform {
            swap_interval: Some(0),
            ..Default::default()
        },
        window_resizable: false,
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> eyre::Result<()> {
    let mut gs = GameState::new(
        "lectures/tinyraycaster/data/map.txt",
        "lectures/tinyraycaster/data/state.tsv",
    )?;
    let textures = Textures::new("lectures/tinyraycaster/data/textures.tsv", 64)?;
    let mut screen = Screen::<WIDTH, HEIGHT>::new();

    // https://github.com/not-fl3/macroquad/issues/380#issuecomment-4775299639
    let fps_target = 60.0;
    let frame_dur = Duration::from_secs_f64(1.0 / fps_target);
    let mut next_tick = Instant::now();
    loop {
        // Logic
        gs.player.ang += 0.5 * PI / 360.0;

        // Limit FPS
        next_tick += frame_dur;
        let now = Instant::now();
        if next_tick > now {
            spin_sleep::sleep(next_tick - now); // spin_sleep crate; might be more reliable than std::thread::sleep
        } else {
            next_tick = now; // catch up after a slow frame
        }
        // Render frame
        screen.render(&gs, &textures)?;
        let texture = Texture2D::from_image(screen.buffer());
        draw_texture(&texture, 0., 0., WHITE);
        // FPS
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

        next_frame().await;
    }
}
