use std::str::FromStr;

use eyre::bail;

use crate::player::Player;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum EnemyType {
    RedBlob,
    Demon,
}

impl From<EnemyType> for char {
    fn from(value: EnemyType) -> Self {
        match value {
            EnemyType::RedBlob => 'a',
            EnemyType::Demon => 'b',
        }
    }
}

impl FromStr for EnemyType {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "red_blob" => EnemyType::RedBlob,
            "demon" => EnemyType::Demon,
            _ => bail!("Invalid string for enemy type {s}"),
        })
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EnemyState {
    Base,
    Injured,
    Enraged,
    Dead,
}

impl FromStr for EnemyState {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "base" => EnemyState::Base,
            "injured" => EnemyState::Injured,
            "enraged" => EnemyState::Enraged,
            "dead" => EnemyState::Dead,
            _ => bail!("Invalid tile state {s}"),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Enemy {
    /// x position
    pub x: f32,
    /// y position
    pub y: f32,
    /// Angle of enemy
    // TODO: Determine if front or back texture based on player position
    #[allow(unused)]
    pub angle: f32,
    /// Current state
    pub state: EnemyState,
    /// texture id on sprite sheet
    pub typ: EnemyType,
    /// Distance from player
    pub dst: f32,
}

impl Enemy {
    pub fn dst_from_player(&self, player: &Player) -> f32 {
        // pythagorean theorem
        (player.x - self.x).powi(2) + (player.y - self.y).powi(2)
    }
}
