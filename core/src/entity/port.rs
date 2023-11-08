use crate::{
    entity::Signature,
    representation::{
        self,
        form::{self, rectangle::Rectangle, Form},
        style::{self, Style},
        Representation,
    },
};

#[derive(Debug)]
pub enum PortType {
    In,
    Out,
}

#[derive(Debug)]
pub struct Port {
    pub sig: Signature,
    pub port_type: PortType,
    pub repr: Representation,
}

impl form::Default for Port {
    fn init() -> Form {
        Form::Rectangle(Rectangle {
            x: 0,
            y: 0,
            w: 25,
            h: 25,
        })
    }
}

impl style::Default for Port {
    fn init() -> Style {
        Style {
            stroke_color: String::from("#000000"),
            fill_color: String::from("#66CCFF"),
        }
    }
}

impl representation::Default for Port {
    fn init() -> Representation {
        Representation {
            form: <Port as form::Default>::init(),
            style: <Port as style::Default>::init(),
        }
    }
}
