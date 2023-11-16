#[derive(Debug, Default)]
pub struct Relative {
    x_: i32,
    y_: i32,
    zoom_: f64,
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
        (v as f64 * self.zoom_) as i32
    }
    pub fn get_zoom(&self) -> f64 {
        self.zoom_
    }
    pub fn set_x(&mut self, x: i32) {
        self.x_ = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.y_ = y;
    }
    pub fn from_coors(&self, x: i32, y: i32) -> Relative {
        Relative::new(self.x(x), self.y(y), Some(self.zoom_))
    }
    pub fn from_origin_coors(&self, x: i32, y: i32) -> Relative {
        Relative::new(self.x_ + x, self.y_ + y, Some(self.zoom_))
    }
    pub fn from_base(&self, base: &Relative) -> Relative {
        Relative::new(base.x(self.x_), base.y(self.y_), Some(self.zoom_))
    }
}
