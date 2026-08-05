use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use rustc_hash::FxHashMap;

use crate::{
    enemy::{Enemy, EnemyState, EnemyType},
    map::Map,
    player::{Player, TurnState, WalkState},
    tiles::{Tile, TileState, TileType},
};
use eyre::bail;
use itertools::Itertools;
use macroquad::prelude::*;

#[derive(Default)]
pub struct GameState {
    // Current map
    pub map: Map,
    // Player
    pub player: Player,
    // Tiles
    // tile position to id
    pub tile_pos_id_map: FxHashMap<(usize, usize), usize>,
    pub id_tile_map: BTreeMap<usize, Tile>,
    // Enemy
    // enemy position to id
    pub enemy_pos_id_map: FxHashMap<(usize, usize), usize>,
    pub id_enemy_map: BTreeMap<usize, Enemy>,
}

impl GameState {
    pub fn new(map: &str, state: &str) -> eyre::Result<Self> {
        let mut game_state = GameState::default();
        let map = Map::new(map, &mut game_state)?;
        game_state.map = map;

        let state_fh = BufReader::new(File::open(state)?);
        for line in state_fh.lines() {
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
                        angle,
                        state,
                        typ: etyp,
                        dst: 0.0,
                    };
                    game_state.spawn_enemy(enemy);
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
                    game_state.spawn_tile(tile);
                }
                "player" => {}
                _ => bail!("Invalid type {typ} for {line}"),
            }
        }

        for enemy in game_state.id_enemy_map.values_mut() {
            enemy.dst = enemy.dst_from_player(&game_state.player);
        }
        Ok(game_state)
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tile_pos_id_map
            .get(&(x, y))
            .and_then(|id| self.id_tile_map.get(id))
    }

    pub fn get_tiles(&self) -> impl Iterator<Item = (usize, usize, Option<&Tile>)> {
        (0..self.map.h).flat_map(move |y| (0..self.map.w).map(move |x| (x, y, self.get_tile(x, y))))
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

    pub fn update_all(&mut self) {
        // Stop walking and turning
        if is_key_released(KeyCode::W)
            || is_key_released(KeyCode::A)
            || is_key_released(KeyCode::S)
            || is_key_released(KeyCode::D)
        {
            self.player.turn = TurnState::Stop;
            self.player.walk = WalkState::Stop;
        }
        if is_key_down(KeyCode::W) {
            self.player.walk = WalkState::Forward
        }
        if is_key_down(KeyCode::A) {
            self.player.turn = TurnState::Left
        }
        if is_key_down(KeyCode::S) {
            self.player.walk = WalkState::Reverse
        }
        if is_key_down(KeyCode::D) {
            self.player.turn = TurnState::Right
        }
        // Update player position
        self.player.update(&self.map);
        // Update enemies
        for enemy in self.id_enemy_map.values_mut() {
            enemy.dst = enemy.dst_from_player(&self.player)
        }
    }
}
