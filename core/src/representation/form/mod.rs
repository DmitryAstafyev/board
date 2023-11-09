pub mod path;
pub mod rectangle;

pub use path::Path;
pub use rectangle::Rectangle;

use crate::elements::relative::Relative;

pub trait Default {
    fn init() -> Form;
}

#[derive(Debug)]
pub enum Form {
    Rectangle(Rectangle),
    Path(Path),
}

impl Form {
    pub fn box_height(&self) -> i32 {
        match self {
            Self::Rectangle(figure) => figure.box_height(),
            Self::Path(figure) => figure.box_height(),
        }
    }
    pub fn box_width(&self) -> i32 {
        match self {
            Self::Rectangle(figure) => figure.box_width(),
            Self::Path(figure) => figure.box_width(),
        }
    }
    pub fn set_box_height(&mut self, h: i32) {
        match self {
            Self::Rectangle(figure) => figure.set_box_height(h),
            Self::Path(_) => { /* Ignore */ }
        }
    }
    pub fn set_coors(&mut self, x: Option<i32>, y: Option<i32>) {
        match self {
            Self::Rectangle(figure) => figure.set_coors(x, y),
            Self::Path(_) => { /* Ignore */ }
        }
    }
    pub fn render(&self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        match self {
            Self::Rectangle(figure) => figure.render(context, relative),
            Self::Path(_) => {
                todo!("Implement render for path")
            }
        }
    }

    pub fn relative(&self) -> Relative {
        match self {
            Self::Rectangle(figure) => figure.relative(),
            Self::Path(_) => {
                todo!("Implement render for path")
            }
        }
    }
}
