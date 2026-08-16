use crate::map::Map;

pub const RAY_INC: f32 = 0.01;

/// Collidable object ID
#[derive(Debug)]
pub enum CollidableObject {
    Tile(usize),
}

#[derive(Debug)]
pub struct RayHit {
    // x coord hit by ray
    pub cx: f32,
    // y coord hit by ray
    pub cy: f32,
    // Distance from player to hit tile
    pub dst: f32,
    // Object hit by ray
    pub obj: Option<CollidableObject>,
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
pub fn cast_ray(
    x: f32,
    y: f32,
    ang: f32,
    map: &Map,
    f_gs: impl Fn(&Map, f32, f32, f32) -> (bool, Option<CollidableObject>),
) -> RayHit {
    let mut dst: f32 = 0.0;
    let cos_ang = ang.cos();
    let sin_ang = ang.sin();
    loop {
        let cx = x + dst * cos_ang;
        let cy = y + dst * sin_ang;

        let (stop, obj) = f_gs(map, cx, cy, dst);
        if stop {
            return RayHit { cx, cy, dst, obj };
        };

        dst += RAY_INC
    }
}
