use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use eyre::bail;
use itertools::Itertools;
use macroquad::prelude::*;

use crate::{
    enemy::Enemy,
    player::Player,
    state::GameState,
    textures::{Texture, Textures},
    tiles::Tile,
};

pub struct RayHit {
    // x coord hit by ray
    cx: f32,
    // y coord hit by ray
    cy: f32,
    // Distance from player to hit tile
    dst: f32,
}

// Store image in 1D array.
// Access elems by specify w + (h * WIDTH)
pub struct Screen {
    pub buffer: Image,
    pub texture: Texture2D,
    pub map_buffer: Image,
    pub map_texture: Texture2D,
    pub depth_buffer: Vec<f32>,
}

pub fn draw_pixel(image: &mut Image, x: usize, y: usize, color: Color) {
    if x >= image.width() || y >= image.height() {
        return;
    }
    image.set_pixel(x as u32, y as u32, color);
}

pub fn draw_rect(image: &mut Image, x: usize, y: usize, w: usize, h: usize, color: Color) {
    // Loop thru length and width adding px by px.
    for i in 0..w {
        for j in 0..h {
            let cx = x + i;
            let cy = y + j;
            draw_pixel(image, cx, cy, color);
        }
    }
}

pub fn draw_image(buffer: &mut Image, x: usize, y: usize, image: &Image) {
    let (w, h) = (buffer.width(), buffer.height());
    let (img_w, img_h) = (image.width(), image.height());
    for i in 0..img_w {
        for j in 0..img_h {
            let cx = x + i;
            let cy = y + j;
            if cx > w || cy > h {
                continue;
            }
            let px = image.get_pixel(i as u32, j as u32);
            draw_pixel(buffer, cx, cy, px)
        }
    }
}

// Based on https://kwojcicki.github.io/blog/NEAREST-NEIGHBOUR
fn nearest_neighbor(image: &Image, new_w: u16, new_h: u16) -> Image {
    let scale_w = image.width() as f32 / new_w as f32;
    let scale_h = image.height() as f32 / new_h as f32;

    let mut new_image = Image::gen_image_color(new_w, new_h, WHITE);
    for y in 0..new_image.height() {
        for x in 0..new_image.width() {
            // Scale and find nearest pixel in original image
            let proj_x = (x as f32 * scale_w).floor() as u32;
            let proj_y = (y as f32 * scale_h).floor() as u32;
            let px = image.get_pixel(proj_x, proj_y);
            new_image.set_pixel(x as u32, y as u32, px);
        }
    }
    new_image
}

impl Screen {
    pub fn new(w: usize, h: usize, mw: usize, mh: usize) -> Self {
        let buffer = Image::gen_image_color(w as u16, h as u16, WHITE);
        let map_buffer = Image::gen_image_color(mw as u16, mh as u16, WHITE);
        let texture = Texture2D::from_image(&buffer);
        let map_texture = Texture2D::from_image(&map_buffer);
        let depth_buffer = vec![1e3; w];
        Screen {
            buffer,
            map_buffer,
            depth_buffer,
            texture,
            map_texture,
        }
    }

    pub fn clear(&mut self) {
        let (w, h) = (self.buffer.width, self.buffer.height);
        let (mw, mh) = (self.map_buffer.width, self.map_buffer.height);
        self.buffer = Image::gen_image_color(w, h, WHITE);
        self.map_buffer = Image::gen_image_color(mw, mh, WHITE);
    }

    /// Write PPM file.
    /// https://netpbm.sourceforge.net/doc/ppm.html
    #[allow(unused)]
    pub fn dump(&self, fname: impl AsRef<Path>) -> eyre::Result<()> {
        let (w, h) = (self.buffer.width(), self.buffer.height());
        // Check images is correct size as given width and height.
        let mut fh = BufWriter::new(File::create(fname)?);
        // Write magic number identifying file type, w, h, max color value. All delimited by newline.
        write!(fh, "P3\n{} {}\n255\n", w, h)?;

        const END_CHAR: [&str; 2] = ["\n", " "];

        for (i, px) in self.buffer.get_image_data().iter().enumerate() {
            let [r, g, b, _] = px;
            // Place end char so after each rgb triplet, properly spaced.
            let end_char = END_CHAR[usize::from(i % w != 0)];
            write!(fh, "{r} {g} {b}{end_char}")?;
        }
        Ok(())
    }

    // TODO: Maybe move to Map.
    pub fn draw_map(&mut self, gs: &GameState, textures: &Textures) -> eyre::Result<()> {
        let rect_w = self.map_buffer.width() / gs.map.w;
        let rect_h = self.map_buffer.height() / gs.map.h;

        for (x, y, tile) in gs.get_tiles() {
            if let Some(tile) = tile {
                // Because each rect is w and h
                let rect_x = x * rect_w;
                let rect_y = y * rect_h;
                let Some(texture) = textures.get_tile(tile) else {
                    bail!("Tile {tile:?} has no texture.")
                };

                // eprintln!("At ({x},{y}) draw {tile:?} tile at ({rect_x}, {rect_y}) ");
                match texture {
                    Texture::Color(color) => {
                        draw_rect(&mut self.map_buffer, rect_x, rect_y, rect_w, rect_h, *color);
                    }
                    Texture::Sprite(img) => {
                        // Draw thumbnail
                        let img_thumbnail = nearest_neighbor(img, rect_w as u16, rect_h as u16);
                        draw_image(&mut self.map_buffer, rect_x, rect_y, &img_thumbnail);
                    }
                };
            }
            continue;
        }
        self.draw_player_on_map(gs)?;
        self.draw_entities_on_map(gs)?;
        Ok(())
    }

    pub fn draw_player_on_map(&mut self, gs: &GameState) -> eyre::Result<()> {
        let rect_w = self.map_buffer.width() / gs.map.w;
        let rect_h = self.map_buffer.height() / gs.map.h;
        // Convert from coordinates to image dim
        let x = (gs.player.x * rect_w as f32) as usize;
        let y = (gs.player.y * rect_h as f32) as usize;
        draw_rect(
            &mut self.map_buffer,
            x,
            y,
            5,
            5,
            Color::from_rgba(0, 0, 0, 255),
        );
        Ok(())
    }

    pub fn draw_entities_on_map(&mut self, gs: &GameState) -> eyre::Result<()> {
        let rect_w = self.map_buffer.width() / gs.map.w;
        let rect_h = self.map_buffer.height() / gs.map.h;

        for entity in gs.id_enemy_map.values() {
            let x = (entity.x * rect_w as f32) as usize;
            let y = (entity.y * rect_h as f32) as usize;
            draw_rect(
                &mut self.map_buffer,
                x,
                y,
                5,
                5,
                Color::from_rgba(255, 0, 0, 255),
            );
        }
        Ok(())
    }

    pub fn draw_sprite(
        &mut self,
        enemy: &Enemy,
        player: &Player,
        textures: &Textures,
    ) -> eyre::Result<()> {
        let (w, h) = (self.buffer.width(), self.buffer.height());
        // https://www.youtube.com/watch?v=VMYk9fqXz_4
        // https://stackoverflow.com/questions/283406/what-is-the-difference-between-atan-and-atan2-in-c
        // Use atan2 incase where x is negative. Allows getting angle with range across all 4 quadrants as opposed to 2 (1 and 4).
        // Angle of enemy relative to player
        let sprite_x = enemy.x - player.x;
        let sprite_y = enemy.y - player.y;
        let (dir_x, dir_y, plane_x, plane_y) = player.camera_info();

        // inverse camera matrix
        let inv_det = 1.0 / (plane_x * dir_y - dir_x * plane_y);
        let transform_x = inv_det * (dir_y * sprite_x - dir_x * sprite_y);
        let transform_y = inv_det * (-plane_y * sprite_x + plane_x * sprite_y);

        let sprite_screen_x = ((w as f32 / 2.0) * (1.0 + transform_x / transform_y)) as i32;
        let sprite_height = (h as f32 / transform_y).abs() as i32;
        let sprite_width = sprite_height;

        let texture_size = textures.size as i32;
        let (wi, hi) = (w as i32, h as i32);
        let draw_start_x = -sprite_width / 2 + sprite_screen_x;
        let draw_end_x = (sprite_width / 2 + sprite_screen_x).min(wi - 1);
        let draw_start_y = -sprite_height / 2 + hi / 2;
        let draw_end_y = (sprite_height / 2 + hi / 2).min(hi - 1);

        let Some(texture) = textures.get_enemy(enemy) else {
            bail!("No texture for enemy {enemy:?}")
        };

        for x in draw_start_x..draw_end_x {
            // the conditions in the if are:
            // 1) it's in front of camera plane so you don't see things behind you
            // 2) it's on the screen (left)
            // 3) it's on the screen (right)
            // 4) ZBuffer, with perpendicular distance
            if !(TryInto::<usize>::try_into(x).is_ok_and(|x| transform_y < self.depth_buffer[x])
                && transform_y > 0.0
                && x > 0
                && x < wi)
            {
                continue;
            }

            let tx_x = (x - draw_start_x) * texture_size / sprite_width;

            for y in draw_start_y..draw_end_y {
                let tx_y = (y - draw_start_y) * texture_size / sprite_height;
                let Some(color) = texture.get_color(tx_x as usize, tx_y as usize) else {
                    bail!("No color.")
                };
                // Only draw opaque pixels
                // https://colorlabs.net/posts/what-are-alpha-channels-in-digital-images
                if color.a > 0.5 {
                    draw_pixel(&mut self.buffer, x as usize, y as usize, color)
                }
            }
        }
        Ok(())
    }

    pub fn draw_sprites(&mut self, gs: &GameState, textures: &Textures) -> eyre::Result<()> {
        // Brute-force draw enemies from farthest to closest
        for entity in gs
            .id_enemy_map
            .values()
            .sorted_by(|a, b| a.dst.total_cmp(&b.dst))
            .rev()
        {
            self.draw_sprite(entity, &gs.player, textures)?;
        }
        Ok(())
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
    pub fn draw_ray<'a>(
        &mut self,
        x: f32,
        y: f32,
        ang: f32,
        gs: &'a GameState,
    ) -> eyre::Result<(&'a Tile, RayHit)> {
        let (w, h) = (self.map_buffer.width(), self.map_buffer.height());
        let rect_w = (w / gs.map.w) as f32;
        let rect_h = (h / gs.map.h) as f32;

        // We don't include a limit (20) unlike the src
        const INC: f32 = 0.01;
        let mut dst: f32 = 0.0;
        loop {
            let cx = x + dst * ang.cos();
            let cy = y + dst * ang.sin();

            // Otherwise, draw ray
            let px_x = cx * rect_w;
            let px_y = cy * rect_h;
            draw_rect(
                &mut self.map_buffer,
                px_x as usize,
                px_y as usize,
                1,
                1,
                Color::from_rgba(190, 190, 190, 255),
            );

            // Out of bounds or hit an object
            if let Some(htile) = gs.get_tile(cx as usize, cy as usize) {
                return Ok((htile, RayHit { cx, cy, dst }));
            };

            dst += INC
        }
    }

    // pub fn draw_floor_ceiling(
    //     &mut self,

    //     gs: &GameState,
    //     textures: &Textures,
    // ) -> eyre::Result<()> {

    //     Ok(())
    // }

    /// # Generate the field-of-view of the player
    /// ```no_run
    ///    ------
    ///   /2\2 \1
    /// f/   \d \
    /// ```
    ///
    /// * 1 is the angle between x-axis and fov angle
    /// * 2 is the fov angle centered on the player's direction angle
    /// * Add both together to calculate the fov
    ///
    /// We iterate over the width because it is the hypotenuse of the FOV tri/cone.
    ///
    /// # To adjust for fisheye distortion:
    /// See https://gamedev.stackexchange.com/a/97580 for diagram.
    /// * Because the height of the walls are determined based on distance, distant rays in the fov are longer and create shorter walls.
    /// * We need to take the range instead of the distance to determine wall height.
    ///
    /// # Drawing textures
    /// In order to draw the texture, we have to know where on the tile was hit.
    /// * It could be on the horizontal (hitx) or vertical (hity)
    /// * They contain (signed) fractional parts of cx and cy (endpoint coordinates of the ray) from 0.5 to -0.5
    /// * The large magnitude indicates that it is the one hit. We can get the coordinate in sprite space as a result.
    /// * Then we draw it.
    pub fn draw_fov(&mut self, gs: &GameState, textures: &Textures) -> eyre::Result<()> {
        let (w, h) = (self.buffer.width(), self.buffer.height());
        let fw: f32 = w as f32;

        // To convert an angle in radians to a vector
        // https://math.stackexchange.com/a/295827
        // Someone also noted how inconvenient the lodev impl was and figured a better way lol
        // https://github.com/almushel/raycast-demo#setting-the-camera-direction
        let (dir_x, dir_y, plane_x, plane_y) = gs.player.camera_info();
        let ray_dir_x0 = dir_x - plane_x;
        let ray_dir_y0 = dir_y - plane_y;
        let ray_dir_x1 = dir_x + plane_x;
        let ray_dir_y1 = dir_y + plane_y;

        let h_i = h as i32;
        let texture_size = textures.size as f32;
        for y in h_i / 2 + 1..h_i {
            let p = y - h_i / 2;
            let pos_z = 0.5 * h_i as f32;
            let row_dst = pos_z / p as f32;
            let floor_step_x = row_dst * (ray_dir_x1 - ray_dir_x0) / w as f32;
            let floor_step_y = row_dst * (ray_dir_y1 - ray_dir_y0) / w as f32;

            let mut floor_x = gs.player.x + row_dst * ray_dir_x0;
            let mut floor_y = gs.player.y + row_dst * ray_dir_y0;
            for x in 0..w {
                let cell_x = floor_x as usize;
                let cell_y = floor_y as usize;

                let tx = ((texture_size * (floor_x - cell_x as f32).clamp(0.0, 1.0)) as usize)
                    .clamp(0, textures.size - 1);
                let ty = ((texture_size * (floor_y - cell_y as f32).clamp(0.0, 1.0)) as usize)
                    .clamp(0, textures.size - 1);

                floor_x += floor_step_x;
                floor_y += floor_step_y;

                let floor_texture = &textures.floor;
                let Some(floor_color) = floor_texture.get_color(tx, ty) else {
                    bail!("No texture for floor ({tx}, {ty})")
                };
                draw_pixel(&mut self.buffer, x, y as usize, floor_color);

                let ceiling_texture = &textures.ceiling;
                let Some(ceilng_color) = ceiling_texture.get_color(tx, ty) else {
                    bail!("No texture for ceiling ({tx}, {ty})")
                };
                draw_pixel(&mut self.buffer, x, (h_i - y - 1) as usize, ceilng_color);
            }
        }

        // Angle between x-axis and fov
        // Draw at every angle within FOV
        for col_x in 0..w {
            // The rest of the FOV angle drawn section by section.
            let camera_x = 2.0 * col_x as f32 / fw - 1.0;
            let ray_dir_x = dir_x + plane_x * camera_x;
            let ray_dir_y = dir_y + plane_y * camera_x;
            let angle = ray_dir_y.atan2(ray_dir_x);

            // Draw floor and ceiling
            let (htile, ray_hit) = self.draw_ray(gs.player.x, gs.player.y, angle, gs)?;

            let Some(texture) = &textures.get_tile(htile) else {
                bail!("No texture for hit tile {htile:?}")
            };
            // Closer means smaller c and thus large ht.
            // We need to adjust this scaling to avoid fisheye distortion due to the ray hitting at multiple angles
            // See https://gamedev.stackexchange.com/a/97580
            // And https://lodev.org/cgtutor/raycasting.html
            let col_ht = (h as f32 / (ray_hit.dst * (angle - gs.player.angle).cos())) as usize;

            // Store distance to know what to occlude from fov
            self.depth_buffer[col_x] = ray_hit.dst;

            // Draw texture/tile
            match texture {
                Texture::Color(color) => {
                    // Start at middle of screen and then drop y by half the col ht. This centers the drawn line.
                    let col_y = h / 2 - col_ht / 2;
                    draw_rect(&mut self.buffer, col_x, col_y, 1, col_ht, *color);
                }
                Texture::Sprite(sprite) => {
                    let size = sprite.height() as f32;
                    // We need to know whether we hit the x or y side of the texture.
                    // hitx and hity contain (signed) fractional parts of cx and cy from 0.5 to -0.5
                    // If hity (fractional part of y) magnitude larger, then "vertical" part of tile hit.
                    //  hitx             hity
                    //  *
                    // ______              ______
                    // |    |            * |    |
                    // |____|              |____|
                    let hitx = ray_hit.cx - (ray_hit.cx + 0.5).floor();
                    let hity = ray_hit.cy - (ray_hit.cy + 0.5).floor();
                    // Once know part of texture was hit, we can get what part of sprite to render from the size and fraction.
                    let mut x_texcoord = if hity.abs() > hitx.abs() {
                        hity * size
                    } else {
                        hitx * size
                    };
                    if x_texcoord < 0.0 {
                        x_texcoord += size
                    }
                    assert!(x_texcoord >= 0.0 && x_texcoord < size);

                    // Scale column to height.
                    let mut texcol = Vec::with_capacity(col_ht);
                    for y in 0..col_ht {
                        let pix_y = (y as f32 * size) / col_ht as f32;
                        texcol.push(sprite.get_pixel(x_texcoord as u32, pix_y as u32));
                    }
                    // Write scaled column
                    for (j, px) in texcol.into_iter().enumerate().take(col_ht) {
                        // Start at middle of screen ht, then half of the column ht. Add pixels to reach col_ht again.
                        let Some(pix_y) = (j + (h / 2)).checked_sub(col_ht / 2) else {
                            continue;
                        };
                        draw_pixel(&mut self.buffer, col_x, pix_y, px)
                    }
                }
            }
        }
        Ok(())
    }

    pub fn render(&mut self, gs: &GameState, textures: &Textures) -> eyre::Result<()> {
        // Clear buffer
        self.clear();
        // Draw fov for player
        self.draw_fov(gs, textures)?;
        // And draw sprites
        self.draw_sprites(gs, textures)?;

        // Then update
        self.texture.update(&self.buffer);
        draw_texture(&self.texture, 0., 0., WHITE);

        // Draw map
        self.draw_map(gs, textures)?;
        self.map_texture.update(&self.map_buffer);
        let map_x = self.buffer.width() - self.map_buffer.width();
        let map_y = self.buffer.height() - self.map_buffer.height();
        draw_texture(&self.map_texture, map_x as f32, map_y as f32, GRAY);

        Ok(())
    }
}
