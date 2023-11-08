pub mod form;
pub mod style;

pub use form::Form;
pub use style::Style;

#[derive(Debug)]
pub struct Representation {
    pub form: Form,
    pub style: Style,
}

pub trait Default {
    fn init() -> Representation;
}
