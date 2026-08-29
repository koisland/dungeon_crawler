use std::str::FromStr;

use eyre::bail;

#[derive(Debug, Clone)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    /// State
    pub state: TileState,
    pub lbl: String,
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
