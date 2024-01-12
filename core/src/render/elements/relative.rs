#[derive(Debug, Default)]
pub struct Relative {
    pub x_: i32,
    pub y_: i32,
    pub zoom_: f64,
}

impl Relative {
    pub fn new(x: i32, y: i32, zoom: Option<f64>) -> Self {
        Self {
            x_: x,
            y_: y,
            zoom_: zoom.unwrap_or(1.0),
        }
    }
    pub fn x(&self, x: i32) -> i32 {
        self.zoom(self.x_ + x)
    }
    pub fn y(&self, y: i32) -> i32 {
        self.zoom(self.y_ + y)
    }
    pub fn zoom(&self, v: i32) -> i32 {
        (v as f64 * self.zoom_).ceil() as i32
    }
    pub fn get_zoom(&self) -> f64 {
        self.zoom_
    }
    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom_ = zoom;
    }
    pub fn clone_from_origin_coors(&self, x: i32, y: i32) -> Relative {
        Relative::new(self.x_ + x, self.y_ + y, Some(self.zoom_))
    }
}
