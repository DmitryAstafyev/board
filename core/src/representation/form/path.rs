#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Path {
    pub points: Vec<Point>,
}

impl Path {
    pub fn box_height(&self) -> i32 {
        if let (Some(min), Some(max)) = (
            self.points.iter().map(|p| p.y).min(),
            self.points.iter().map(|p| p.y).max(),
        ) {
            if max > min {
                max - min
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn box_width(&self) -> i32 {
        if let (Some(min), Some(max)) = (
            self.points.iter().map(|p| p.x).min(),
            self.points.iter().map(|p| p.x).max(),
        ) {
            if max > min {
                max - min
            } else {
                0
            }
        } else {
            0
        }
    }
}
