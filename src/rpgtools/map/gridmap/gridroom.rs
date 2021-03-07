use std::collections::HashSet;

/// A representation of a room made up of GridCells
///
/// The GridRoom represents a set of cells that are all connected together. These
/// form a shape on the map that may or may not be connected to other rooms or
/// features.
///
/// Currently the GridRoom represents the cells by a set of indexes.
#[derive(Debug, PartialEq)]
pub struct GridRoom {
    cells: HashSet<(usize, usize)>,
    connected: bool,
}

impl GridRoom {
    /// Make a new GridRoom
    pub fn new() -> GridRoom {
        GridRoom {
            cells: HashSet::new(),
            connected: false,
        }
    }

    /// Add an index to a cell that's part of the GridRoom
    pub fn add_cell(&mut self, (x, y): &(usize, usize)) -> Result<bool, String> {
        Ok(self.cells.insert((*x, *y)))
    }

    /// Remove an index from the GridRoom
    pub fn remove_cell(&mut self, (x, y): &(usize, usize)) -> Result<bool, String> {
        Ok(self.cells.remove(&(*x, *y)))
    }

    /// Test whether the GridRoom is empty (contains no indexes/cells)
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn nearest_cells(
        &self,
        other: &GridRoom,
    ) -> Result<((usize, usize), (usize, usize)), String> {
        if self.is_empty() {
            return Err("our room is empty; no nearest cells".to_string());
        } else if other.is_empty() {
            return Err("their room is empty; no nearest cells".to_string());
        }

        let mut best_distance = std::usize::MAX;
        let mut our_closest = (0, 0);
        let mut their_closest = (0, 0);

        // There must be a better way than n^2...
        for our_cell in self.iter_cells() {
            for their_cell in other.iter_cells() {
                // Bunch of casting since we've used usize for the indices but
                // the internal subtraction can be negative.
                let this_distance = ((our_cell.0 as isize - their_cell.0 as isize).pow(2)
                    + (our_cell.1 as isize - their_cell.1 as isize).pow(2))
                    as usize;

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

    pub fn iter_cells(&self) -> std::collections::hash_set::Iter<'_, (usize, usize)> {
        self.cells.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let room = GridRoom::new();

        assert_eq!(0, room.iter_cells().count());
    }

    #[test]
    fn nearest_square() {
        let mut r1 = GridRoom::new();
        let mut r2 = GridRoom::new();

        // r1 is a square 0,0 -> 1,1
        r1.add_cell(&(0, 0)).unwrap();
        r1.add_cell(&(0, 1)).unwrap();
        r1.add_cell(&(1, 0)).unwrap();
        r1.add_cell(&(1, 1)).unwrap();

        // r2 is a square on the diagonal @ 3,3 -> 4,4
        r2.add_cell(&(3, 3)).unwrap();
        r2.add_cell(&(3, 4)).unwrap();
        r2.add_cell(&(4, 3)).unwrap();
        r2.add_cell(&(4, 4)).unwrap();

        // Closest should be 1,1 and 3,3
        assert_eq!(((1, 1), (3, 3)), r1.nearest_cells(&r2).unwrap())
    }
}
