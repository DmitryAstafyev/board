use std::f64::consts::PI;

use crate::render::Relative;

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Path {
    pub points: Vec<Point>,
    pub id: String,
}

impl Path {
    pub fn new(id: String, points: Vec<Point>) -> Self {
        Self { id, points }
    }
    pub fn get_box_size(&self) -> (i32, i32) {
        (
            {
                let w = self.points.iter().map(|p| p.x).max().unwrap_or(0)
                    - self.points.iter().map(|p| p.x).min().unwrap_or(0);
                if w < 0 {
                    0
                } else {
                    w
                }
            },
            {
                let h = self.points.iter().map(|p| p.y).max().unwrap_or(0)
                    - self.points.iter().map(|p| p.y).min().unwrap_or(0);
                if h < 0 {
                    0
                } else {
                    h
                }
            },
        )
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
        context.begin_path();
        let _ = context.ellipse(
            relative.x(self.points[0].x) as f64,
            relative.y(self.points[0].y) as f64,
            3.0,
            3.0,
            0.0,
            0.0,
            360.0 * (PI / 180.0),
        );
        context.fill();
        context.begin_path();
        let _ = context.ellipse(
            relative.x(self.points[self.points.len() - 1].x) as f64,
            relative.y(self.points[self.points.len() - 1].y) as f64,
            3.0,
            3.0,
            0.0,
            0.0,
            360.0 * (PI / 180.0),
        );
        context.fill();
    }
}
