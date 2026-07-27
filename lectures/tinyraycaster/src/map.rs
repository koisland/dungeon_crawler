use eyre::bail;
use itertools::Itertools;

use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use crate::{
    enemy::{Enemy, EnemyState, EnemyType},
    tiles::{Tile, TileState, TileType},
};

pub struct Map {
    pub src: String,
    pub w: usize,
    pub h: usize,
    // tile position to id
    pub tile_pos_id_map: HashMap<(usize, usize), usize>,
    pub id_tile_map: BTreeMap<usize, Tile>,
    // enemy position to id
    pub enemy_pos_id_map: HashMap<(usize, usize), usize>,
    pub id_enemy_map: BTreeMap<usize, Enemy>,
}

impl Map {
    pub fn new(infile: &str) -> eyre::Result<Self> {
        let fh = BufReader::new(File::open(infile)?);
        let mut map = Map {
            src: String::new(),
            w: 0,
            h: 0,
            enemy_pos_id_map: HashMap::default(),
            id_enemy_map: BTreeMap::default(),
            tile_pos_id_map: HashMap::default(),
            id_tile_map: BTreeMap::default(),
        };

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

                let eid = map.id_tile_map.len();
                map.tile_pos_id_map.insert((x, h), eid);
                map.id_tile_map.insert(eid, tile);
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

    pub fn with_state(&mut self, infile: &str) -> eyre::Result<()> {
        let fh = BufReader::new(File::open(infile)?);

        for line in fh.lines() {
            let line = line?;
            if line.starts_with('#') {
                continue;
            }
            let Some((typ, lbl, state, x, y, angle)) = line.trim().split('\t').collect_tuple()
            else {
                bail!("Invalid format for state for {line}")
            };
            let x = x.parse::<f32>()?;
            let y = y.parse::<f32>()?;
            let angle = (!angle.is_empty())
                .then(|| angle.parse::<f32>())
                .transpose()?
                .unwrap_or_default();

            match typ {
                "enemy" => {
                    let etyp = EnemyType::from_str(lbl)?;
                    let state = EnemyState::from_str(state)?;

                    let enemy = Enemy {
                        x,
                        y,
                        _angle: angle,
                        state,
                        typ: etyp,
                    };
                    self.spawn_enemy(enemy);
                }
                "tile" => {
                    let tiletype = TileType::from_str(lbl)?;
                    let state = TileState::from_str(state)?;

                    let tile = Tile {
                        x: x as usize,
                        y: y as usize,
                        state,
                        typ: tiletype,
                    };

                    self.spawn_tile(tile);
                }
                _ => bail!("Invalid type {typ} for {line}"),
            }
        }

        Ok(())
    }

    pub fn tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tile_pos_id_map
            .get(&(x, y))
            .and_then(|id| self.id_tile_map.get(id))
    }

    pub fn tiles(&self) -> impl Iterator<Item = (usize, usize, Option<&Tile>)> {
        (0..self.h).flat_map(move |y| (0..self.w).map(move |x| (x, y, self.tile(x, y))))
    }

    pub fn spawn_tile(&mut self, tile: Tile) {
        let tid = self.id_tile_map.len();
        self.tile_pos_id_map.insert((tile.x, tile.y), tid);
        self.id_tile_map.insert(tid, tile);
    }

    pub fn spawn_enemy(&mut self, enemy: Enemy) {
        let eid = self.id_enemy_map.len();
        // TODO: This should change
        self.enemy_pos_id_map
            .insert((enemy.x as usize, enemy.y as usize), eid);
        self.id_enemy_map.insert(eid, enemy);
    }

    // pub fn get_entity_by_id(&mut self, id: usize) -> Option<&mut Box<dyn Entity + 'static>> {
    //     self.entities.get_mut(&id)
    // }
}
