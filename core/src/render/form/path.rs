use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use crate::render::{Ratio, Relative};

#[derive(Debug)]
pub struct Params {
    pub radius: u32,
}

impl Params {
    pub fn new(ratio: &Ratio) -> Self {
        Self {
            radius: ratio.get(3),
        }
    }
}
#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Path {
    pub points: Vec<Point>,
    pub id: String,
    pub params: Params,
    pub sdot: bool,
    pub edot: bool,
    pub sarrow: bool,
    pub earrow: bool,
}

impl Path {
    pub fn new(id: String, points: Vec<Point>, ratio: &Ratio) -> Self {
        Self {
            id,
            points,
            params: Params::new(ratio),
            sdot: false,
            edot: false,
            sarrow: false,
            earrow: false,
        }
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
        fn arrow(
            ctx: &mut CanvasRenderingContext2d,
            x1: f64,
            y1: f64,
            x2: f64,
            y2: f64,
            head_len: f64,
        ) {
            let angle = (y2 - y1).atan2(x2 - x1);

            ctx.begin_path();
            ctx.move_to(x2, y2);
            ctx.line_to(
                x2 - head_len * (angle - std::f64::consts::PI / 6.0).cos(),
                y2 - head_len * (angle - std::f64::consts::PI / 6.0).sin(),
            );
            ctx.line_to(
                x2 - head_len * (angle + std::f64::consts::PI / 6.0).cos(),
                y2 - head_len * (angle + std::f64::consts::PI / 6.0).sin(),
            );
            ctx.close_path();
            ctx.fill();
        }
        fn dot(ctx: &mut CanvasRenderingContext2d, x: f64, y: f64, r: f64) {
            ctx.begin_path();
            let _ = ctx.ellipse(x, y, r, r, 0.0, 0.0, 360.0 * (PI / 180.0));
            ctx.fill();
        }
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
        if self.sdot || self.sarrow {
            if self.sdot {
                dot(
                    context,
                    relative.x(self.points[self.points.len() - 1].x) as f64,
                    relative.y(self.points[self.points.len() - 1].y) as f64,
                    self.params.radius as f64 * relative.get_zoom(),
                );
            } else if self.sarrow {
                arrow(
                    context,
                    relative.x(self.points[0].x) as f64,
                    relative.y(self.points[0].y) as f64,
                    relative.x(self.points[self.points.len() - 1].x) as f64,
                    relative.y(self.points[self.points.len() - 1].y) as f64,
                    self.params.radius as f64 * 2.0 * relative.get_zoom(),
                );
            }
        }
        if self.edot || self.earrow {
            if self.edot {
                dot(
                    context,
                    relative.x(self.points[0].x) as f64,
                    relative.y(self.points[0].y) as f64,
                    self.params.radius as f64 * relative.get_zoom(),
                );
            } else if self.earrow {
                arrow(
                    context,
                    relative.x(self.points[self.points.len() - 1].x) as f64,
                    relative.y(self.points[self.points.len() - 1].y) as f64,
                    relative.x(self.points[0].x) as f64,
                    relative.y(self.points[0].y) as f64,
                    self.params.radius as f64 * 2.0 * relative.get_zoom(),
                );
            }
        }
    }
}
