use std::{f32::consts::PI, path::PathBuf};

use crate::{map::Map, player::Player, screen::Screen, textures::Textures};

mod color;
mod enemy;
mod map;
mod player;
mod screen;
mod textures;
mod tiles;

fn main() -> eyre::Result<()> {
    // Parse map.
    let mut map = Map::new("lectures/tinyraycaster/data/map.txt")?;
    map.with_state("lectures/tinyraycaster/data/state.tsv")?;

    let textures = Textures::new("lectures/tinyraycaster/data/textures.tsv", 64)?;

    // With initialization function.
    let mut screen = Screen::<1024, 512>::new();
    // TODO: Allow setting fov in degrees but internally use radians.
    let mut player = Player {
        x: 3.456,
        y: 2.345,
        ang: 1.523,
        fov: PI / 3.0,
    };

    let outdir = PathBuf::from(".");
    for i in 0..1 {
        let fname = outdir.join(format!("{i}.ppm"));
        player.ang += 2.0 * PI / 360.0;
        // Render frame
        screen.render(&player, &map, &textures)?;
        // Before dumping to outfile.
        screen.dump(fname)?;
    }

    Ok(())
}
