use crate::{
    entity::{Component, Connection, Signature},
    representation::{
        self,
        form::{self, rectangle::Rectangle, Form},
        style::{self, Style},
        Representation,
    },
};

#[derive(Debug)]
pub struct Composition<'a> {
    pub sig: Signature,
    pub components: &'a [Component],
    pub connections: &'a [Connection<'a>],
    pub repr: Representation,
}

impl form::Default for Composition<'_> {
    fn init() -> Form {
        Form::Rectangle(Rectangle {
            x: 0,
            y: 0,
            w: 200,
            h: 200,
        })
    }
}

impl style::Default for Composition<'_> {
    fn init() -> Style {
        Style {
            stroke_color: String::from("#000000"),
            fill_color: String::from("#FAFAFA"),
        }
    }
}

impl representation::Default for Composition<'_> {
    fn init() -> Representation {
        Representation {
            form: <Composition<'_> as form::Default>::init(),
            style: <Composition<'_> as style::Default>::init(),
        }
    }
}
