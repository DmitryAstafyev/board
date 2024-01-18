pub mod button;
pub mod grid_rectangle;
pub mod label;
pub mod path;
pub mod rectangle;

use crate::{error::E, render::Relative, render::Style};
pub use button::Button;
pub use grid_rectangle::GridRectangle;
pub use label::Label;
pub use path::{Path, Point};
pub use rectangle::Rectangle;

#[derive(Debug)]
pub struct View {
    pub container: Container,
    pub elements: Vec<Container>,
}

impl View {
    pub fn render(&mut self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        self.container.render(context, relative);
        self.elements
            .iter_mut()
            .for_each(|container| container.render(context, relative));
    }
}

#[derive(Debug)]
pub struct Container {
    pub form: Form,
    pub style: Style,
    pub hover: Option<Style>,
}

impl Container {
    pub fn set_form(&mut self, form: Form) {
        self.form = form;
    }
    pub fn get_box_size(&self) -> (i32, i32) {
        self.form.get_box_size()
    }
    pub fn set_box_size(&mut self, w: Option<i32>, h: Option<i32>) {
        self.form.set_box_size(w, h)
    }
    pub fn set_coors(&mut self, x: Option<i32>, y: Option<i32>) {
        self.form.set_coors(x, y)
    }
    pub fn get_coors(&self) -> (i32, i32) {
        self.form.get_coors()
    }
    pub fn get_coors_with_zoom(&self, relative: &Relative) -> (i32, i32) {
        self.form.get_coors_with_zoom(relative)
    }
    pub fn id(&self) -> String {
        self.form.id()
    }
    pub fn render(&mut self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        self.style.apply(context);
        self.form.render(context, relative);
    }
}

#[derive(Debug)]
pub enum Form {
    GridRectangle(GridRectangle),
    Rectangle(Rectangle),
    Path(Path),
    Button(Button),
    Label(Label),
}

impl Form {
    pub fn get_box_size(&self) -> (i32, i32) {
        match self {
            Self::Rectangle(figure) => figure.get_box_size(),
            Self::GridRectangle(figure) => figure.get_box_size(),
            Self::Path(figure) => figure.get_box_size(),
            Self::Button(figure) => figure.get_box_size(),
            Self::Label(figure) => figure.get_box_size(),
        }
    }
    pub fn set_box_size(&mut self, w: Option<i32>, h: Option<i32>) {
        match self {
            Self::Rectangle(figure) => figure.set_box_size(w, h),
            Self::GridRectangle(figure) => figure.set_box_size(w, h),
            Self::Path(_) => { /* Ignore */ }
            Self::Button(_) => { /* Ignore */ }
            Self::Label(_) => { /* Ignore */ }
        }
    }
    pub fn set_coors(&mut self, x: Option<i32>, y: Option<i32>) {
        match self {
            Self::Rectangle(figure) => figure.set_coors(x, y),
            Self::GridRectangle(figure) => figure.set_coors(x, y),
            Self::Path(_) => { /* Ignore */ }
            Self::Button(figure) => figure.set_coors(x, y),
            Self::Label(figure) => figure.set_coors(x, y),
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
            Self::Button(figure) => figure.get_coors(),
            Self::Label(figure) => figure.get_coors(),
        }
    }
    pub fn get_coors_with_zoom(&self, relative: &Relative) -> (i32, i32) {
        match self {
            Self::Rectangle(_) | Self::GridRectangle(_) | Self::Path(_) => {
                /* Ignore */
                (0, 0)
            }
            Self::Button(figure) => figure.get_coors_with_zoom(relative),
            Self::Label(figure) => figure.get_coors_with_zoom(relative),
        }
    }
    pub fn cells(&self) -> Result<(u32, u32), E> {
        match self {
            Self::Rectangle(_) => Err(E::NotGridForm),
            Self::GridRectangle(figure) => Ok(figure.cells),
            Self::Path(_) => Err(E::NotGridForm),
            Self::Button(_) => Err(E::NotGridForm),
            Self::Label(_) => Err(E::NotGridForm),
        }
    }
    pub fn id(&self) -> String {
        match self {
            Self::Rectangle(figure) => figure.id.clone(),
            Self::GridRectangle(figure) => figure.id.clone(),
            Self::Path(figure) => figure.id.clone(),
            Self::Button(figure) => figure.id.clone(),
            Self::Label(figure) => figure.id.clone(),
        }
    }
    pub fn render(&mut self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        match self {
            Self::Rectangle(figure) => figure.render(context, relative),
            Self::GridRectangle(figure) => figure.render(context, relative),
            Self::Path(figure) => figure.render(context, relative),
            Self::Button(figure) => figure.render(context, relative),
            Self::Label(figure) => figure.render(context, relative),
        }
    }

    pub fn calc(&mut self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        match self {
            Self::Rectangle(_) => {}
            Self::GridRectangle(_) => {}
            Self::Path(_) => {}
            Self::Button(figure) => figure.calc(context, relative),
            Self::Label(figure) => figure.calc(context, relative),
        }
    }
}
