pub mod path;
pub mod rectangle;

pub use path::Path;
pub use rectangle::Rectangle;

pub trait Default {
    fn init() -> Form;
}

#[derive(Debug)]
pub enum Form {
    Rectangle(Rectangle),
    Path(Path),
}
