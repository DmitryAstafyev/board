pub mod elements;
pub mod entity;
pub mod form;
pub mod grid;
pub mod representation;
pub mod style;

pub use elements::{border::Border, relative::Relative};
pub use form::Form;
pub use grid::Grid;
pub use representation::Representation;
pub use style::Style;

#[derive(Debug)]
pub struct Render<T> {
    entity: T,
    form: Form,
    style: Style,
    grid: Option<Grid>,
}

impl<T> Render<T> {
    pub fn origin(&self) -> &T {
        &self.entity
    }

    pub fn origin_mut(&mut self) -> &mut T {
        &mut self.entity
    }

    pub fn relative(&self, base: &Relative) -> Relative {
        let (x, y) = self.form.get_coors();
        base.from_origin_coors(x, y)
    }

    pub fn own_relative(&self) -> Relative {
        let (x, y) = self.form.get_coors();
        Relative::new(x, y, None)
    }
}
