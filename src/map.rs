use rustc_hash::{FxHashMap, FxHashSet};

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
    pub w: usize,
    pub h: usize,
    // Store only position to ids.
    // Then can query enemy/tile in gamestate
    pub tiles: FxHashMap<(usize, usize), usize>,
    // Multiple enemies can be on a single tile
    pub enemies: FxHashMap<(usize, usize), Vec<usize>>,
    /// Seen tiles
    pub visible: FxHashSet<(usize, usize)>,
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
                map.tiles.insert((x, h), eid);
                state.id_tile_map.insert(eid, tile);
            }

            map_w = w;
            // 0-index
            map_h = h + 1;
        }

        map.w = map_w;
        map.h = map_h;
        Ok(map)
    }

    pub fn get_tile_id(&self, x: usize, y: usize) -> Option<&usize> {
        self.tiles.get(&(x, y))
    }

    #[allow(unused)]
    pub fn get_tile_ids(&self) -> impl Iterator<Item = (usize, usize, Option<&usize>)> {
        (0..self.h).flat_map(move |y| (0..self.w).map(move |x| (x, y, self.get_tile_id(x, y))))
    }
}
