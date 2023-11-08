use crate::{
    elements::border::Border,
    entity::Signature,
    representation::{
        self,
        form::{self, rectangle::Rectangle, Form},
        style::{self, Style},
        Representation,
    },
};

const PORTS_VERTICAL_OFFSET: i32 = 10;

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

#[derive(Debug)]
pub struct Ports {
    pub ports: Vec<Port>,
    pub border: Border,
}

impl Ports {
    pub fn new() -> Self {
        Self {
            ports: vec![],
            border: Default::default(),
        }
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
                    .set_coors(Some(self.border.width + (w / 2)), Some(cursor));
                cursor += h + PORTS_VERTICAL_OFFSET;
            });
    }
}
