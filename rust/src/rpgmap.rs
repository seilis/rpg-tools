// rpgmap main source file
//
// Program for making simple RPG maps. This is the Rust language implementation.
use std::clone::Clone;
use std::collections::VecDeque;
use std::cmp;
use rand::{seq, thread_rng};
use rand::distributions::{IndependentSample, Range};
use std::fs::File;
use image::ColorType;
use image::png::PNGEncoder;

extern crate rand;
extern crate image;

// TODO: split this into multiple files. For now, just use here.
#[derive(Debug)]
enum RouteMethod {
    HorizontalFirst,
    VerticalFirst,
//    Diagonal,
//    Circular,
    Manhattan,
//    Random,
}


#[derive(Clone, Debug, PartialEq)]
enum AreaType {
    Nothing,
    Entrance,
    Room,
//    Stairs,
//    Tested,
}

#[derive(Clone, Debug)]
enum WallType {
    Nothing,
    Wall,
//    Door,
//    SecretDoor,
}

#[derive(Clone, Debug)]
enum PointType {
    Nothing,
//    Pillar,
}

/// Representation of a GridCell, which is a single unit in a grid.
#[derive(Clone, Debug)]
struct GridCell {
    // Accessable_t (Python) not needed in Rust. We'll use an
    // Option<bool> to represent that.
    accessable: Option<bool>,
    area: AreaType,
    vert_wall: WallType,
    horiz_wall: WallType,
    point: PointType,
}

impl GridCell {
    /// Construct a new GridCell
    fn new() -> GridCell {
        GridCell {
            accessable: None,
            area: AreaType::Nothing,
            vert_wall: WallType::Nothing,
            horiz_wall: WallType::Nothing,
            point: PointType::Nothing,
        }
    }

    /// Check if a GridCell is 'empty', meaning that all drawing-related
    /// fields have a value of 'Nothing'.
    fn is_empty(&self) -> bool {
        match self {
            &GridCell { area: AreaType::Nothing,
                       vert_wall: WallType::Nothing,
                       horiz_wall: WallType::Nothing,
                       point: PointType::Nothing,
                       .. } => true,
            _ => false
        }
    }

    /// Check whether the GridCell represents a dungeon entrance
    fn is_entrance(&self) -> bool {
        match self {
            &GridCell { area: AreaType::Entrance, ..} => true,
            _ => false
        }
    }

    /// Check whether the GridCell is part of a dungeon room
    fn is_room(&self) -> bool {
        match self {
            &GridCell { area: AreaType::Room, ..} => true,
            _ => false
        }
    }
}


struct GridMap {
    xmax: usize,
    ymax: usize,
    cells: Vec<Vec<GridCell>>
}

impl GridMap {
    /// Make a new GridMap
    fn new(xmax: usize, ymax: usize) -> GridMap {
        let vec = vec![GridCell::new(); ymax];
        GridMap {
            xmax: xmax,
            ymax: ymax,
            cells: vec![vec; xmax]
        }
    }

    /// Set the entrance
    fn place_entrance(&mut self, (x, y): (usize, usize)) {
        for xshift in x .. self.xmax {
            match self.cells[x][y].area {
                AreaType::Room => 
                {
                    self.cells[xshift][y].area = AreaType::Entrance;
                    break
                },
                _ => continue
            }
        }
    }

    /// Set a room
    fn place_room(&mut self, (x0, y0): (usize, usize), (x1, y1): (usize, usize)) {
        // For lower bounds, note that 0 is implicit lower bound in usize
        let x_lower = cmp::min(x0, x1); // Calculate the lower bound of x
        let y_lower = cmp::min(y0, y1); // Calculate the lower bound of y
        let x_upper = cmp::min(self.xmax, cmp::max(x0, x1)); // Calculate the upper bound of x
        let y_upper = cmp::min(self.ymax, cmp::max(y0, y1)); // Calculate the upper bound of y 

        for i in x_lower .. x_upper+1 {
            for j in y_lower .. y_upper+1 {
                self.cells[i][j].area = AreaType::Room;
            }
        }
    }

    /// Place a room by setting the origin and width/height
    /// Arguments:
    ///     (x, y) -> The origin of the room. This will be a corner.
    ///     (w, h) -> Width/Height of the room. Note, they're isize because
    ///               you can specify the w/h to be left OR right of the origin
    ///               and up OR down, respectively.
    fn place_room_dimensions(&mut self, (x, y): (usize, usize), (w, h): (isize, isize)) {
        let x_lower = if w >= 0 { x } else { x - w as usize };
        let y_lower = if h >= 0 { y } else { y - h as usize };
        let x_upper = if w >= 0 { x + w as usize } else { x };
        let y_upper = if h >= 0 { y + h as usize } else { y };
        
        self.place_room((x_lower, y_lower), (x_upper, y_upper));
    }

    /// Place a hallway between two points
    fn place_hallway(&mut self, (x0, y0): (usize, usize), (x1, y1): (usize, usize), route: RouteMethod) {
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
    fn find_nearest_connected(self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        let mut rooms = Vec::<(usize,usize)>::new();
        let mut found = false;
        let mut radius: usize = 0;

        // Do an early check that the origin isn't a room. If it is, just return
        // it directly.
        if self.cells[x][y].is_room() {
            return Some((x, y));
        }

        while !found {
            radius += 1; // Increase search radius every loop
            let xmin = x.saturating_sub(radius); // Bounds xmin at 0
            let xmax = cmp::min(self.xmax, x+radius); // Bounds xmax at self.xmax
            let ymin = y.saturating_sub(radius);
            let ymax = cmp::min(self.ymax, y+radius);

            if xmin == 0 && ymin == 0 && xmax == self.xmax && ymax == self.ymax {
                // Todo, return error?
                found = true;
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
                    rooms.push((i, ymin));
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
    fn generate_random_cells(&mut self, limit: i64) {
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

    fn generate_annealed_random_cells(&mut self) {
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

    /// Determine if i, or j lies on an edge.
    fn on_edge(&self, i: usize, j: usize) -> bool {
        let xmax = self.xmax;
        let ymax = self.ymax;

        match (i, j) {
            (0, j) => true,
            (xmax, j) => true,
            (i, 0) => true,
            (i, ymax) => true,
            _ => false
        }
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
        let mut visited = vec![false; (self.xmax+1)*(self.ymax+1)];

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

    fn generate_cave(&mut self, iter: i64, seed_limit: i64) {
        self.generate_random_cells(seed_limit);

        for i in 0 .. iter {
            self.generate_cave_iteration();
        }
        self.remove_orphans();
    }

    /// Make a PNG file of the gridmap
    fn draw_to_file(self, filename: &str, scale: usize) -> Result<(), std::io::Error> {
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
                // will have this value
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




fn main() {
    let mut map = GridMap::new(100, 100);
    //map.generate_cave(4, 51);
    map.generate_random_cells(80);

    map.place_room((0, 0), (0, 0));
    map.place_room((1, 0), (1, 0));
    map.place_room((99, 0), (99, 0));
    map.place_room((99, 99), (99, 99));

    map.place_room((47, 47), (53, 53));
    map.place_entrance((50, 50));

    let filename = "example.png";

    let result = map.draw_to_file("example.png", 10);

    match result {
        Ok(o) => println!("Dungeon generated: {}", filename),
        Err(e) => println!("Error: {}", e),
    }
}
