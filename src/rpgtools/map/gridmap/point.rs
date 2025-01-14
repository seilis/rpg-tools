//! A point in a grid
use std::ops::{Add, Neg, Sub};

/// A point in a grid, represented by a pair of integers
///
/// This struct represents a specific point in a grid that may include cells at negative integers
/// (i.e. the origin may not be at the edge of the map).
///
/// Note that this type is just an index and therefore implements both Clone and Copy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    /// Make a new point, centred at the origin
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    /// Calculate the distance between two points in the normal way
    ///
    /// Returns the euclidean distance between two points.
    pub fn distance(&self, other: &Self) -> f64 {
        (self.distance2(other) as f64).sqrt()
    }

    /// Calculate the square of the distance between two points
    ///
    /// Calculate the square of the euclidean distance, which is sometimes useful as an optimized
    /// number for comparisons between distances. I.e.  if you only wish to see which distance is
    /// greater then you can save a sqrt() operation, which can be expensive in some contexts.
    pub fn distance2(&self, other: &Self) -> u64 {
        let x = self.x - other.x;
        let y = self.y - other.y;

        (x * x + y * y) as u64
    }

    pub fn is_in_bounds(&self, lower: Point, upper: Point) -> bool {
        self.x >= lower.x && self.x <= upper.x && self.y >= lower.y && self.y <= upper.y
    }
}

/// Points can be added together, as vectors.
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Points can be negated, as vectors.
impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// Points can be subtracted, as vectors.
impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Point> for (i64, i64) {
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0 as i64,
            y: value.1 as i64,
        }
    }
}

impl TryFrom<Point> for (i32, i32) {
    type Error = crate::error::RpgError;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        let x = value.x.try_into()?;
        let y = value.y.try_into()?;

        Ok((x, y))
    }
}

impl From<(i16, i16)> for Point {
    fn from(value: (i16, i16)) -> Self {
        Self {
            x: value.0 as i64,
            y: value.1 as i64,
        }
    }
}

impl TryFrom<Point> for (i16, i16) {
    type Error = crate::error::RpgError;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        let x = value.x.try_into()?;
        let y = value.y.try_into()?;

        Ok((x, y))
    }
}

impl From<(i8, i8)> for Point {
    fn from(value: (i8, i8)) -> Self {
        Self {
            x: value.0 as i64,
            y: value.1 as i64,
        }
    }
}

impl TryFrom<Point> for (i8, i8) {
    type Error = crate::error::RpgError;
    fn try_from(value: Point) -> Result<Self, Self::Error> {
        let x = value.x.try_into()?;
        let y = value.y.try_into()?;

        Ok((x, y))
    }
}

impl TryFrom<(u64, u64)> for Point {
    type Error = crate::error::RpgError;

    fn try_from(value: (u64, u64)) -> Result<Self, Self::Error> {
        let x = value.0.try_into()?;
        let y = value.1.try_into()?;

        Ok(Point::new(x, y))
    }
}

impl TryFrom<(usize, usize)> for Point {
    type Error = crate::error::RpgError;

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        let x = value.0.try_into()?;
        let y = value.1.try_into()?;

        Ok(Point::new(x, y))
    }
}

impl TryFrom<Point> for (usize, usize) {
    type Error = crate::error::RpgError;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        let x = value.x.try_into()?;
        let y = value.y.try_into()?;

        Ok((x, y))
    }
}

impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0 as i64,
            y: value.1 as i64,
        }
    }
}

impl TryFrom<Point> for (u32, u32) {
    type Error = crate::error::RpgError;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        let x = value.x.try_into()?;
        let y = value.y.try_into()?;

        Ok((x, y))
    }
}

impl From<(u16, u16)> for Point {
    fn from(value: (u16, u16)) -> Self {
        Self {
            x: value.0 as i64,
            y: value.1 as i64,
        }
    }
}

impl TryFrom<Point> for (u16, u16) {
    type Error = crate::error::RpgError;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        let x = value.x.try_into()?;
        let y = value.y.try_into()?;

        Ok((x, y))
    }
}

impl From<(u8, u8)> for Point {
    fn from(value: (u8, u8)) -> Self {
        Self {
            x: value.0 as i64,
            y: value.1 as i64,
        }
    }
}

impl TryFrom<Point> for (u8, u8) {
    type Error = crate::error::RpgError;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        let x = value.x.try_into()?;
        let y = value.y.try_into()?;

        Ok((x, y))
    }
}

impl Default for Point {
    /// The default for a Point is at the origin (0, 0).
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A new point should be at the origin and adding a point to the origin
    // should be equal to a new point at the origin
    #[test]
    fn test_add_origin() {
        assert_eq!(Point::new(0, 0) + Point::new(0, 0), Point::new(0, 0));
    }

    // A point that has zero added to it should be the same
    #[test]
    fn test_add_zero() {
        assert_eq!(Point::new(3, 5) + Point::new(0, 0), Point::new(3, 5));
    }

    // Points should add in the normal vector way
    #[test]
    fn test_add_two_points() {
        assert_eq!(Point::new(3, 5) + Point::new(7, 9), Point::new(10, 14));
        assert_eq!(Point::new(3, 5) + Point::new(-2, -14), Point::new(1, -9));
    }

    // Points should subtract in the normal vector way
    #[test]
    fn test_subtract_two_points() {
        assert_eq!(Point::new(3, 5) - Point::new(7, 9), Point::new(-4, -4));
        assert_eq!(Point::new(3, 5) - Point::new(-2, -14), Point::new(5, 19));
    }

    #[test]
    fn test_distance_between_points() {
        // Simplified threshold detection
        assert!(Point::new(3, 5).distance(&Point::new(4, 4)) - 2.0f64.sqrt() < 0.000001);
    }
}
