// std library
use std::cmp;
use std::collections::VecDeque;
use std::fs::File;

// Extern crates
extern crate rand;
use rand::{seq, thread_rng, Rng};
use rand::distributions::{IndependentSample, Range};

extern crate image;
use image::ColorType;
use image::png::PNGEncoder;

// Local modules
mod gridcell;
use gridcell::{GridCell, AreaType};

pub mod route;
use route::RouteMethod;

#[derive(Clone, Debug)]
pub struct GridMap {
    xmax: usize,
    ymax: usize,
    cells: Vec<Vec<GridCell>>
}

impl GridMap {
    /// Make a new GridMap
    pub fn new(xmax: usize, ymax: usize) -> GridMap {
        GridMap {
            xmax: xmax,
            ymax: ymax,
            cells: vec![vec![GridCell::new(); ymax]; xmax]
        }
    }

    /// Set the entrance
    pub fn place_entrance(&mut self, (x, y): (usize, usize)) {
        if x >= self.xmax || y >= self.ymax {
            panic!("Tried to place an entrance outside the bounds of the map");
        }

        self.cells[x][y].area = AreaType::Entrance;
    }

    /// Set a room
    pub fn place_room(&mut self, (x0, y0): (usize, usize), (x1, y1): (usize, usize)) {
        // For lower bounds, note that 0 is implicit lower bound in usize
        let x_lower = cmp::min(x0, x1); // Calculate the lower bound of x
        let y_lower = cmp::min(y0, y1); // Calculate the lower bound of y
        let x_upper = cmp::min(self.xmax, cmp::max(x0, x1)); // Calculate the upper bound of x
        let y_upper = cmp::min(self.ymax, cmp::max(y0, y1)); // Calculate the upper bound of y

        for i in x_lower .. x_upper+1 {
            for j in y_lower .. y_upper+1 {
                if self.cells[i][j].area != AreaType::Entrance {
                    self.cells[i][j].area = AreaType::Room;
                }
            }
        }
    }

    /// Place a room by setting the origin and width/height
    /// # Arguments:
    ///     (x, y) -> The origin of the room. This will be a corner.
    ///     (w, h) -> Width/Height of the room. Note, they're isize because
    ///               you can specify the w/h to be left OR right of the origin
    ///               and up OR down, respectively.
    pub fn place_room_dimensions(&mut self, (x, y): (usize, usize), (w, h): (isize, isize)) {
        let x_lower = if w >= 0 { x } else { x - w as usize };
        let y_lower = if h >= 0 { y } else { y - h as usize };
        let x_upper = if w >= 0 { x + w as usize } else { x };
        let y_upper = if h >= 0 { y + h as usize } else { y };

        self.place_room((x_lower, y_lower), (x_upper, y_upper));
    }

    /// Place a hallway between two points
    pub fn place_hallway(&mut self, (x0, y0): (usize, usize), (x1, y1): (usize, usize), route: RouteMethod) {
        let route_selected = match route {
            RouteMethod::HorizontalFirst => RouteMethod::HorizontalFirst,
            RouteMethod::VerticalFirst => RouteMethod::VerticalFirst,
            RouteMethod::Manhattan => {
                if rand::random::<bool>() {
                    RouteMethod::HorizontalFirst
                } else {
                    RouteMethod::VerticalFirst
                }
            }
        };

        match route_selected {
            RouteMethod::HorizontalFirst => {
                self.place_room((x0, y0), (x1, y0));
                self.place_room((x1, y0), (x1, y1));
            },
            RouteMethod::VerticalFirst => {
                self.place_room((x0, y0), (x0, y1));
                self.place_room((x0, y1), (x1, y1));
            }
            _ =>  panic!("Found unsupported route!")
        };
    }

    /// Find the nearest connected cell to the cell specified
    pub fn find_nearest_connected(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        let mut rooms = Vec::<(usize,usize)>::new();
        let mut found = false;
        let mut radius: usize = 0;

        if x >= self.xmax || y >= self.ymax {
            return None;
        }

        // Do an early check that the origin isn't a room. If it is, just return
        // it directly.
        if self.cells[x][y].is_room() {
            return Some((x, y));
        }

        while !found {
            radius += 1; // Increase search radius every loop
            let xmin = x.saturating_sub(radius); // Bounds xmin at 0
            let xmax = cmp::min(self.xmax-1, x+radius); // Bounds xmax at self.xmax
            let ymin = y.saturating_sub(radius);
            let ymax = cmp::min(self.ymax-1, y+radius);

            if xmin == 0 && ymin == 0 && xmax == self.xmax-1 && ymax == self.ymax-1 {
                // If this condition is true then we've searched the whole grid
                // and didn't find what we were looking for. Return None, which
                // indicates that no rooms were found.
                return None
            }

            // Scan bottom x
            for i in xmin .. xmax+1 {
                if self.cells[i][ymin].is_room() {
                    rooms.push((i, ymin));
                    found = true;
                }
            }

            // Scan top x
            for i in xmin .. xmax+1 {
                if self.cells[i][ymax].is_room() {
                    rooms.push((i, ymax));
                    found = true;
                }
            }

            // Scan left y
            for j in ymin .. ymax+1 {
                if self.cells[xmin][j].is_room() {
                    rooms.push((xmin, j));
                    found = true;
                }
            }

            // Scan right y
            for j in ymin .. ymax+1 {
                if self.cells[xmax][j].is_room() {
                    rooms.push((xmax, j));
                    found = true;
                }
            }
        }

        // Finished our search loop. Now return a random room
        if rooms.len() > 0 {
            let mut rng = thread_rng();
            // This is probably the most complicated expression I've written in Rust
            // so far, so I'll just make the note here about what's happening.
            // seq::sample_iter() takes a random-number generator, an iterable and
            // the number of items you want out of the iterable. It returns an Option
            // so I need to unpack it. I have already checked that the vector has
            // a non-zero length so this should always return something. Lastly,
            // the return type is a Vec so I take the first index, since I know
            // that I only requested one.
            Some(seq::sample_iter(&mut rng, rooms, 1).unwrap()[0])
        } else {
            None
        }
    }

    /// Generate random cells with a biasing towards more/less rooms. Limit is a value
    /// between 1 and 100. This limit sets the chance that the cells are a room.
    /// Higher limit means fewer rooms.
    pub fn generate_random_cells(&mut self, limit: i64) {
        let mut rng = thread_rng();
        let between = Range::new(1, 100);
        for i in 0 .. self.xmax {
            for j in 0 .. self.ymax {
                let val = between.ind_sample(&mut rng);
                if val > limit {
                    self.cells[i][j].area = AreaType::Room;
                } else {
                    self.cells[i][j].area = AreaType::Nothing;
                }
            }
        }
    }

    pub fn generate_annealed_random_cells(&mut self) {
        // Start by generating a random grid
        self.generate_random_cells(80);

        // Anneal by removing stragglers
        for i in 1 .. self.xmax {
            for j in 1 .. self.ymax {
                let alone = self.cells[i-1][j].is_empty() && self.cells[i][j-1].is_empty()
                            && self.cells[i+1][j].is_empty() && self.cells[i][j+1].is_empty();
                if alone {
                    self.cells[i][j].area = AreaType::Nothing;
                }
            }
        }
    }

    /// Place a randomly sized room of up to scale length or width.
    pub fn place_random_room(&mut self, scale: usize, connect: bool) {
        // Initialize a random number generator
        let mut rng = rand::thread_rng();

        // Generate size of the room
        let width = rng.gen_range(2, scale);
        let height = rng.gen_range(2, scale);

        // Generate the origin (location) of the room
        let x0 = rng.gen_range(1, self.xmax);
        let y0 = rng.gen_range(1, self.ymax);

        // See if we need to connect the room to an existing one.
        if connect {

            // Find the nearest connected location and return
            // the coordinates.
            let (x, y) = self.find_nearest_connected((x0, y0)).expect("no existing rooms to connect");
            // Drow the hallway; some of this will be overwritten by
            // the room placement below.
            self.place_hallway((x0, y0), (x, y), RouteMethod::Manhattan);
        }

        // Set x/y min/max while checking for overflows on either
        // the lower or upper bounds.
        let xmin = x0.saturating_sub(width/2);
        let ymin = y0.saturating_sub(height/2);
        let xmax = cmp::min(self.xmax-1, x0+width/2);
        let ymax = cmp::min(self.ymax-1, y0+width/2);

        self.place_room((xmin, ymin),
                        (xmax, ymax));
    }

    /// Determine if i, or j lies on an edge.
    fn on_edge(&self, i: usize, j: usize) -> bool {
        // Logic here is pretty simple: if either i or j is the max
        // value or 0 then they're on the edge of the map. All maps
        // are rectangular.
        i == 0 || i == self.xmax-1 || j ==0 || j == self.ymax-1
    }

    fn cave_anneal_cell(&self, i: usize, j: usize) -> bool {
        if !self.on_edge(i, j) {
            let mut neighbours = 0;

            // 3x3 grid
            for x in i-1..i+2 {
                for y in j-1..j+2 {
                    if self.cells[x][y].area == AreaType::Room {
                        neighbours += 1;
                    }
                }
            }

            // Whether we can anneal the cell
            neighbours >= 5
        } else {
            false
        }
    }

    fn generate_cave_iteration(&mut self) {
        let mut tmp_map = self.cells.to_vec();

        for i in 0..self.xmax {
            for j in 0..self.ymax {
                if self.cave_anneal_cell(i, j) {
                    tmp_map[i][j].area = AreaType::Room;
                } else {
                    tmp_map[i][j].area = AreaType::Nothing;
                }
            }
        }

        self.cells = tmp_map;
    }

    /// Clear away small, unattached rooms.
    fn remove_orphans(&mut self) {
        let mut size;
        for i in 0..self.xmax {
            for j in 0..self.ymax {
                size = self.get_room_size(i, j);
                if size < 15 {
                    self.clear_room(i, j);
                }
            }
        }
    }

    fn get_room_size(&self, i: usize, j: usize) -> u64 {
        // Output value
        let mut size = 0;

        // Process all of the points in a list. Doing this in an iterative fashion because
        // of max recursion limits.
        let mut proc_queue = VecDeque::new();
        proc_queue.push_back((i, j));

        // Mask to make sure we don't revisit rooms
        let mut visited = vec![false; (self.xmax)*(self.ymax)];

        while proc_queue.len() > 0 {
            // Remove a room from the queue and unpack the values
            let (x, y) = proc_queue.pop_front().unwrap();

            // If we have visited this cell before or it is not a "room" then
            // we should stop processing and move on to the next
            if visited[x*self.xmax+y] {
                continue;
            } else if self.cells[x][y].area != AreaType::Room {
                continue
            }

            // We got here so it's a room that we haven't visited. Mark it as visited now
            visited[x*self.xmax+y] = true;

            // This is a room and we haven't visited, so add one to size.
            size += 1;

            // Add all neighbours to the queue
            proc_queue.push_back((x-1, y));
            proc_queue.push_back((x+1, y));
            proc_queue.push_back((x, y-1));
            proc_queue.push_back((x, y+1));
        }
        // Returning the full size of the room
        size
    }

    fn clear_room(&mut self, x: usize, y: usize) {
        if self.cells[x][y].area == AreaType::Nothing {
            return;
        }

        let mut proc_queue = VecDeque::new();
        proc_queue.push_back((x, y));

        while proc_queue.len() > 0 {
            let (i, j) = proc_queue.pop_front().unwrap();

            if self.cells[i][j].area == AreaType::Nothing {
                continue;
            }

            self.cells[i][j].area = AreaType::Nothing;
            proc_queue.push_back((i+1, j));
            proc_queue.push_back((i-1, j));
            proc_queue.push_back((i, j+1));
            proc_queue.push_back((i, j-1));
        }
    }

    pub fn generate_cave(&mut self, iter: i64, seed_limit: i64) {
        self.generate_random_cells(seed_limit);

        for _ in 0 .. iter {
            self.generate_cave_iteration();
        }
        self.remove_orphans();
    }

    /// Make a PNG file of the gridmap
    pub fn draw_to_file(&self, filename: &str, scale: usize) -> Result<(), std::io::Error> {
        let output = File::create(filename)?;
        let encoder = PNGEncoder::new(output);

        // Find the limits of our image
        let row_length = self.xmax*scale;
        let num_rows = self.ymax*scale;

        // Allocate enough pixels to hold the image. Note that the encode()
        // function used below requires this to be a linear array.
        let mut pixels = vec![0; (self.xmax)*(self.ymax)*(scale*scale) as usize];
                           
        // Loop through all of our cells
        for x in 0 .. self.xmax {
            for y in 0 .. self.ymax {
                // Set the colour of this cell. All pixels within the cell
                // will have this value.
                let color = match self.cells[x][y].area {
                    AreaType::Room => 200,
                    AreaType::Entrance => 255,
                    _ => 25
                };

                // Loop through all of the pixels in the cell. We do this by
                // calculating a base offset and then filling in all of the
                // horizontal pixels before moving on to the next row. The
                // base offset is the number pixels in a row multiplied
                // by the number of rows down, then we add the x-offset to
                // determine where we should begin.
                for i in 0..scale {
                    let base = x*scale + (y*scale+i)*row_length;
                    for j in base..(base+scale) {
                        pixels[j] = color;
                    }
                }
            }
        }

        // Now we have filled out the entire pixel array, we pass it to the
        // encode() method. At the moment, we just need a grayscale image.
        encoder.encode(&pixels, row_length as u32, num_rows as u32, ColorType::Gray(8))?;
        Ok(()) // Finished!
    }
}
