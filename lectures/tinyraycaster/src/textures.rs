use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use eyre::{bail, ContextCompat};
use itertools::Itertools;
use macroquad::{color::Color, math::Rect, prelude::ImageFormat, texture::Image};

use crate::{
    enemy::{Enemy, EnemyState, EnemyType},
    tiles::{Tile, TileState, TileType},
};

#[derive(Debug, Clone)]
pub enum Texture {
    /// Single color
    Color(Color),
    /// Sprite
    Sprite(Image),
}
impl Texture {
    pub fn get_color(&self, x: usize, y: usize) -> Option<Color> {
        match self {
            Texture::Color(color) => Some(*color),
            Texture::Sprite(image) => Some(image.get_pixel(x as u32, y as u32)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Textures {
    // Size of texture square length and width.
    pub size: usize,
    // Textures mapped to map tiles and state
    pub tiles: HashMap<TileType, HashMap<TileState, Texture>>,
    // Textures mapped to map entities and state
    pub entities: HashMap<EnemyType, HashMap<EnemyState, Texture>>,
}

impl Textures {
    pub fn get_enemy(&self, enemy: &Enemy) -> Option<&Texture> {
        self.entities
            .get(&enemy.typ)
            .and_then(|enemy_states| enemy_states.get(&enemy.state))
    }

    pub fn get_tile(&self, tile: &Tile) -> Option<&Texture> {
        self.tiles
            .get(&tile.typ)
            .and_then(|tilestates| tilestates.get(&tile.state))
    }

    pub fn new(infile: &str, size: usize) -> eyre::Result<Self> {
        let mut textures = Textures {
            size,
            tiles: HashMap::new(),
            entities: HashMap::new(),
        };
        let fh = BufReader::new(File::open(infile)?);
        let mut texture_cache = HashMap::new();

        for line in fh.lines() {
            let line = line?;
            // Skip comments or header
            if line.starts_with('#') {
                continue;
            }

            let Some((typ, lbl, state, src, idx)) = line.trim().split('\t').collect_tuple() else {
                bail!("Invalid line format for {line}. Expects two columns, tilechar and path/rgb")
            };

            // Check if RGB color or image texture
            let texture = if src.contains(',') {
                let (r, g, b) = src
                    .split(',')
                    .map(|v| v.parse::<u8>())
                    .collect_tuple()
                    .with_context(|| format!("Invalid rgb format for {src}"))?;
                Texture::Color(Color::from_rgba(r?, g?, b?, 0))
            } else {
                let path = std::path::PathBuf::from(src);
                if !path.exists() {
                    bail!("Invalid src ({src}) for {line}.");
                }

                let img = if let Some(img) = texture_cache.get(src) {
                    img
                } else {
                    let img_data = std::fs::read(src)?;
                    let image = Image::from_file_with_format(&img_data, Some(ImageFormat::Png))?;
                    texture_cache.insert(src.to_owned(), image);
                    // Already inserted.
                    texture_cache.get(src).unwrap()
                };

                let w = img.width();
                let idx: usize = idx.parse()?;

                // Assuming square.
                let ncols = w / textures.size;
                // Convert idx to pixel coordinates
                let div = idx / ncols;
                let rem = idx % ncols;
                let x = (rem * textures.size) as u32;
                let y = (div * textures.size) as u32;
                // Then slice image out of sprite sheet
                let img_slice = img.sub_image(Rect::new(
                    x as f32,
                    y as f32,
                    textures.size as f32,
                    textures.size as f32,
                ));
                Texture::Sprite(img_slice)
            };

            match typ {
                "tile" => {
                    let tiletype: TileType = TileType::from_str(lbl)?;
                    let texture_state = TileState::from_str(state)?;

                    if let Some(tile_src) = textures.tiles.get_mut(&tiletype) {
                        tile_src.insert(texture_state, texture);
                    } else {
                        textures
                            .tiles
                            .insert(tiletype, HashMap::from_iter([(texture_state, texture)]));
                    }
                }
                "enemy" => {
                    let enemy: EnemyType = EnemyType::from_str(lbl)?;
                    let texture_state = EnemyState::from_str(state)?;
                    if let Some(ety_src) = textures.entities.get_mut(&enemy) {
                        ety_src.insert(texture_state, texture);
                    } else {
                        textures
                            .entities
                            .insert(enemy, HashMap::from_iter([(texture_state, texture)]));
                    }
                }
                _ => bail!("Invalid type. {typ}"),
            };
        }

        Ok(textures)
    }
}
