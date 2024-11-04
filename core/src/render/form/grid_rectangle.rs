use crate::render::{grid, Ratio, Relative};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    pub cell: u32,
}

impl Params {
    pub fn new(ratio: &Ratio) -> Self {
        Self {
            cell: ratio.get(grid::CELL),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GridRectangle {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub cells: (u32, u32),
    pub id: String,
    pub params: Params,
}

impl GridRectangle {
    fn sync(&mut self) {
        let mut cells: (u32, u32) = (
            self.w as u32 / self.params.cell,
            self.h as u32 / self.params.cell,
        );
        if cells.0 == 0 {
            cells.0 = 1
        }
        if cells.1 == 0 {
            cells.1 = 1
        }
        if cells.0 * self.params.cell < self.w as u32 {
            cells.0 += 1;
        }
        if cells.1 * self.params.cell < self.h as u32 {
            cells.1 += 1
        }
        self.w = (cells.0 * self.params.cell) as i32;
        self.h = (cells.1 * self.params.cell) as i32;
        self.cells = cells;
    }
    pub fn new(id: String, x: i32, y: i32, w: i32, h: i32, ratio: &Ratio) -> Self {
        let params = Params::new(ratio);
        let mut instance = Self {
            x,
            y,
            w,
            h,
            cells: (1, 1),
            id,
            params,
        };
        instance.sync();
        instance
    }
    pub fn get_box_size(&self) -> (i32, i32) {
        (self.w, self.h)
    }
    pub fn set_box_size(&mut self, w: Option<i32>, h: Option<i32>) {
        if let Some(w) = w {
            self.w = w;
        }
        if let Some(h) = h {
            self.h = h;
        }
        self.sync();
    }
    pub fn set_coors(&mut self, x: Option<i32>, y: Option<i32>) {
        if let Some(x) = x {
            self.x = x;
        }
        if let Some(y) = y {
            self.y = y;
        }
    }
    pub fn get_coors(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    pub fn render(&self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        context.fill_rect(
            relative.x(self.x) as f64,
            relative.y(self.y) as f64,
            relative.zoom(self.w) as f64,
            relative.zoom(self.h) as f64,
        );
        context.stroke_rect(
            relative.x(self.x) as f64,
            relative.y(self.y) as f64,
            relative.zoom(self.w) as f64,
            relative.zoom(self.h) as f64,
        );
    }
}
