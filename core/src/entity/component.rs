use crate::{
    entity::{Port, Signature},
    representation::{
        self,
        form::{self, rectangle::Rectangle, Form},
        style::{self, Style},
        Representation,
    },
};

#[derive(Debug)]
pub struct Component<'a> {
    pub sig: Signature,
    pub ports: &'a [Port],
    pub repr: Representation,
}

impl form::Default for Component<'_> {
    fn init() -> Form {
        Form::Rectangle(Rectangle {
            x: 0,
            y: 0,
            w: 50,
            h: 100,
        })
    }
}

impl style::Default for Component<'_> {
    fn init() -> Style {
        Style {
            stroke_color: String::from("#000000"),
            fill_color: String::from("#DCDCDC"),
        }
    }
}

impl representation::Default for Component<'_> {
    fn init() -> Representation {
        Representation {
            form: <Component<'_> as form::Default>::init(),
            style: <Component<'_> as style::Default>::init(),
        }
    }
}
