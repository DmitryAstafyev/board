pub mod grid_rectangle;
pub mod path;
pub mod rectangle;

use crate::{error::E, render::Relative};
pub use grid_rectangle::GridRectangle;
pub use path::{Path, Point};
pub use rectangle::Rectangle;

#[derive(Debug)]
pub enum Form {
    GridRectangle(GridRectangle),
    Rectangle(Rectangle),
    Path(Path),
}

impl Form {
    pub fn get_box_size(&self) -> (i32, i32) {
        match self {
            Self::Rectangle(figure) => figure.get_box_size(),
            Self::GridRectangle(figure) => figure.get_box_size(),
            Self::Path(figure) => figure.get_box_size(),
        }
    }
    pub fn set_box_size(&mut self, w: Option<i32>, h: Option<i32>) {
        match self {
            Self::Rectangle(figure) => figure.set_box_size(w, h),
            Self::GridRectangle(figure) => figure.set_box_size(w, h),
            Self::Path(_) => { /* Ignore */ }
        }
    }
    pub fn set_coors(&mut self, x: Option<i32>, y: Option<i32>) {
        match self {
            Self::Rectangle(figure) => figure.set_coors(x, y),
            Self::GridRectangle(figure) => figure.set_coors(x, y),
            Self::Path(_) => { /* Ignore */ }
        }
    }
    pub fn get_coors(&self) -> (i32, i32) {
        match self {
            Self::Rectangle(figure) => figure.get_coors(),
            Self::GridRectangle(figure) => figure.get_coors(),
            Self::Path(_) => {
                /* Ignore */
                (0, 0)
            }
        }
    }
    pub fn cells(&self) -> Result<(u32, u32), E> {
        match self {
            Self::Rectangle(_) => Err(E::NotGridForm),
            Self::GridRectangle(figure) => Ok(figure.cells),
            Self::Path(_) => Err(E::NotGridForm),
        }
    }
    pub fn id(&self) -> usize {
        match self {
            Self::Rectangle(figure) => figure.id,
            Self::GridRectangle(figure) => figure.id,
            Self::Path(figure) => figure.id,
        }
    }
    pub fn render(&self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        match self {
            Self::Rectangle(figure) => figure.render(context, relative),
            Self::GridRectangle(figure) => figure.render(context, relative),
            Self::Path(figure) => figure.render(context, relative),
        }
    }
}
