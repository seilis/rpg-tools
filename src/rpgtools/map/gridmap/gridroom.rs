use std::collections::HashSet;

/// A representation of a room made up of GridCells
///
/// The GridRoom represents a set of cells that are all connected together. These
/// form a shape on the map that may or may not be connected to other rooms or
/// features.
///
/// Currently the GridRoom represents the cells by a set of indexes.
#[derive(Debug)]
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
}
