#[derive(Debug)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug)]
pub struct Path {
    pub points: Vec<Point>,
}
