
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
    // Accessable_t (Python) not needed in Rust. We'll use an
    // Option<bool> to represent that.
    accessable: Option<bool>,
    pub area: AreaType,
    vert_wall: WallType,
    horiz_wall: WallType,
    point: PointType,
}

impl GridCell {
    /// Construct a new GridCell
    pub fn new() -> GridCell {
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
    pub fn is_empty(&self) -> bool {
        match self {
            &GridCell { area: AreaType::Nothing,
                       vert_wall: WallType::Nothing,
                       horiz_wall: WallType::Nothing,
                       point: PointType::Nothing,
                       .. } => true,
            _ => false
        }
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
        match self {
            &GridCell { area: AreaType::Room, ..} => true,
            &GridCell { area: AreaType::Entrance, .. } => true,
            _ => false
        }
    }
}
