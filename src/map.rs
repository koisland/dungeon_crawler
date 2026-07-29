use eyre::bail;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    state::GameState,
    tiles::{Tile, TileState, TileType},
};

#[derive(Default)]
pub struct Map {
    pub src: String,
    pub w: usize,
    pub h: usize,
}

impl Map {
    pub fn new(infile: &str, state: &mut GameState) -> eyre::Result<Self> {
        let fh = BufReader::new(File::open(infile)?);
        let mut map = Map::default();

        let mut map_w: usize = 0;
        let mut map_h: usize = 0;
        for (h, line) in fh.lines().enumerate() {
            let line = line?;
            let line = line.trim();
            let w = line.len();

            // Add tiles as entities
            for (x, tile) in line.chars().enumerate() {
                let Ok(tile_typ) = TryInto::<TileType>::try_into(tile) else {
                    continue;
                };
                let tile = Tile {
                    x,
                    y: h,
                    state: TileState::Base,
                    typ: tile_typ,
                };

                let eid = state.id_tile_map.len();
                state.tile_pos_id_map.insert((x, h), eid);
                state.id_tile_map.insert(eid, tile);
            }

            map.src.push_str(line);
            // eprintln!("{line} ({w}, {h})");
            // Only check at end. Could also do here and give failing line
            map_w = w;
            // 0-index
            map_h = h + 1;
        }

        if map_w * map_h != map.src.len() {
            bail!("Map does not have uniform length and width");
        }
        map.w = map_w;
        map.h = map_h;
        Ok(map)
    }

    // pub fn update_src(x: usize, y: usize, c: char) -> eyre::Result<()> {
    //     todo!()
    // }

    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x > 0 && x < self.w as i32 && y > 0 && y < self.h as i32
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.src.as_bytes()[y * self.w + x] == b' '
    }
}
