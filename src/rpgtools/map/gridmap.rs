// std library
use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::ops::Index;

// Extern crates
use rand::prelude::*;
use rand::{thread_rng, Rng};

use itertools::Itertools;

// Local modules
use super::cell::Cell;
use super::room::Room;
pub use super::point::Point;
use super::area::Area;

// Need RouteMethod from rpgmap::route
use super::route::RouteMethod;

use crate::error::{Result, RpgError};

/// A map with grid coordinates
///
/// # Examples
/// ```
/// # use rpgtools::map::GridMap;
/// // Make a map that's 25x25 cells in size.
/// let map = GridMap::new(25, 25);
/// ```
#[derive(Clone, Debug)]
pub struct GridMap {
    xmax: usize,
    ymax: usize,
    cells: Vec<Vec<Cell>>,
}

impl GridMap {
    /// Make a new GridMap
    /// # Examples
    /// ```
    /// # use rpgtools::map::GridMap;
    /// // Make a map that's 25x25 cells in size.
    /// let map = GridMap::new(25, 25);
    /// ```
    pub fn new(xmax: usize, ymax: usize) -> GridMap {
        GridMap {
            xmax,
            ymax,
            cells: vec![vec![Cell::new(); ymax]; xmax],
        }
    }

    /// Returns size of map in (x, y) format
    ///
    /// # Examples
    /// ```
    /// # let map = rpgtools::map::GridMap::new(25, 25);
    /// let limits = map.get_limits();
    /// assert_eq!(limits, (25, 25));
    /// ```
    pub fn get_limits(&self) -> (usize, usize) {
        (self.xmax, self.ymax)
    }

    /// Get a reference to a GridCell at coordinate.
    pub fn get_cell_ref(&self, point: impl Into<Point>) -> &Cell {
        let (x, y): (usize, usize) = point.into().try_into().unwrap();
        &self.cells[x][y]
    }

    /// Get a mutable reference to a GridCell at coordinate.
    pub fn get_cell_mut(&mut self, point: impl Into<Point>) -> &mut Cell {
        let (x, y): (usize, usize) = point.into().try_into().unwrap();
        &mut self.cells[x][y]
    }

    /// Set the entrance at a particular location. The cell at this location
    /// will be marked as having the entrance area type. Typically this will
    /// be coloured differently on a map.
    pub fn place_entrance(&mut self, point: impl Into<Point>) -> Result<()> {
        let (x, y): (usize, usize) = point.into().try_into().unwrap();
        if x >= self.xmax || y >= self.ymax {
            return Err(RpgError::OutOfBounds);
        }

        self.cells[x][y].area = Area::Entrance;
        Ok(())
    }

    /// Similar to place entrance, however it starts with the coordinates and
    /// finds the nearest spot that is already a "room". This allows entrances
    /// to be placed in non-deterministic generators, such as caves.
    pub fn place_entrance_near(&mut self, point: impl Into<Point>) -> Result<()> {
        let point = point.into();
        if !point.is_in_bounds(Point::new(0, 0), (self.xmax, self.ymax).try_into().unwrap()) {
            return Err(RpgError::OutOfBounds);
        }

        let point = self
            .find_by(point, &|cell: &Cell| -> bool { cell.is_room() })
            .unwrap();

        self.place_entrance(point)?;
        Ok(())
    }

    /// Set a room
    ///
    /// The room is specified by two sets of coordinates. The coordinates specify the bounds of the
    /// resulting room. The X-axis location of the room is the two specified X coordinates and all
    /// of the cells in between them. The Y-axis location of the room is the two specified Y
    /// coordinates and all of the cells in between them.
    pub fn place_room(&mut self, point0: impl Into<Point>, point1: impl Into<Point>) {
        let (x0, y0): (usize, usize) = point0.into().try_into().unwrap();
        let (x1, y1): (usize, usize) = point1.into().try_into().unwrap();

        // For lower bounds, note that 0 is implicit lower bound in usize
        let x_lower = cmp::min(x0, x1); // Calculate the lower bound of x
        let y_lower = cmp::min(y0, y1); // Calculate the lower bound of y
        let x_upper = cmp::min(self.xmax, cmp::max(x0, x1)); // Calculate the upper bound of x
        let y_upper = cmp::min(self.ymax, cmp::max(y0, y1)); // Calculate the upper bound of y

        for i in x_lower..x_upper + 1 {
            for j in y_lower..y_upper + 1 {
                if self.cells[i][j].area != Area::Entrance {
                    self.cells[i][j].area = Area::Room;
                }
            }
        }
    }

    /// Place a room by setting the origin and width/height
    ///
    /// Arguments:
    ///  *   (x, y) -> The origin of the room. This will be a corner.
    ///  *   (w, h) -> Width/Height of the room. Note, they're isize because
    ///               you can specify the w/h to be left OR right of the origin
    ///               and up OR down, respectively.
    pub fn place_room_dimensions(&mut self, point: impl Into<Point>, (w, h): (isize, isize)) {
        let (x, y): (usize, usize) = point.into().try_into().unwrap();
        let x_lower = if w >= 0 { x } else { x - w as usize };
        let y_lower = if h >= 0 { y } else { y - h as usize };
        let x_upper = if w >= 0 { x + w as usize } else { x };
        let y_upper = if h >= 0 { y + h as usize } else { y };

        let lower: Point = (x_lower, y_lower).try_into().unwrap();
        let upper: Point = (x_upper, y_upper).try_into().unwrap();

        self.place_room(lower, upper);
    }

    /// Place a hallway between two points
    pub fn place_hallway(
        &mut self,
        point0: impl Into<Point>,
        point1: impl Into<Point>,
        route: RouteMethod,
    ) {
        let (x0, y0) = point0.into().into();
        let (x1, y1) = point1.into().into();

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
            }
            RouteMethod::VerticalFirst => {
                self.place_room((x0, y0), (x0, y1));
                self.place_room((x0, y1), (x1, y1));
            }
            _ => panic!("Found unsupported route!"),
        };
    }

    /// Find the nearest connected cell to the cell specified
    fn find_nearest_connected(&self, point: impl Into<Point>) -> Option<Point> {
        let (x, y) = point.into().into();
        self.find_by((x, y), &|cell: &Cell| -> bool { cell.is_room() })
    }

    /// Find a cell with an arbitrary condition. This function takes a starting
    /// point and searches for nearby cells that satisfy condition 'cond'. The
    /// condition is passed in in the form of a function that takes a gridcell
    /// and outputs a result containing a boolean stating whether the match has
    /// been made or not.
    fn find_by<F>(&self, point: impl Into<Point>, cond: &F) -> Option<Point>
    where
        F: Fn(&Cell) -> bool,
    {
        let (x, y): (usize, usize) = point.into().try_into().unwrap();
        let mut rooms: Vec<Point> = vec![];
        let mut found = false;
        let mut radius: usize = 0;

        while !found {
            radius += 1; // Increase search radius every loop
            let xmin = x.saturating_sub(radius); // Bounds xmin at 0
            let xmax = cmp::min(self.xmax - 1, x + radius); // Bounds xmax at self.xmax
            let ymin = y.saturating_sub(radius);
            let ymax = cmp::min(self.ymax - 1, y + radius);

            if xmin == 0 && ymin == 0 && xmax == self.xmax - 1 && ymax == self.ymax - 1 {
                // If this condition is true then we've searched the whole grid
                // and didn't find what we were looking for. Return None, which
                // indicates that no rooms were found.
                return None;
            }

            // Scan horizontal neighbours
            for i in xmin..xmax + 1 {
                if cond(&self.cells[i][ymin]) {
                    rooms.push((i, ymin).try_into().unwrap());
                    found = true;
                }
                if cond(&self.cells[i][ymax]) {
                    rooms.push((i, ymax).try_into().unwrap());
                    found = true;
                }
            }

            // Scan vertical neighbours
            for j in ymin..ymax + 1 {
                if cond(&self.cells[xmin][j]) {
                    rooms.push((xmin, j).try_into().unwrap());
                    found = true;
                }
                if cond(&self.cells[xmax][j]) {
                    rooms.push((xmax, j).try_into().unwrap());
                    found = true;
                }
            }
        }

        // Now pick a random room
        let mut rng = thread_rng();
        // If we found a room then we need to make a copy of the value
        // that's found there. x.choose() returns a reference and not
        // the value itself.
        rooms.choose(&mut rng).copied()
    }

    /// Generate random cells with a biasing towards more/less rooms. Limit is a value
    /// between 1 and 100. This limit sets the chance that the cells are a room.
    /// Higher limit means fewer rooms.
    pub fn generate_random_cells(&mut self, limit: i64) {
        let mut rng = thread_rng();
        for i in 0..self.xmax {
            for j in 0..self.ymax {
                let val = rng.gen_range(1..100);
                if val > limit {
                    self.cells[i][j].area = Area::Room;
                } else {
                    self.cells[i][j].area = Area::Nothing;
                }
            }
        }
    }

    pub fn generate_annealed_random_cells(&mut self) {
        // Start by generating a random grid
        self.generate_random_cells(80);

        // Anneal by removing stragglers
        for i in 1..self.xmax {
            for j in 1..self.ymax {
                let alone = self.cells[i - 1][j].is_empty()
                    && self.cells[i][j - 1].is_empty()
                    && self.cells[i + 1][j].is_empty()
                    && self.cells[i][j + 1].is_empty();
                if alone {
                    self.cells[i][j].area = Area::Nothing;
                }
            }
        }
    }

    /// Place a randomly sized room of up to scale length or width.
    pub fn place_random_room(&mut self, scale: usize, connect: bool) {
        // Initialize a random number generator
        let mut rng = rand::thread_rng();

        // Generate size of the room
        let width = rng.gen_range(2..scale);
        let height = rng.gen_range(2..scale);

        // Generate the origin (location) of the room
        let x0 = rng.gen_range(1..self.xmax);
        let y0 = rng.gen_range(1..self.ymax);
        let point0: Point = (x0, y0).try_into().unwrap();

        // See if we need to connect the room to an existing one.
        if connect {
            // Find the nearest connected location and return
            // the coordinates.
            let p1 = self
                .find_nearest_connected(point0)
                .expect("no existing rooms to connect");
            // Drow the hallway; some of this will be overwritten by
            // the room placement below.
            let p0: Point = (x0, y0).try_into().unwrap();
            self.place_hallway(p0, p1, RouteMethod::Manhattan);
        }

        // Set x/y min/max while checking for overflows on either
        // the lower or upper bounds.
        let xmin = x0.saturating_sub(width / 2);
        let ymin = y0.saturating_sub(height / 2);
        let xmax = cmp::min(self.xmax - 1, x0 + width / 2);
        let ymax = cmp::min(self.ymax - 1, y0 + width / 2);

        let min: Point = (xmin, ymin).try_into().unwrap();
        let max: Point = (xmax, ymax).try_into().unwrap();
        self.place_room(min, max);
    }

    pub fn generate_dungeon(&mut self, num_rooms: usize, room_size: usize) {
        self.clear();

        for _ in 0..num_rooms {
            self.place_random_room(room_size, false);
        }

        let mut rooms = self.partition_rooms();
        let mut distance = 36;

        while rooms.len() > 1 {
            for rooms_combo in rooms.iter().combinations(2) {
                let room0 = rooms_combo[0];
                let room1 = rooms_combo[1];

                if room0 == room1 {
                    continue;
                }

                let (cell0, cell1) = room0
                    .nearest_cells(room1)
                    .expect("finding nearest cells failed");

                if cell0.distance2(&cell1) < distance {
                    self.place_hallway(cell0, cell1, RouteMethod::Manhattan);
                }
            }

            rooms = self.partition_rooms();
            distance += 150;
        }
    }

    /// Determine if i, or j lies on an edge.
    fn on_edge(&self, i: usize, j: usize) -> bool {
        // Logic here is pretty simple: if either i or j is the max
        // value or 0 then they're on the edge of the map. All maps
        // are rectangular.
        i == 0 || i == self.xmax - 1 || j == 0 || j == self.ymax - 1
    }

    fn cave_anneal_cell(&self, i: usize, j: usize) -> bool {
        if !self.on_edge(i, j) {
            let mut neighbours = 0;

            // 3x3 grid
            for x in i - 1..i + 2 {
                for y in j - 1..j + 2 {
                    if self.cells[x][y].area == Area::Room {
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
                    tmp_map[i][j].area = Area::Room;
                } else {
                    tmp_map[i][j].area = Area::Nothing;
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
                let point: Point = (i, j).try_into().unwrap();
                size = self.get_room_size(point);
                if size < 15 {
                    self.clear_room(point);
                }
            }
        }
    }

    /// Get the size of the room (in number of cells) at a point.
    fn get_room_size(&self, point: impl Into<Point>) -> usize {
        let (i, j): (usize, usize) = point.into().try_into().unwrap();

        // Output value
        let mut size = 0;

        // Process all of the points in a list. Doing this in an iterative fashion because
        // of max recursion limits.
        let mut proc_queue = VecDeque::new();
        proc_queue.push_back((i, j));

        // Mask to make sure we don't revisit rooms
        let mut visited = vec![false; (self.xmax) * (self.ymax)];

        while !proc_queue.is_empty() {
            // Remove a room from the queue and unpack the values
            let (x, y) = proc_queue.pop_front().unwrap();

            // If we have visited this cell before or it is not a "room" then
            // we should stop processing and move on to the next
            if visited[x * self.xmax + y] || self.cells[x][y].area != Area::Room {
                continue;
            }

            // We got here so it's a room that we haven't visited. Mark it as visited now
            visited[x * self.xmax + y] = true;

            // This is a room and we haven't visited, so add one to size.
            size += 1;

            // Add all neighbours to the queue
            proc_queue.push_back((x - 1, y));
            proc_queue.push_back((x + 1, y));
            proc_queue.push_back((x, y - 1));
            proc_queue.push_back((x, y + 1));
        }
        // Returning the full size of the room
        size
    }

    fn clear_room(&mut self, point: impl Into<Point>) {
        let (x, y): (usize, usize) = point.into().try_into().unwrap();
        if self.cells[x][y].area == Area::Nothing {
            return;
        }

        let mut proc_queue = VecDeque::new();
        proc_queue.push_back((x, y));

        while !proc_queue.is_empty() {
            let (i, j) = proc_queue.pop_front().unwrap();

            if self.cells[i][j].area == Area::Nothing {
                continue;
            }

            self.cells[i][j].area = Area::Nothing;
            proc_queue.push_back((i + 1, j));
            proc_queue.push_back((i - 1, j));
            proc_queue.push_back((i, j + 1));
            proc_queue.push_back((i, j - 1));
        }
    }

    pub fn generate_cave(&mut self, iter: i64, seed_limit: i64) {
        // Makes a random selection of cells
        self.generate_random_cells(seed_limit);

        // Anneal the cells into blobs
        for _ in 0..iter {
            self.generate_cave_iteration();
        }

        // Get rid of small caves; reduces visual noise
        self.remove_orphans();

        // Connect caves together
        let caves = self.partition_rooms();

        // So many n^2 algos :-(
        for caves_combo in caves.iter().combinations(2) {
            let cave = caves_combo[0];
            let cave2 = caves_combo[1];

            let (cell1, cell2) = cave
                .nearest_cells(cave2)
                .expect("finding nearest cells failed");

            if cell1.distance2(&cell2) < 36 {
                self.place_hallway(cell1, cell2, RouteMethod::Manhattan);
            }
        }
    }

    fn partition_rooms(&self) -> Vec<Room> {
        self.partition_spaces(false)
    }

    /// Partition the map into groups of cells, called Rooms.
    ///
    /// The 'rooms' are just collections of cells of the same type, so there is
    /// at least one room that contains the walls/Nothing cells. These rooms
    /// can then be used for path processing or connectivity testing.
    fn partition_spaces(&self, include_nothing: bool) -> Vec<Room> {
        let mut out = Vec::new();

        // Make an set of the unvisited cells. Use this for finding new
        // locations
        let mut unvisited = HashSet::<(usize, usize)>::with_capacity(self.xmax * self.ymax);
        for i in 0..self.xmax {
            for j in 0..self.ymax {
                unvisited.insert((i, j));
            }
        }

        // Now keep looping until we've covered ever cell in the map and found
        // all of the rooms
        while !unvisited.is_empty() {
            // Each time, we start with an index; we don't really care what
            // it is so the unordered hashset works well here.
            let first_index = unvisited.iter().next().unwrap();
            let mut x = first_index.0;
            let mut y = first_index.1;
            let this_area_type = &self.cells[x][y].area;

            // This is going to be a 'room' (which includes contiguous AreaType::Nothing
            // spaces). Make a new one here that we're going to build up.
            let mut room = Room::new();

            // Need a queue for the flood algorithm. We're going to keep adding
            // to this queue until we run out of spaces to add to it.
            let mut proc_queue = VecDeque::new();
            // Our initial seed is the starting index that we got from 'unvisited'
            proc_queue.push_back((x, y));

            // The queue works on the concept that we can keep adding to it
            // whenever we find a new cell of our room but that we don't when
            // the new cell is not a part of our room. We flood from the initial
            // cell, meaning that we start at the starting index and then add
            // the index at the above/below/left/right of that cell. In the beginning
            // this fans out a lot, with massive expansion of the queue, but as
            // we begin to run into walls then we stop adding to the queue and
            // it begins to drain.
            while !proc_queue.is_empty() {
                // pop_front makes this a FIFO. Doesn't really matter, we could
                // have done it another way.
                let index = proc_queue.pop_front().unwrap();
                x = index.0;
                y = index.1;

                if x >= self.xmax || y >= self.ymax {
                    // If the value is too big then just continue. The value
                    // has already been removed from the queue and it doesn't
                    // need to be removed from unvisited since it's outside
                    // the map area.
                    continue;
                }

                if self.cells[x][y].area != *this_area_type {
                    // Check that the cell is the correct type. If it is, then continue
                    // with processing it, otherwise don't remove it from the unvisited
                    // list (since we still might need to visit it).
                    continue;
                }

                if !unvisited.remove(&index) {
                    // HashSet.remove() returns a bool that's true if the value
                    // was in the set. In this case that tells us if we've been
                    // here before. If we have, then don't do any further processing.
                    continue;
                }

                // If we've got here then the cell is a part of our room because
                // it has the correct type and we haven't processed it yet. Add
                // it too our room and then add the immediate nearest neighbours
                // to our processing queue.
                let index: Point = index.try_into().unwrap();
                room.add_cell(index).expect("failed to add cell");
                proc_queue.push_back((x + 1, y));
                proc_queue.push_back((x.saturating_sub(1), y));
                proc_queue.push_back((x, y + 1));
                proc_queue.push_back((x, y.saturating_sub(1)));
            }
            // The room is now complete; add it to our output vector and forget
            // about this particular room.
            if *this_area_type != Area::Nothing || include_nothing {
                out.push(room);
            }
        }

        // Room processing is done. Return.
        out
    }

    /// Delete everything in this map and reset to nothing
    fn clear(&mut self) {
        for x in 0..self.xmax {
            for y in 0..self.ymax {
                self.cells[x][y].area = Area::Nothing;
            }
        }
    }
}

impl Index<Point> for GridMap {
    type Output = Cell;

    fn index(&self, index: Point) -> &Self::Output {
        let (x, y): (usize, usize) = index.try_into().unwrap();
        &self.cells[x][y]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Ensure that regenerating halls multiple times doesn't hang
    #[test]
    fn regenerate_dungeon() {
        let mut map = GridMap::new(25, 25);

        // This used to fail due to an infinite loop in the halls algorithm.
        for _ in 0..10 {
            map.generate_dungeon(10, 10);
        }
    }
}
