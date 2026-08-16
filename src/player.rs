use std::f32::consts::PI;

use crate::{
    map::Map,
    ray::{cast_ray, CollidableObject, RAY_INC},
};

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
    // Player visibility. Controls map and draw distance.
    pub visibility: f32,
}

impl Player {
    /// Turn player adjusting angle based on [TurnState] and speed.
    pub fn turn(&mut self) {
        self.angle += self.turn_speed * (self.turn as i32) as f32
    }

    /// Move player based on [WalkState] and speed. Also checks if in bound.
    pub fn walk(&mut self, map: &Map) {
        let walk_magn = (self.walk as i32) as f32;
        let angle = match self.walk {
            WalkState::Forward => self.angle,
            WalkState::Reverse => self.angle + PI,
        };
        // Acceleration is a fraction of the maximum walk speed.
        // Input increase/decrease this fraction. No input decreases and clamps at 0.0.
        let walk_speed = self.walk_speed * self.acc;
        let walk_dst = walk_magn * walk_speed;

        // Cast ray to check if movement valid
        // Stop if:
        // * hit tile
        // * Or distance traveled by ray is greater than walk distance
        let ray_hit = cast_ray(self.x, self.y, angle, map, |map, cx, cy, dst| {
            let exceeds_dst = dst.abs() > walk_dst.abs();
            let htile = map.get_tile_id(cx as usize, cy as usize);
            if let Some(htile_id) = htile {
                return (true, Some(CollidableObject::Tile(*htile_id)));
            }
            if exceeds_dst {
                return (true, None);
            };
            (false, None)
        });
        if ray_hit.dst != RAY_INC && ray_hit.obj.is_none() {
            // eprintln!("{:?}", (self.x, self.y, &ray_hit));
            // No collision and free to update
            self.x = ray_hit.cx;
            self.y = ray_hit.cy;
        }
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
            visibility: 5.0,
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
