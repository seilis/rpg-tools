use std::fs::File;
use std::io::{Error, ErrorKind};

use image::png::PngEncoder;
use image::ColorType;

use super::GridMap;
use super::gridmap::AreaType;


pub struct Renderer {
    map   : GridMap,
    scale : usize,
}

impl Renderer {
    pub fn new(map: &GridMap, scale: usize) -> Renderer {
        Renderer {
            map: map.to_owned(),
            scale,
        }
    }

    pub fn draw_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
        const GRID_SEP_COLOUR: u8 = 190;

        let output = File::create(filename)?;
        let encoder = PngEncoder::new(output);

        let (xmax, ymax) = self.map.get_limits();

        // Find the limits of our image
        let row_length = xmax * self.scale;
        let num_rows = ymax * self.scale;

        // Allocate enough pixels to hold the image. Note that the encode()
        // function used below requires this to be a linear array.
        let mut pixels = vec![0; (xmax) * (ymax) * (self.scale * self.scale) as usize];

        // Loop through all of our cells
        for x in 0..xmax {
            for y in 0..ymax {
                // Set the colour of this cell. All pixels within the cell
                // will have this value.
                let color = match self.map.get_cell_ref(x, y).area {
                    AreaType::Room => 200,
                    AreaType::Entrance => 255,
                    _ => 25,
                };

                // Loop through all of the pixels in the cell. We do this by
                // calculating a base offset and then filling in all of the
                // horizontal pixels before moving on to the next row. The
                // base offset is the number pixels in a row multiplied
                // by the number of rows down, then we add the x-offset to
                // determine where we should begin.
                for i in 0..self.scale {
                    let base = x * self.scale + (y * self.scale + i) * row_length;
                    for j in base..(base + self.scale) {
                        pixels[j] = color;
                    }
                }

                // Now check whether ne need to draw the borders of the cell
                if x < xmax - 1
                    && self.map.get_cell_ref(x, y).area == AreaType::Room
                    && self.map.get_cell_ref(x + 1, y).area == AreaType::Room
                {
                    // Base calculation: the pixels are packed into a linear
                    // array where a whole horizontal row is adjacent. So the
                    // index in general is:  idx = y * row_length + x
                    //
                    // In this case, we are drawing a vertical line so we
                    // have to keep calculating this index rather than just
                    // using a range. Here, we're finding the base by taking
                    // the first horizontal pixel of the _next_ horizontal
                    // cell (x+1)*scale and subtracting by 1, which is the
                    // last pixel of our own cell. All of this needs to be
                    // offset by our y index row length to get the pixel
                    // for our cell in the y offset. When drowing, this
                    // is the pixel in the top right-hand corner of the cell.
                    let base = (x + 1) * self.scale - 1 + y * self.scale * row_length;
                    // Draw the vertical line. Need to visit each pixel on
                    // the rightmost side.
                    for row in 0..self.scale {
                        // Since we know our base, we can just offset by the
                        // row_length each time to find the pixel directly
                        // below the last.
                        let index = base + row * row_length;
                        pixels[index] = GRID_SEP_COLOUR;
                    }
                }
                if x > 0
                    && self.map.get_cell_ref(x, y).area == AreaType::Room
                    && self.map.get_cell_ref(x - 1, y).area == AreaType::Room
                {
                    // Explanation is the same as above but now it's the first
                    // pixel in our box
                    let base = x * self.scale + y * self.scale * row_length;
                    for row in 0..self.scale {
                        // Since we know our base, we can just offset by the
                        // row_length each time to find the pixel directly
                        // below the last.
                        let index = base + row * row_length;
                        pixels[index] = GRID_SEP_COLOUR;
                    }
                }
                if y < ymax - 1
                    && self.map.get_cell_ref(x, y).area == AreaType::Room
                    && self.map.get_cell_ref(x, y+1).area == AreaType::Room
                {
                    // Explanation is the same as above but now it's a horizontal
                    // line so we can just use the range syntax.
                    let base = x * self.scale + y * self.scale * row_length + (self.scale - 1) * row_length;
                    for index in base..base + self.scale {
                        pixels[index] = GRID_SEP_COLOUR;
                    }
                }
                if y > 0
                    && self.map.get_cell_ref(x, y).area == AreaType::Room
                    && self.map.get_cell_ref(x, y-1).area == AreaType::Room
                {
                    // Explanation is the same as above but now it's a horizontal
                    // line so we can just use the range syntax.
                    let base = x * self.scale + y * self.scale * row_length;
                    for index in base..base + self.scale {
                        pixels[index] = GRID_SEP_COLOUR;
                    }
                }
            }
        }

        // Now we have filled out the entire pixel array, we pass it to the
        // encode() method. At the moment, we just need a grayscale image.
        match encoder.encode(&pixels, row_length as u32, num_rows as u32, ColorType::L8) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(ErrorKind::Other, "failed to encode image")),
        }
    }
}
