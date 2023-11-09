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
    // Returns box data: (x,y,w,h)
    pub fn box_data(forms: Vec<&Form>) -> Option<(i32, i32, i32, i32)> {
        // TODO: needs to be optimized
        let x0 = forms.iter().map(|f| f.get_coors().0).min();
        let y0 = forms.iter().map(|f| f.get_coors().1).min();
        let x1 = forms.iter().map(|f| f.box_width() + f.get_coors().0).max();
        let y1 = forms.iter().map(|f| f.box_height() + f.get_coors().1).max();
        if let (Some(x), Some(y), Some(x1), Some(y1)) = (x0, y0, x1, y1) {
            Some((x, y, x1 - x, y1 - y))
        } else {
            None
        }
    }
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

    pub fn get_coors(&self) -> (i32, i32) {
        match self {
            Self::Rectangle(figure) => figure.get_coors(),
            Self::Path(_) => {
                /* Ignore */
                (0, 0)
            }
        }
    }
    pub fn render(&self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        match self {
            Self::Rectangle(figure) => figure.render(context, relative),
            Self::Path(figure) => figure.render(context, relative),
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
