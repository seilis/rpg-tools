//! For rendering
use std::io::{Error, ErrorKind};

use image::{RgbaImage, Rgba, imageops::rotate90};

use rand::prelude::*;

use super::GridMap;
use super::gridmap::AreaType;

// Assets
const FLOOR_STONE: &str = include_str!("assets/floor-stone.svg");
const FLOOR_STONE_2: &str = include_str!("assets/floor-stone-2.svg");

/// A renderer that can take a map and draw it to a file
pub struct Renderer {
    map   : GridMap,
    scale : u32,

    /// Rendered assets
    assets : Vec<RgbaImage>,
}

impl Renderer {
    pub fn new(map: &GridMap, scale: usize) -> Renderer {
        let mut new = Renderer {
            map: map.to_owned(),
            scale: scale as u32,
            assets: vec![],
        };

        new.render_sprites(scale).expect("unable to render sprites, aborting");
        new
    }

    pub fn draw_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
        const GRID_SEP_COLOUR: Rgba<u8> = Rgba([190, 190, 190, 255]);

        let (xmax, ymax) = self.map.get_limits();
        // Apparently not possible to simultaneously cast these
        let xmax = xmax as u32;
        let ymax = ymax as u32;

        let mut img = RgbaImage::new((xmax*self.scale) as u32, (ymax*self.scale) as u32);

        // Loop through all of our cells
        for x in 0..xmax {
            for y in 0..ymax {
                // Set the colour of this cell. All pixels within the cell
                // will have this value.
                let color = match self.map.get_cell_ref(x as usize, y as usize).area {
                    AreaType::Room => Rgba([200, 200, 200, 255]),
                    AreaType::Entrance => Rgba([255, 119, 0, 255]),
                    _ => Rgba([25, 25, 25, 255]),
                };

                if self.map.get_cell_ref(x as usize, y as usize).area == AreaType::Room {
                    let mut sprite = self.get_floor_sprite().expect("failed to open file");
                    let mut rng = thread_rng();
                    let dist = rand::distributions::Uniform::new_inclusive(0, 3);
                    for _ in 0..rng.sample(dist) {
                        sprite = rotate90(&sprite);
                    }
                    self.draw_sprite_at(x, y, &mut img, &sprite);
                } else {
                    // Loop through all of the pixels in the cell.
                    for x_pixel in x*self.scale .. x*self.scale+self.scale {
                        for y_pixel in y*self.scale .. y*self.scale+self.scale {
                            img.put_pixel(x_pixel, y_pixel, color);
                        }
                    }
                }

                // Now check whether we need to draw the borders of the cell
                if x < xmax - 1
                    && self.map.get_cell_ref(x as usize, y as usize).area == AreaType::Room
                    && self.map.get_cell_ref(x as usize + 1, y as usize).area == AreaType::Room
                {
                    let x_pixel = (x+1)*self.scale - 1;
                    for y_pixel in y*self.scale .. (y+1)*self.scale {
                        img.put_pixel(x_pixel, y_pixel, GRID_SEP_COLOUR);

                    }
                }
                if x > 0
                    && self.map.get_cell_ref(x as usize, y as usize).area == AreaType::Room
                    && self.map.get_cell_ref(x as usize - 1, y as usize).area == AreaType::Room
                {
                    // Explanation is the same as above but now it's the first
                    // pixel in our box
                    let x_pixel = x*self.scale;
                    for y_pixel in y*self.scale .. (y+1)*self.scale {
                        img.put_pixel(x_pixel, y_pixel, GRID_SEP_COLOUR);

                    }
                }
                if y < ymax - 1
                    && self.map.get_cell_ref(x as usize, y as usize).area == AreaType::Room
                    && self.map.get_cell_ref(x as usize, y as usize +1).area == AreaType::Room
                {
                    let y_pixel = (y+1)*self.scale - 1;
                    for x_pixel in x*self.scale .. (x+1)*self.scale {
                        img.put_pixel(x_pixel, y_pixel, GRID_SEP_COLOUR);
                    }
                }
                if y > 0
                    && self.map.get_cell_ref(x as usize, y as usize).area == AreaType::Room
                    && self.map.get_cell_ref(x as usize, y as usize-1).area == AreaType::Room
                {
                    // Explanation is the same as above.
                    let y_pixel = y*self.scale;
                    for x_pixel in x*self.scale .. (x+1)*self.scale {
                        img.put_pixel(x_pixel, y_pixel, GRID_SEP_COLOUR);
                    }
                }
            }
        }

        // Now we have filled out the entire pixel array, we pass it to the
        // encode() method. At the moment, we just need a grayscale image.
        match img.save(filename) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(ErrorKind::Other, "failed to encode image")),
        }
    }

    /// Get a floor sprite as an RGBA image
    fn get_floor_sprite(&self) -> Result<RgbaImage, std::io::Error> {
        let mut rng = thread_rng();
        let dist = rand::distributions::Uniform::new_inclusive(0, 1);
        let sample = rng.sample(dist);


        if sample < self.assets.len() {
            return Ok(self.assets[sample].clone());
        }

        Err(std::io::Error::new(std::io::ErrorKind::NotFound, ":-("))
    }

    fn render_sprites(&mut self, size: usize) -> Result<(), std::io::Error> {
        let sprites_raw = [FLOOR_STONE, FLOOR_STONE_2];

        for sprite in sprites_raw {
            let mut options = usvg::Options::default();
            options.resources_dir = std::fs::canonicalize("src/floor-stone.svg")
                                        .ok()
                                        .and_then(
                                            |p| p.parent().map(
                                                |p| p.to_path_buf()));

            let rtree = usvg::Tree::from_str(&sprite, &options.to_ref()).unwrap();
            let mut pixmap = tiny_skia::Pixmap::new(size as u32, size as u32).unwrap();
            resvg::render(&rtree,
                          usvg::FitTo::Width(self.scale as u32),
                          tiny_skia::Transform::identity(),
                          pixmap.as_mut()).unwrap();

            let image = RgbaImage::from_vec(size as u32, size as u32, pixmap.take()).unwrap();
            self.assets.push(image);
        }
        Ok(())
    }

    /// Draw a sprite into a location in the image
    fn draw_sprite_at(&self, x: u32, y: u32, image: &mut RgbaImage, sprite: &RgbaImage) {
        let base_x = x * self.scale;
        let base_y = y * self.scale;

        for (px, py, pixel) in sprite.enumerate_pixels() {
            image.put_pixel(base_x+px, base_y+py, *pixel);
        }
    }
}
