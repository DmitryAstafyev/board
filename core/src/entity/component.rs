use std::ops::RangeInclusive;

use crate::{
    elements::relative::Relative,
    entity::{port::PortType, Ports, Signature, SignatureProducer},
    representation::{
        self,
        form::{self, rectangle::Rectangle, Form},
        style::{self, Style},
        Default, Representation,
    },
};

const MIN_HEIGHT: i32 = 64;
const MIN_WIDTH: i32 = 64;

#[derive(Debug)]
pub struct Component {
    pub sig: Signature,
    pub ports: Ports,
    pub repr: Representation,
}

impl Component {
    pub fn new(sig: Signature) -> Self {
        Self {
            sig,
            ports: Ports::new(),
            repr: Component::init(),
        }
    }

    pub fn dummy(producer: &mut SignatureProducer, ports: RangeInclusive<usize>) -> Self {
        Self {
            sig: producer.next(),
            ports: Ports::dummy(producer, ports),
            repr: Component::init(),
        }
    }

    pub fn relative(&self) -> Relative {
        self.repr.form.relative()
    }
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
            stroke_style: String::from("rgb(0,0,0)"),
            fill_style: String::from("rgb(200,200,200)"),
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

impl representation::Rendering for Component {
    fn render(
        &self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &crate::elements::relative::Relative,
    ) {
        let self_relative = self.relative();
        self.repr.style.apply(context);
        self.repr.form.render(context, relative);
        self.ports.render(context, &relative.merge(&self_relative));
    }
}
