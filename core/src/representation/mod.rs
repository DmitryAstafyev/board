pub mod form;
pub mod style;

pub use form::Form;
pub use style::Style;

use crate::elements::relative::Relative;

#[derive(Debug)]
pub struct Representation {
    pub form: Form,
    pub style: Style,
}

pub trait Default {
    fn init() -> Representation;
}

pub trait Virtualization {
    fn calc(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &crate::elements::relative::Relative,
    );
}

pub trait Rendering {
    fn render(&self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative);
}
