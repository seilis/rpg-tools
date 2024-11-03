#[derive(Clone, Debug, PartialEq)]
pub enum AreaType {
    Nothing,
    Entrance,
    Room,
    //    Stairs,
    //    Tested,
}

#[derive(Clone, Debug)]
pub enum WallType {
    Nothing,
    //    Wall,
    //    Door,
    //    SecretDoor,
}

#[derive(Clone, Debug)]
pub enum PointType {
    Nothing,
    //    Pillar,
}

/// Representation of a GridCell, which is a single unit in a grid.
#[derive(Clone, Debug)]
pub struct GridCell {
    /// The type of area contained within this sell
    pub area: AreaType,
    vert_wall: WallType,
    horiz_wall: WallType,
    point: PointType,
}

impl GridCell {
    /// Construct a new GridCell
    pub fn new() -> GridCell {
        GridCell {
            area: AreaType::Nothing,
            vert_wall: WallType::Nothing,
            horiz_wall: WallType::Nothing,
            point: PointType::Nothing,
        }
    }

    /// Check if a GridCell is 'empty', meaning that all drawing-related
    /// fields have a value of 'Nothing'.
    pub fn is_empty(&self) -> bool {
        matches!(
            *self,
            GridCell {
                area: AreaType::Nothing,
                vert_wall: WallType::Nothing,
                horiz_wall: WallType::Nothing,
                point: PointType::Nothing,
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
            GridCell {
                area: AreaType::Nothing,
                ..
            }
        )
    }

    /// Get the type of area that this cell represents
    pub fn area(&self) -> &AreaType {
        &self.area
    }
}

impl Default for GridCell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let cell = GridCell::new();
        assert_eq!(AreaType::Nothing, cell.area);
    }

    #[test]
    fn is_empty() {
        let mut cell = GridCell::new();
        // Do checks
        cell.area = AreaType::Nothing;
        assert_eq!(true, cell.is_empty());
        cell.area = AreaType::Entrance;
        assert_eq!(false, cell.is_empty());
        cell.area = AreaType::Room;
        assert_eq!(false, cell.is_empty());
    }

    #[test]
    fn is_room() {
        let mut cell = GridCell::new();
        // Do checks
        cell.area = AreaType::Nothing;
        assert_eq!(false, cell.is_room());
        cell.area = AreaType::Entrance;
        assert_eq!(true, cell.is_room());
        cell.area = AreaType::Room;
        assert_eq!(true, cell.is_room());
    }
}
