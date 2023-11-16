#[derive(Debug, Default)]
pub struct Relative {
    x_: i32,
    y_: i32,
}

impl Relative {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x_: x, y_: y }
    }
    pub fn x(&self, x: i32) -> i32 {
        self.x_ + x
    }
    pub fn y(&self, y: i32) -> i32 {
        self.y_ + y
    }
    pub fn set_x(&mut self, x: i32) {
        self.x_ = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.y_ = y;
    }
    pub fn from_coors(&self, x: i32, y: i32) -> Relative {
        Relative::new(self.x(x), self.y(y))
    }
    pub fn from_base(&self, base: &Relative) -> Relative {
        Relative::new(base.x(self.x_), base.y(self.y_))
    }
}
