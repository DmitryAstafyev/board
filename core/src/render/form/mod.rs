pub mod button;
pub mod grid_rectangle;
pub mod label;
pub mod path;
pub mod rectangle;

use crate::{
    error::E,
    render::{grid::ElementType, Relative, Style},
};
pub use button::Button;
pub use grid_rectangle::GridRectangle;
pub use label::Label;
pub use path::{Path, Point};
pub use rectangle::Rectangle;

use super::options::Options;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct View {
    pub container: Container,
    pub elements: Vec<Container>,
}

impl View {
    pub fn render(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
        options: &Options,
    ) {
        self.container.render(context, relative, options);
        self.elements
            .iter_mut()
            .for_each(|container| container.render(context, relative, options));
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Container {
    pub form: Form,
    pub style: Style,
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
    pub fn render(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
        options: &Options,
    ) {
        self.style.apply(context);
        self.form.render(context, relative, options);
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Deserialize, Serialize)]
pub enum Form {
    GridRectangle(ElementType, GridRectangle),
    Rectangle(ElementType, Rectangle),
    Path(ElementType, Path),
    #[allow(dead_code)]
    Button(ElementType, Button),
    Label(ElementType, Label),
}

impl Form {
    pub fn get_el_ty(&self) -> &ElementType {
        match self {
            Self::Rectangle(ty, _) => ty,
            Self::GridRectangle(ty, _) => ty,
            Self::Path(ty, _) => ty,
            Self::Button(ty, _) => ty,
            Self::Label(ty, _) => ty,
        }
    }

    pub fn get_box_size(&self) -> (i32, i32) {
        match self {
            Self::Rectangle(_, figure) => figure.get_box_size(),
            Self::GridRectangle(_, figure) => figure.get_box_size(),
            Self::Path(_, figure) => figure.get_box_size(),
            Self::Button(_, figure) => figure.get_box_size(),
            Self::Label(_, figure) => figure.get_box_size(),
        }
    }
    pub fn set_box_size(&mut self, w: Option<i32>, h: Option<i32>) {
        match self {
            Self::Rectangle(_, figure) => figure.set_box_size(w, h),
            Self::GridRectangle(_, figure) => figure.set_box_size(w, h),
            Self::Path(_, _) => { /* Ignore */ }
            Self::Button(_, _) => { /* Ignore */ }
            Self::Label(_, _) => { /* Ignore */ }
        }
    }
    pub fn set_coors(&mut self, x: Option<i32>, y: Option<i32>) {
        match self {
            Self::Rectangle(_, figure) => figure.set_coors(x, y),
            Self::GridRectangle(_, figure) => figure.set_coors(x, y),
            Self::Path(_, _) => { /* Ignore */ }
            Self::Button(_, figure) => figure.set_coors(x, y),
            Self::Label(_, figure) => figure.set_coors(x, y),
        }
    }
    pub fn get_coors(&self) -> (i32, i32) {
        match self {
            Self::Rectangle(_, figure) => figure.get_coors(),
            Self::GridRectangle(_, figure) => figure.get_coors(),
            Self::Path(_, _) => {
                /* Ignore */
                (0, 0)
            }
            Self::Button(_, figure) => figure.get_coors(),
            Self::Label(_, figure) => figure.get_coors(),
        }
    }
    pub fn get_coors_with_zoom(&self, relative: &Relative) -> (i32, i32) {
        match self {
            Self::Rectangle(_, _) | Self::GridRectangle(_, _) | Self::Path(_, _) => {
                /* Ignore */
                (0, 0)
            }
            Self::Button(_, figure) => figure.get_coors_with_zoom(relative),
            Self::Label(_, figure) => figure.get_coors_with_zoom(relative),
        }
    }
    pub fn cells(&self) -> Result<(u32, u32), E> {
        match self {
            Self::Rectangle(_, _) => Err(E::NotGridForm),
            Self::GridRectangle(_, figure) => Ok(figure.cells),
            Self::Path(_, _) => Err(E::NotGridForm),
            Self::Button(_, _) => Err(E::NotGridForm),
            Self::Label(_, _) => Err(E::NotGridForm),
        }
    }
    pub fn id(&self) -> String {
        match self {
            Self::Rectangle(_, figure) => figure.id.clone(),
            Self::GridRectangle(_, figure) => figure.id.clone(),
            Self::Path(_, figure) => figure.id.clone(),
            Self::Button(_, figure) => figure.id.clone(),
            Self::Label(_, figure) => figure.id.clone(),
        }
    }
    pub fn render(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
        options: &Options,
    ) {
        match self {
            Self::Rectangle(_, figure) => figure.render(context, relative),
            Self::GridRectangle(_, figure) => figure.render(context, relative),
            Self::Path(_, figure) => figure.render(context, relative),
            Self::Button(_, figure) => figure.render(context, relative),
            Self::Label(_, figure) => figure.render(context, relative, options),
        }
    }

    pub fn calc(&mut self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        match self {
            Self::Rectangle(_, _) => {}
            Self::GridRectangle(_, _) => {}
            Self::Path(_, _) => {}
            Self::Button(_, figure) => figure.calc(context, relative),
            Self::Label(_, figure) => figure.calc(context, relative),
        }
    }
}
