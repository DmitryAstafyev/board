use crate::render::{grid::CELL, Relative};

const TEXT_PADDING: u32 = 3;

#[derive(Debug)]
pub enum Align {
    Left,
    Right,
}
#[derive(Debug)]
pub struct Label {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub label: String,
    pub padding: i32,
    pub id: String,
    pub align: Align,
}

impl Label {
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
            Align::Left => (self.x + self.padding, self.y),
            Align::Right => (self.x - self.w - self.padding, self.y),
        }
    }
    pub fn get_coors_with_zoom(&self, relative: &Relative) -> (i32, i32) {
        match self.align {
            Align::Left => (relative.zoom(self.x + self.padding), relative.zoom(self.y)),
            Align::Right => (
                relative.zoom(self.x - self.padding) - self.w,
                relative.zoom(self.y),
            ),
        }
    }
    // Take into account self.w already condiser zooming, because it's calculated by
    // render and already reflects zoom-factor.
    pub fn render(&mut self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        let text_padding = relative.zoom(TEXT_PADDING as i32) as f64;
        self.w = if let Ok(metric) = context.measure_text(&self.label) {
            metric.width() as i32
        } else {
            64
        } + (text_padding as i32) * 2;
        self.h = relative.zoom((CELL as f64 * 0.7).floor() as i32);
        let x = match self.align {
            Align::Left => relative.x(self.x + self.padding),
            Align::Right => relative.x(self.x - self.padding) - self.w,
        } as f64;
        let y = relative.y(self.y) as f64;
        context.set_text_baseline("top");
        context.set_font(&format!("{}px serif", self.h - 6));
        context.fill_rect(x, y, self.w as f64, self.h as f64);
        context.stroke_rect(x, y, self.w as f64, self.h as f64);
        let _ = context.stroke_text(&self.label, x + text_padding, y + text_padding);
    }
}
