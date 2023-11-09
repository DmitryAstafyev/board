use crate::elements::relative::Relative;

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

    pub fn render(&self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        if self.points.is_empty() {
            return;
        }
        context.begin_path();
        context.move_to(
            relative.x(self.points[0].x) as f64,
            relative.y(self.points[0].y) as f64,
        );
        self.points.iter().for_each(|p| {
            context.line_to(relative.x(p.x) as f64, relative.y(p.y) as f64);
        });
        context.stroke();
    }
}
