use crate::{
    entity::{Ports, Signature},
    representation::{
        self,
        form::{self, rectangle::Rectangle, Form},
        style::{self, Style},
        Representation,
    },
};

use super::port::PortType;

const MIN_HEIGHT: i32 = 50;
const MIN_WIDTH: i32 = 50;

#[derive(Debug)]
pub struct Component {
    pub sig: Signature,
    pub ports: Ports,
    pub repr: Representation,
}

impl form::Default for Component {
    fn init() -> Form {
        Form::Rectangle(Rectangle {
            x: 0,
            y: 0,
            w: MIN_WIDTH,
            h: MIN_HEIGHT,
        })
    }
}

impl style::Default for Component {
    fn init() -> Style {
        Style {
            stroke_color: String::from("#000000"),
            fill_color: String::from("#DCDCDC"),
        }
    }
}

impl representation::Default for Component {
    fn init() -> Representation {
        Representation {
            form: <Component as form::Default>::init(),
            style: <Component as style::Default>::init(),
        }
    }
}

impl representation::Virtualization for Component {
    fn calc(&mut self) {
        // In/Out ports are located on a left/right sides. We are taking
        // a space, which is required by each type of ports and select
        // max of it. It will be minimal required height of box.
        self.repr.form.set_box_height(
            [
                MIN_HEIGHT,
                self.ports.required_height(PortType::In),
                self.ports.required_height(PortType::Out),
            ]
            .iter()
            .max()
            .copied()
            .unwrap_or(0),
        );
        // Set border for ports
        self.ports.border.set_height(self.repr.form.box_height());
        self.ports.border.set_width(self.repr.form.box_width());
        // Order ports
        self.ports.calc();
    }
}
