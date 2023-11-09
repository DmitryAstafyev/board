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
    pub fn merge(&self, relative: &Relative) -> Relative {
        Relative::new(relative.x(self.x_), relative.y(self.y_))
    }
}
