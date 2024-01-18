use crate::render::Relative;

const PADDING_IN_HORIZONT: u32 = 8;

#[derive(Debug)]
pub enum Align {
    _Left,
    Right,
}
#[derive(Debug)]
pub struct Button {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub label: String,
    pub padding: i32,
    pub id: String,
    pub align: Align,
}

impl Button {
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
                self.x - (self.w + (PADDING_IN_HORIZONT / 2) as i32),
                self.y + 2,
            ),
        }
    }
    pub fn get_coors_with_zoom(&self, relative: &Relative) -> (i32, i32) {
        match self.align {
            Align::_Left => (relative.zoom(self.x), relative.zoom(self.y)),
            Align::Right => (
                relative.zoom(self.x) - (self.w + (PADDING_IN_HORIZONT / 2) as i32),
                relative.zoom(self.y + 2),
            ),
        }
    }

    pub fn calc(&mut self, context: &mut web_sys::CanvasRenderingContext2d, _relative: &Relative) {
        context.set_text_baseline("middle");
        context.set_font("12px serif");
        let w = if let Ok(metric) = context.measure_text(&self.label) {
            metric.width()
        } else {
            64.0
        };
        self.w = w as i32 + PADDING_IN_HORIZONT as i32;
        self.h = 18;
    }

    pub fn render(&mut self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        context.set_text_baseline("middle");
        context.set_font("12px serif");
        let w = if let Ok(metric) = context.measure_text(&self.label) {
            metric.width()
        } else {
            64.0
        };
        let mut x = relative.x(self.x) as f64;
        let y = (relative.y(self.y) + self.padding) as f64;
        if matches!(self.align, Align::Right) {
            x -= w + PADDING_IN_HORIZONT as f64 + self.padding as f64;
        } else {
            x += self.padding as f64;
        }
        context.fill_rect(x, y, w + PADDING_IN_HORIZONT as f64, 18.0);
        let _ = context.stroke_text(&self.label, x + 3.0, y + 12.0);
        self.w = w as i32 + PADDING_IN_HORIZONT as i32;
        self.h = 18;
    }
}
