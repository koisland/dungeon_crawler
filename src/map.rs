use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    state::GameState,
    tiles::{Tile, TileState},
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

            let Some((x, y, lbl, _icon)) = line.split('\t').collect_tuple() else {
                continue;
            };
            let x = x.parse()?;
            let y = y.parse()?;
            let tile = Tile {
                x,
                y,
                lbl: lbl.to_owned(),
                state: TileState::Base,
            };
            let eid = state.id_tile_map.len();
            map.tiles.insert((x, h), eid);
            state.id_tile_map.insert(eid, tile);

            map_w = std::cmp::max(map_w, x);
            map_h = std::cmp::max(map_h, y);
        }

        map.w = map_w + 1;
        map.h = map_h + 1;
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
