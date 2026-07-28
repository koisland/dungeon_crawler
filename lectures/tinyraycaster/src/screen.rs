use std::{
    f32::consts::PI,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use eyre::bail;
use itertools::Itertools;
use macroquad::{
    color::{Color, WHITE},
    texture::Image,
};

use crate::{
    enemy::Enemy,
    player::Player,
    state::GameState,
    textures::{Texture, Textures},
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
pub struct Screen<const W: usize, const H: usize> {
    buffer: Image,
    depth_buffer: Vec<f32>,
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

impl<const W: usize, const H: usize> Screen<W, H> {
    pub fn new() -> Self {
        let buffer = Image::gen_image_color(W as u16, H as u16, WHITE);

        let depth_buffer = vec![1e3; W / 2];
        Screen::<W, H> {
            buffer,
            depth_buffer,
        }
    }

    pub fn buffer(&self) -> &Image {
        &self.buffer
    }

    pub fn clear(&mut self) {
        self.buffer = Image::gen_image_color(W as u16, H as u16, WHITE)
    }

    /// Write PPM file.
    /// https://netpbm.sourceforge.net/doc/ppm.html
    #[allow(unused)]
    pub fn dump(&self, fname: impl AsRef<Path>) -> eyre::Result<()> {
        // Check images is correct size as given width and height.
        let mut fh = BufWriter::new(File::create(fname)?);
        // Write magic number identifying file type, w, h, max color value. All delimited by newline.
        write!(fh, "P3\n{W} {H}\n255\n")?;

        const END_CHAR: [&str; 2] = ["\n", " "];

        for (i, px) in self.buffer.get_image_data().iter().enumerate() {
            let [r, g, b, _] = px;
            // Place end char so after each rgb triplet, properly spaced.
            let end_char = END_CHAR[usize::from(i % W != 0)];
            write!(fh, "{r} {g} {b}{end_char}")?;
        }
        Ok(())
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= W || y >= H {
            return;
        }
        self.buffer.set_pixel(x as u32, y as u32, color);
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: Color) {
        // Loop thru length and width adding px by px.
        for i in 0..w {
            for j in 0..h {
                let cx = x + i;
                let cy = y + j;
                self.draw_pixel(cx, cy, color);
            }
        }
    }

    pub fn draw_image(&mut self, x: usize, y: usize, image: &Image) {
        let (w, h) = (image.width(), image.height());
        for i in 0..w {
            for j in 0..h {
                let cx = x + i;
                let cy = y + j;
                if cx > W || cy > H {
                    continue;
                }
                let px = image.get_pixel(i as u32, j as u32);
                self.draw_pixel(cx, cy, px)
            }
        }
    }

    // TODO: Maybe move to Map.
    pub fn draw_map(&mut self, gs: &GameState, textures: &Textures) -> eyre::Result<()> {
        let rect_w = W / (gs.map.w * 2);
        let rect_h = H / gs.map.h;
        // eprintln!("Rects (w: {rect_w}, h: {rect_h})");

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
                        self.draw_rect(rect_x, rect_y, rect_w, rect_h, *color);
                    }
                    Texture::Sprite(img) => {
                        // Draw thumbnail
                        let img_thumbnail = nearest_neighbor(img, rect_w as u16, rect_h as u16);
                        self.draw_image(rect_x, rect_y, &img_thumbnail);
                    }
                };
            }
            continue;
        }
        self.draw_player_on_map(gs)?;
        self.draw_entities_on_map(gs)?;
        Ok(())
    }

    // TODO: Refactor draw_* to take a struct that implents and Entity trait
    pub fn draw_player_on_map(&mut self, gs: &GameState) -> eyre::Result<()> {
        let rect_w = W / (gs.map.w * 2);
        let rect_h = H / gs.map.h;
        // Convert from coordinates to image dim
        let x = (gs.player.x * rect_w as f32) as usize;
        let y = (gs.player.y * rect_h as f32) as usize;
        self.draw_rect(x, y, 5, 5, Color::from_rgba(0, 0, 0, 0));
        Ok(())
    }

    pub fn draw_entities_on_map(&mut self, gs: &GameState) -> eyre::Result<()> {
        let rect_w = W / (gs.map.w * 2);
        let rect_h = H / gs.map.h;

        for entity in gs.id_enemy_map.values() {
            let x = (entity.x * rect_w as f32) as usize;
            let y = (entity.y * rect_h as f32) as usize;
            self.draw_rect(x, y, 5, 5, Color::from_rgba(255, 0, 0, 0));
        }
        Ok(())
    }

    pub fn draw_sprite(
        &mut self,
        enemy: &Enemy,
        player: &Player,
        textures: &Textures,
    ) -> eyre::Result<()> {
        // https://www.youtube.com/watch?v=VMYk9fqXz_4
        // https://stackoverflow.com/questions/283406/what-is-the-difference-between-atan-and-atan2-in-c
        // Use atan2 incase where x is negative. Allows getting angle with range across all 4 quadrants as opposed to 2 (1 and 4).
        // Angle of enemy relative to player
        let mut angle = (enemy.y - player.y).atan2(enemy.x - player.x);
        while angle - player.angle > PI {
            angle -= 2. * PI;
        } // remove unncesessary periods from the relative direction
        while angle - player.angle < -PI {
            angle += 2. * PI;
        }

        let Some(texture) = textures.get_enemy(enemy) else {
            bail!("No texture for enemy {enemy:?}")
        };
        // Scale sprite by distance from player and clamp to 2000 if very close.
        let sprite_screen_size = ((H as f32 / enemy.dst) as usize).min(2000);
        let h_offset = ((angle - player.angle) * (W / 2) as f32 / (player.fov)
            + (W / 2) as f32 / 2.
            - sprite_screen_size as f32 / 2.) as i32; // do not forget the 3D view takes only a half of the framebuffer, thus fb.w/2 for the screen width
        let v_offset = (H / 2 - sprite_screen_size / 2) as i32;

        for i in 0..sprite_screen_size {
            let i_int = i as i32;
            let px_x = h_offset + i_int;
            // Don't draw horizontal pixel if OOB
            if px_x < 0 || px_x >= W as i32 / 2 {
                continue;
            }
            // Occluded. Pixel at x-pos in front of sprite (closer).
            if TryInto::<usize>::try_into(px_x).is_ok_and(|x| self.depth_buffer[x] < enemy.dst) {
                continue;
            }

            for j in 0..sprite_screen_size {
                let j_int = j as i32;
                let px_y = v_offset + j_int;
                if px_y < 0 || px_y >= H as i32 {
                    continue;
                }
                let Some(color) = texture.get_color(
                    i * textures.size / sprite_screen_size,
                    j * textures.size / sprite_screen_size,
                ) else {
                    bail!("No color.")
                };

                // Only draw opaque pixels
                // https://colorlabs.net/posts/what-are-alpha-channels-in-digital-images
                if color.a > 0.5 {
                    let px_x = (W / 2) + px_x as usize;
                    let px_y = px_y as usize;
                    self.draw_pixel(px_x, px_y, color)
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
            .sorted_by(|a, b| {
                let dst_a = a.dst_from_player(&gs.player);
                let dst_b = b.dst_from_player(&gs.player);
                dst_a.total_cmp(&dst_b)
            })
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
    pub fn draw_ray(
        &mut self,
        x: f32,
        y: f32,
        ang: f32,
        gs: &GameState,
        textures: &Textures,
        mut f_hit: impl FnMut(&mut Screen<W, H>, &Texture, RayHit),
    ) -> eyre::Result<f32> {
        let rect_w = (W / (gs.map.w * 2)) as f32;
        let rect_h = (H / gs.map.h) as f32;

        // We don't include a limit (20) unlike the src
        const INC: f32 = 0.01;
        let mut dst: f32 = 0.0;
        loop {
            let cx = x + dst * ang.cos();
            let cy = y + dst * ang.sin();

            // Otherwise, draw ray
            let px_x = cx * rect_w;
            let px_y = cy * rect_h;
            self.draw_rect(
                px_x as usize,
                px_y as usize,
                1,
                1,
                Color::from_rgba(190, 190, 190, 128),
            );

            // Out of bounds or hit an object
            if let Some(htile) = gs.get_tile(cx as usize, cy as usize) {
                // Call function on hit.
                // TODO: Abstract to function to handle cases where no key
                let Some(texture) = &textures.get_tile(htile) else {
                    bail!("No texture for hit tile {htile:?}")
                };
                f_hit(self, texture, RayHit { cx, cy, dst });
                break;
            };

            dst += INC
        }
        Ok(dst)
    }

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
        let fw: f32 = (W / 2) as f32;
        // Angle between x-axis and fov
        // Direction - (FOV / 2)
        let pt_1 = gs.player.angle - gs.player.fov / 2.;
        for i in 0..(W / 2) {
            // The rest of the FOV angle drawn section by section.
            // (FOV * 0..512) / 512.
            let pt_2 = gs.player.fov * (i as f32 / fw);
            let angle = pt_1 + pt_2;
            self.draw_ray(
                gs.player.x,
                gs.player.y,
                angle,
                gs,
                textures,
                move |img, tile, ray_hit| {
                    // Closer means smaller c and thus large ht.
                    // We need to adjust this scaling to avoid fisheye distortion due to the ray hitting at multiple angles
                    // See https://gamedev.stackexchange.com/a/97580
                    // And https://lodev.org/cgtutor/raycasting.html
                    let col_ht =
                        (H as f32 / (ray_hit.dst * (angle - gs.player.angle).cos())) as usize;
                    // Draw at every angle within FOV
                    let col_x = W / 2 + i;

                    // Store distance to know what to occlude from fov
                    img.depth_buffer[i] = ray_hit.dst;

                    // Draw texture/tile
                    match tile {
                        Texture::Color(color) => {
                            // Start at middle of screen and then drop y by half the col ht. This centers the drawn line.
                            let col_y = H / 2 - col_ht / 2;
                            img.draw_rect(col_x, col_y, 1, col_ht, *color);
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
                                let Some(pix_y) = (j + (H / 2)).checked_sub(col_ht / 2) else {
                                    continue;
                                };
                                img.draw_pixel(col_x, pix_y, px)
                            }
                        }
                    };
                },
            )?;
        }
        Ok(())
    }

    pub fn render(&mut self, gs: &GameState, textures: &Textures) -> eyre::Result<()> {
        // Clear buffer
        self.clear();
        // Draw fov for player
        self.draw_fov(gs, textures)?;
        // Then draw map and player.
        self.draw_map(gs, textures)?;
        // And draw sprites
        self.draw_sprites(gs, textures)?;
        Ok(())
    }
}
