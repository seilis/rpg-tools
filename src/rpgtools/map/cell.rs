use super::Area;

#[derive(Clone, Debug)]
pub enum Wall {
    Nothing,
    //    Wall,
    //    Door,
    //    SecretDoor,
}

#[derive(Clone, Debug)]
pub enum Point {
    Nothing,
    //    Pillar,
}

/// Representation of a GridCell, which is a single unit in a grid.
#[derive(Clone, Debug)]
pub struct Cell {
    /// The type of area contained within this sell
    pub area: Area,
    vert_wall: Wall,
    horiz_wall: Wall,
    point: Point,
}

impl Cell {
    /// Construct a new Cell
    pub fn new() -> Cell {
        Cell {
            area: Area::Nothing,
            vert_wall: Wall::Nothing,
            horiz_wall: Wall::Nothing,
            point: Point::Nothing,
        }
    }

    /// Check if a GridCell is 'empty', meaning that all drawing-related
    /// fields have a value of 'Nothing'.
    pub fn is_empty(&self) -> bool {
        matches!(
            *self,
            Cell {
                area: Area::Nothing,
                vert_wall: Wall::Nothing,
                horiz_wall: Wall::Nothing,
                point: Point::Nothing,
                ..
            }
        )
    }

    //    /// Check whether the GridCell represents a dungeon entrance
    //    pub fn is_entrance(&self) -> bool {
    //        match self {
    //            &GridCell { area: AreaType::Entrance, ..} => true,
    //            _ => false
    //        }
    //    }

    /// Check whether the GridCell is part of a dungeon room
    pub fn is_room(&self) -> bool {
        !matches!(
            *self,
            Cell {
                area: Area::Nothing,
                ..
            }
        )
    }

    /// Get the type of area that this cell represents
    pub fn area(&self) -> &Area {
        &self.area
    }

    /// Set the type of area that this cell represents
    pub fn set_area(&mut self, area: Area) {
        self.area = area;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let cell = Cell::new();
        assert_eq!(Area::Nothing, cell.area);
    }

    #[test]
    fn is_empty() {
        let mut cell = Cell::new();
        // Do checks
        cell.area = Area::Nothing;
        assert_eq!(true, cell.is_empty());
        cell.area = Area::Entrance;
        assert_eq!(false, cell.is_empty());
        cell.area = Area::Room;
        assert_eq!(false, cell.is_empty());
    }

    #[test]
    fn is_room() {
        let mut cell = Cell::new();
        // Do checks
        cell.area = Area::Nothing;
        assert_eq!(false, cell.is_room());
        cell.area = Area::Entrance;
        assert_eq!(true, cell.is_room());
        cell.area = Area::Room;
        assert_eq!(true, cell.is_room());
    }
}
