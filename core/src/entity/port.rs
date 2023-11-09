use std::ops::RangeInclusive;

use crate::{
    elements::border::Border,
    entity::{Signature, SignatureProducer},
    representation::{
        self,
        form::{self, rectangle::Rectangle, Form},
        style::{self, Style},
        Default, Representation,
    },
};
use rand::Rng;

const PORTS_VERTICAL_OFFSET: i32 = 8;

#[derive(Debug, PartialEq)]
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

impl Port {
    fn dummy(producer: &mut SignatureProducer) -> Port {
        Port {
            sig: producer.next(),
            port_type: if rand::random() {
                PortType::In
            } else {
                PortType::Out
            },
            repr: Port::init(),
        }
    }
}

impl form::Default for Port {
    fn init() -> Form {
        Form::Rectangle(Rectangle {
            x: 0,
            y: 0,
            w: 8,
            h: 8,
        })
    }
}

impl style::Default for Port {
    fn init() -> Style {
        Style {
            stroke_style: String::from("rgb(0,0,0)"),
            fill_style: String::from("rgb(50,50,50)"),
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

#[derive(Debug)]
pub struct Ports {
    pub ports: Vec<Port>,
    pub border: Border,
}

impl Ports {
    pub fn new() -> Self {
        Self {
            ports: vec![],
            border: std::default::Default::default(),
        }
    }

    pub fn dummy(producer: &mut SignatureProducer, ports: RangeInclusive<usize>) -> Self {
        let count = rand::thread_rng().gen_range(ports);
        let mut instance = Self::new();
        for _ in 0..count {
            instance.ports.push(Port::dummy(producer));
        }
        instance
    }

    pub fn link(&mut self, ports: Vec<Port>) {
        self.ports = ports;
    }

    pub fn required_height(&self, port_type: PortType) -> i32 {
        if self.ports.is_empty() {
            return 0;
        }
        let mut required: i32 = PORTS_VERTICAL_OFFSET;
        self.ports
            .iter()
            .filter(|p| p.port_type == port_type)
            .for_each(|p| required += p.repr.form.box_height() + PORTS_VERTICAL_OFFSET);
        required
    }

    pub fn len(&self) -> usize {
        self.ports.len()
    }

    pub fn get(&self, index: usize) -> &Port {
        &self.ports[index]
    }
}

impl representation::Virtualization for Ports {
    fn calc(&mut self) {
        // Order ports on a left side
        let mut cursor: i32 = PORTS_VERTICAL_OFFSET;
        self.ports
            .iter_mut()
            .filter(|p| p.port_type == PortType::In)
            .for_each(|p| {
                let h = p.repr.form.box_height();
                let w = p.repr.form.box_width();
                p.repr.form.set_coors(Some(-(w / 2)), Some(cursor));
                cursor += h + PORTS_VERTICAL_OFFSET;
            });
        // Order ports on a right side
        let mut cursor: i32 = PORTS_VERTICAL_OFFSET;
        self.ports
            .iter_mut()
            .filter(|p| p.port_type == PortType::Out)
            .for_each(|p| {
                let h = p.repr.form.box_height();
                let w = p.repr.form.box_width();
                p.repr
                    .form
                    .set_coors(Some(self.border.width - (w / 2)), Some(cursor));
                cursor += h + PORTS_VERTICAL_OFFSET;
            });
    }
}

impl representation::Rendering for Port {
    fn render(
        &self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &crate::elements::relative::Relative,
    ) {
        self.repr.style.apply(context);
        self.repr.form.render(context, relative);
    }
}

impl representation::Rendering for Ports {
    fn render(
        &self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &crate::elements::relative::Relative,
    ) {
        self.ports.iter().for_each(|p| p.render(context, relative));
    }
}
