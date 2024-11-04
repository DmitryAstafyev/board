pub mod elements;
pub mod entity;
pub mod form;
pub mod grid;
pub mod options;
pub mod ratio;
pub mod representation;
pub mod style;

pub use elements::relative::Relative;
pub use form::{Container, Form, View};
pub use grid::Grid;
pub use ratio::*;
pub use representation::Representation;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use style::Style;

#[derive(Debug, Deserialize, Serialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub struct Render<T>
where
    T: Serialize + DeserializeOwned,
{
    entity: T,
    view: View,
    hidden: bool,
}

impl<T> Render<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn origin(&self) -> &T {
        &self.entity
    }

    pub fn origin_mut(&mut self) -> &mut T {
        &mut self.entity
    }

    pub fn relative(&self, base: &Relative) -> Relative {
        let (x, y) = self.view.container.get_coors();
        base.clone_from_origin_coors(x, y)
    }

    pub fn own_relative(&self) -> Relative {
        let (x, y) = self.view.container.get_coors();
        Relative::new(x, y, None)
    }

    pub fn set_over_style(&mut self, _style: Option<Style>) {
        // self.over_style = style;
    }

    pub fn hide(&mut self) {
        self.hidden = true;
    }
    pub fn show(&mut self) {
        self.hidden = false;
    }
}
