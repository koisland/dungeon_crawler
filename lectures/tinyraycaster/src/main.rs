use macroquad::prelude::*;
use std::f32::consts::PI;

mod enemy;
mod map;
mod player;
mod screen;
mod state;
mod textures;
mod tiles;

use crate::{screen::Screen, state::GameState, textures::Textures};

#[macroquad::main("tinyraycaster")]
async fn main() -> eyre::Result<()> {
    let mut gs = GameState::new(
        "lectures/tinyraycaster/data/map.txt",
        "lectures/tinyraycaster/data/state.tsv",
    )?;
    let textures = Textures::new("lectures/tinyraycaster/data/textures.tsv", 64)?;
    let mut screen = Screen::<1024, 512>::new();

    loop {
        gs.player.ang += 2.0 * PI / 360.0;
        // Render frame
        screen.render(&gs, &textures)?;

        let texture = Texture2D::from_image(screen.buffer());
        draw_texture(&texture, 0., 0., WHITE);
        next_frame().await;
    }
}
