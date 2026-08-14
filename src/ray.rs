use crate::{state::GameState, tiles::Tile};

pub struct RayHit {
    // x coord hit by ray
    pub cx: f32,
    // y coord hit by ray
    pub cy: f32,
    // Distance from player to hit tile
    pub dst: f32,
}

/// # Drawing a ray.
/// Our diagram of the player in space looks like this.
/// ```no_run
///  a
/// ___
/// \p |
///  \ | b
/// c \|
///    (x, y)
/// ```
///
/// Remember soh-cah-toa? This allows us to calculate `x` and `y` from `p_angle` (`ang`).
/// * `cos(p_angle) = a/c` which also is `a = c * cos(p_angle)`
/// * `sin(p_angle) = b/c` which also is `b = c * cos(p_angle)`
///
/// So:
/// * `x` and `y` is the endpoint of the ray (hypotenuse of c) along the triangle.
/// * `c` is some arbitrary value representing the distance from object hit by ray
///
/// Thus:
/// * `x = p_x + c * cos(p_angle)`
/// * `y = p_y + c * sin(p_angle)`
///
/// This function returns the distance (length of c) to the endpoint of the ray.
pub fn cast_ray_to_tile(x: f32, y: f32, ang: f32, gs: &GameState) -> eyre::Result<(&Tile, RayHit)> {
    // We don't include a limit (20) unlike the src
    const INC: f32 = 0.01;
    let mut dst: f32 = 0.0;
    let cos_ang = ang.cos();
    let sin_ang = ang.sin();
    loop {
        let cx = x + dst * cos_ang;
        let cy = y + dst * sin_ang;

        // Out of bounds or hit an object
        if let Some(htile) = gs.get_tile(cx as usize, cy as usize) {
            return Ok((htile, RayHit { cx, cy, dst }));
        };

        dst += INC
    }
}
