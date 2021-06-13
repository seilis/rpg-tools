use std::io::{Error, ErrorKind};

use image::{RgbImage, Rgb};

use super::GridMap;
use super::gridmap::AreaType;


pub struct Renderer {
    map   : GridMap,
    scale : u32,
}

impl Renderer {
    pub fn new(map: &GridMap, scale: usize) -> Renderer {
        Renderer {
            map: map.to_owned(),
            scale: scale as u32,
        }
    }

    pub fn draw_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
        const GRID_SEP_COLOUR: Rgb<u8> = Rgb([190, 190, 190]);

        let (xmax, ymax) = self.map.get_limits();
        // Apparently not possible to simultaneously cast these
        let xmax = xmax as u32;
        let ymax = ymax as u32;

        let mut img = RgbImage::new((xmax*self.scale) as u32, (ymax*self.scale) as u32);

        // Loop through all of our cells
        for x in 0..xmax {
            for y in 0..ymax {
                // Set the colour of this cell. All pixels within the cell
                // will have this value.
                let color = match self.map.get_cell_ref(x as usize, y as usize).area {
                    AreaType::Room => Rgb([200, 200, 200]),
                    AreaType::Entrance => Rgb([255, 119, 0]),
                    _ => Rgb([25, 25, 25]),
                };

                // Loop through all of the pixels in the cell.
                for x_pixel in x*self.scale .. x*self.scale+self.scale {
                    for y_pixel in y*self.scale .. y*self.scale+self.scale {
                        img.put_pixel(x_pixel, y_pixel, color);
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
}
