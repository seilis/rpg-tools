use std::collections::HashSet;

use super::point::Point;
use crate::error::{Result, RpgError};

/// A representation of a room made up of GridCells
///
/// The GridRoom represents a set of cells that are all connected together. These
/// form a shape on the map that may or may not be connected to other rooms or
/// features.
///
/// Currently the GridRoom represents the cells by a set of indexes.
#[derive(Debug, PartialEq)]
pub struct Room {
    cells: HashSet<Point>,
    connected: bool,
}

impl Room {
    /// Make a new GridRoom
    pub fn new() -> Room {
        Room {
            cells: HashSet::new(),
            connected: false,
        }
    }

    /// Add an index to a cell that's part of the GridRoom
    pub fn add_cell(&mut self, point: impl Into<Point>) -> Result<bool> {
        Ok(self.cells.insert(point.into()))
    }

    /// Remove an index from the GridRoom
    pub fn remove_cell(&mut self, point: impl Into<Point>) -> Result<bool> {
        Ok(self.cells.remove(&point.into()))
    }

    /// Test whether the GridRoom is empty (contains no indexes/cells)
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn nearest_cells(&self, other: &Room) -> Result<(Point, Point)> {
        if self.is_empty() {
            return Err(RpgError::Empty(
                "our room is empty; no nearest cells".to_string(),
            ));
        } else if other.is_empty() {
            return Err(RpgError::Empty(
                "their room is empty; no nearest cells".to_string(),
            ));
        }

        let mut best_distance = u64::MAX;
        let mut our_closest = Point::default();
        let mut their_closest = Point::default();

        // There must be a better way than n^2...
        for our_cell in self.iter_cells() {
            for their_cell in other.iter_cells() {
                // Bunch of casting since we've used usize for the indices but
                // the internal subtraction can be negative.
                let this_distance = our_cell.distance2(their_cell);

                if this_distance > best_distance {
                    continue;
                }

                best_distance = this_distance;
                our_closest = *our_cell;
                their_closest = *their_cell;
            }
        }

        Ok((our_closest, their_closest))
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = &Point> {
        self.cells.iter()
    }
}

impl Default for Room {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let room = Room::new();

        assert_eq!(0, room.iter_cells().count());
    }

    #[test]
    fn nearest_square() {
        let mut r1 = Room::new();
        let mut r2 = Room::new();

        // r1 is a square 0,0 -> 1,1
        r1.add_cell((0, 0)).unwrap();
        r1.add_cell((0, 1)).unwrap();
        r1.add_cell((1, 0)).unwrap();
        r1.add_cell((1, 1)).unwrap();

        // r2 is a square on the diagonal @ 3,3 -> 4,4
        r2.add_cell((3, 3)).unwrap();
        r2.add_cell((3, 4)).unwrap();
        r2.add_cell((4, 3)).unwrap();
        r2.add_cell((4, 4)).unwrap();

        // Closest should be 1,1 and 3,3
        assert_eq!(
            (Point::new(1, 1), Point::new(3, 3)),
            r1.nearest_cells(&r2).unwrap()
        )
    }
}
