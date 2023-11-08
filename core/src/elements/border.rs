#[derive(Debug, Default)]
pub struct Border {
    pub height: i32,
    pub width: i32,
}

impl Border {
    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }
    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }
}
