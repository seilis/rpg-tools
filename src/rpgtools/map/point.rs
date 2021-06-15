
pub struct Point2D<T> {
    x: T,
    y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Point2D<T> {
        Point2D { x, y }
    }
}
