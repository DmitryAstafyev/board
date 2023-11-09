use crate::{
    elements::relative::{self, Relative},
    error::E,
};

#[derive(Debug)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rectangle {
    pub fn box_height(&self) -> i32 {
        self.h
    }
    pub fn box_width(&self) -> i32 {
        self.w
    }
    pub fn set_box_height(&mut self, h: i32) {
        self.h = h;
    }
    pub fn set_box_width(&mut self, w: i32) {
        self.w = w;
    }

    pub fn set_coors(&mut self, x: Option<i32>, y: Option<i32>) {
        if let Some(x) = x {
            self.x = x;
        }
        if let Some(y) = y {
            self.y = y;
        }
    }

    pub fn relative(&self) -> Relative {
        Relative::new(self.x, self.y)
    }

    pub fn render(&self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        context.fill_rect(
            relative.x(self.x) as f64,
            relative.y(self.y) as f64,
            self.w as f64,
            self.h as f64,
        );
        // context.stroke();
    }
}
