use std::f32::consts::PI;

use crate::map::Map;

const MIN_ACC: f32 = 0.0;
const MAX_ACC: f32 = 1.0;

/// Player
pub struct Player {
    /// Player x coordinate
    pub x: f32,
    /// Player y coordinate
    pub y: f32,
    /// Player view direction in radians
    /// the angle between the view direction and the x axis
    pub angle: f32,
    /// Player fov in radians where midpoint is self.direction
    pub fov: f32,
    /// If walking
    pub walk: WalkState,
    pub walk_speed: f32,
    // Acceleration between (0.0 - 1.0)
    pub acc: f32,
    // If turning
    pub turn: TurnState,
    pub turn_speed: f32,
}

impl Player {
    /// Turn player adjusting angle based on [TurnState] and speed.
    pub fn turn(&mut self) {
        self.angle += self.turn_speed * (self.turn as i32) as f32
    }

    /// Move player based on [WalkState] and speed. Also checks if in bound.
    pub fn walk(&mut self, map: &Map) {
        let walk_magn = (self.walk as i32) as f32;
        // Acceleration is a fraction of the maximum walk speed.
        // Input increase/decrease this fraction. No input decreases and clamps at 0.0.
        let walk_speed = self.walk_speed * self.acc;
        let dt_x = walk_magn * self.angle.cos() * walk_speed;
        let dt_y = walk_magn * self.angle.sin() * walk_speed;

        let new_x = self.x + dt_x;
        let new_y = self.y + dt_y;

        if map.is_in_bounds(new_x as i32, new_y as i32) {
            // Move if empty space
            if map.is_empty(new_x as usize, self.y as usize) {
                self.x = new_x;
            }
            if map.is_empty(self.x as usize, new_y as usize) {
                self.y = new_y;
            }
        }
    }

    pub fn update(&mut self, map: &Map) {
        self.turn();
        self.walk(map);
    }

    pub fn accelerate(&mut self, dt: f32) {
        self.acc = (self.acc + dt).clamp(MIN_ACC, MAX_ACC)
    }

    pub(crate) fn camera_info(&self) -> (f32, f32, f32, f32) {
        let dir_x = self.angle.cos();
        let dir_y = self.angle.sin();
        let (plane_x, plane_y) = {
            let angle = PI / 2.;
            (
                (dir_x * angle.cos() - dir_y * angle.sin()) * self.fov,
                (dir_x * angle.sin() + dir_y * angle.cos()) * self.fov,
            )
        };
        (dir_x, dir_y, plane_x, plane_y)
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: 3.456,
            y: 2.345,
            angle: 1.523,
            fov: PI / 3.0,
            walk: WalkState::Forward,
            walk_speed: 0.06,
            acc: 0.0,
            turn: TurnState::Stop,
            turn_speed: 0.05,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WalkState {
    Forward = 1,
    Reverse = -1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TurnState {
    Left = -1,
    Stop = 0,
    Right = 1,
}
