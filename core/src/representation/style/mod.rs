#[derive(Debug)]
pub struct Style {
    pub stroke_color: String,
    pub fill_color: String,
}

pub trait Default {
    fn init() -> Style;
}
