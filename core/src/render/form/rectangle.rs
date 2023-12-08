use crate::render::Relative;

#[derive(Debug)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub id: String,
}

impl Rectangle {
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
        let w = relative.zoom(self.w);
        let h = relative.zoom(self.h);
        if w <= 1 && h <= 1 {
            let x = relative.x(self.x) as f64;
            let y = relative.y(self.y) as f64;
            context.begin_path();
            context.move_to(x, y);
            context.line_to(x, y);
            context.stroke();
        } else if w <= 2 && h <= 2 {
            context.stroke_rect(
                relative.x(self.x) as f64,
                relative.y(self.y) as f64,
                w as f64,
                h as f64,
            );
        } else {
            context.fill_rect(
                relative.x(self.x) as f64,
                relative.y(self.y) as f64,
                w as f64,
                h as f64,
            );
            context.stroke_rect(
                relative.x(self.x) as f64,
                relative.y(self.y) as f64,
                w as f64,
                h as f64,
            );
        }
    }
}
