use std::str::FromStr;

use eyre::bail;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TileType {
    Rock,
    Dirt,
    RedBrick,
    Door,
    SolidRock,
    Brick,
}

impl TryFrom<char> for TileType {
    type Error = eyre::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '0' => TileType::Rock,
            '1' => TileType::Dirt,
            '2' => TileType::RedBrick,
            '3' => TileType::Door,
            '4' => TileType::SolidRock,
            '5' => TileType::Brick,
            _ => bail!("Invalid tile type. {value}"),
        })
    }
}

impl From<TileType> for char {
    fn from(value: TileType) -> Self {
        match value {
            TileType::Rock => '0',
            TileType::Dirt => '1',
            TileType::RedBrick => '2',
            TileType::Door => '3',
            TileType::SolidRock => '4',
            TileType::Brick => '5',
        }
    }
}

impl FromStr for TileType {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "rock" => TileType::Rock,
            "dirt" => TileType::Dirt,
            "red_brick" => TileType::RedBrick,
            "door" => TileType::Door,
            "solid_rock" => TileType::SolidRock,
            "brick" => TileType::Brick,
            _ => bail!("Invalid tile type {s}"),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    /// State
    pub state: TileState,
    /// Type of tile
    pub typ: TileType,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TileState {
    Base,
}

impl FromStr for TileState {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "base" => TileState::Base,
            _ => bail!("Invalid tile state {s}"),
        })
    }
}
