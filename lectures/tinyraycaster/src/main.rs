use macroquad::prelude::*;
use std::f32::consts::PI;

mod enemy;
mod map;
mod player;
mod screen;
mod textures;
mod tiles;

use crate::{map::Map, player::Player, screen::Screen, textures::Textures};

#[macroquad::main("tinyraycaster")]
async fn main() -> eyre::Result<()> {
    let mut map = Map::new("lectures/tinyraycaster/data/map.txt")?;
    map.with_state("lectures/tinyraycaster/data/state.tsv")?;

    let textures = Textures::new("lectures/tinyraycaster/data/textures.tsv", 64)?;
    let mut screen = Screen::<1024, 512>::new();

    // With initialization function.
    // TODO: Allow setting fov in degrees but internally use radians.
    let mut player = Player {
        x: 3.456,
        y: 2.345,
        ang: 1.523,
        fov: PI / 3.0,
    };

    loop {
        player.ang += 2.0 * PI / 360.0;
        // Render frame
        screen.render(&player, &map, &textures)?;

        let texture = Texture2D::from_image(screen.buffer());
        draw_texture(&texture, 0., 0., WHITE);
        next_frame().await
    }
}
