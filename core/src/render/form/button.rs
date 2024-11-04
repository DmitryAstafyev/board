use wasm_bindgen::JsValue;

use crate::render::{Ratio, Relative};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    pub pad_hor: i32,
    pub r_off: i32,
    pub min_h: i32,
    pub min_w: i32,
    pub x_off: i32,
    pub y_off: i32,
    pub f_size: i32,
}

impl Params {
    pub fn new(ratio: &Ratio) -> Self {
        Self {
            pad_hor: ratio.get(8),
            r_off: ratio.get(2),
            min_h: ratio.get(18),
            min_w: ratio.get(64),
            x_off: ratio.get(3),
            y_off: ratio.get(12),
            f_size: ratio.get(12),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Align {
    _Left,
    #[allow(dead_code)]
    Right,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Button {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub font: String,
    pub label: String,
    pub padding: i32,
    pub id: String,
    pub align: Align,
    pub params: Params,
}

impl Button {
    #[allow(clippy::too_many_arguments)]
    #[allow(dead_code)]
    pub fn new(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        font: String,
        label: String,
        padding: i32,
        id: String,
        align: Align,
        ratio: &Ratio,
    ) -> Self {
        let params = Params::new(ratio);
        Self {
            x,
            y,
            w,
            h,
            font,
            label,
            padding: ratio.get(padding),
            id,
            align,
            params,
        }
    }
    pub fn get_box_size(&self) -> (i32, i32) {
        (self.w, self.h)
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
        match self.align {
            Align::_Left => (self.x, self.y),
            Align::Right => (
                self.x - (self.w + (self.params.pad_hor / 2)),
                self.y + self.params.r_off,
            ),
        }
    }
    pub fn get_coors_with_zoom(&self, relative: &Relative) -> (i32, i32) {
        match self.align {
            Align::_Left => (relative.zoom(self.x), relative.zoom(self.y)),
            Align::Right => (
                relative.zoom(self.x) - (self.w + (self.params.pad_hor / 2)),
                relative.zoom(self.y + self.params.r_off),
            ),
        }
    }

    pub fn calc(&mut self, context: &mut web_sys::CanvasRenderingContext2d, _relative: &Relative) {
        context.set_text_baseline("middle");
        context.set_font(&format!("{}px {}", self.params.f_size, self.font));
        let w = if let Ok(metric) = context.measure_text(&self.label) {
            metric.width()
        } else {
            self.params.min_w as f64
        };
        self.w = w as i32 + self.params.pad_hor;
        self.h = self.params.min_h;
    }

    pub fn render(&mut self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        context.set_text_baseline("middle");
        context.set_font(&format!("{}px {}", self.params.f_size, self.font));
        let w = if let Ok(metric) = context.measure_text(&self.label) {
            metric.width()
        } else {
            self.params.min_w as f64
        };
        let mut x = relative.x(self.x) as f64;
        let y = (relative.y(self.y) + self.padding) as f64;
        if matches!(self.align, Align::Right) {
            x -= w + self.params.pad_hor as f64 + self.padding as f64;
        } else {
            x += self.padding as f64;
        }
        context.fill_rect(
            x,
            y,
            w + self.params.pad_hor as f64,
            self.params.min_h as f64,
        );
        context.set_fill_style(&JsValue::from_str("rgb(0,0,0)"));
        let _ = context.fill_text(
            &self.label,
            x + self.params.x_off as f64,
            y + self.params.y_off as f64,
        );
        self.w = w as i32 + self.params.pad_hor;
        self.h = self.params.min_h;
    }
}
