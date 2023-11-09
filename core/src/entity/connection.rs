use crate::{
    entity::{port::PortType, Component, Port, Signature},
    representation::{
        self,
        form::{self, path::Path, Form},
        style::{self, Style},
        Default, Representation,
    },
};

#[derive(Debug)]
pub struct Joint {
    pub port: usize,
    pub component: usize,
}

impl Joint {
    pub fn new(port_id: usize, component_id: usize) -> Self {
        Self {
            port: port_id,
            component: component_id,
        }
    }
}

#[derive(Debug)]
pub struct Connection {
    pub sig: Signature,
    pub joint_in: Joint,
    pub joint_out: Joint,
    pub repr: Representation,
}

impl Connection {
    pub fn new(sig: Signature, joint_in: Joint, joint_out: Joint) -> Self {
        Self {
            sig,
            repr: Connection::init(),
            joint_in,
            joint_out,
        }
    }

    pub fn get_linked_to(&self, sig: &Signature) -> Option<(PortType, usize)> {
        if self.joint_in.component == sig.id {
            Some((PortType::Out, self.joint_out.component))
        } else if self.joint_out.component == sig.id {
            Some((PortType::In, self.joint_in.component))
        } else {
            None
        }
    }
}

impl form::Default for Connection {
    fn init() -> Form {
        Form::Path(Path { points: vec![] })
    }
}

impl style::Default for Connection {
    fn init() -> Style {
        Style {
            stroke_style: String::from("#222222"),
            fill_style: String::from("#FFFFFF"),
        }
    }
}

impl representation::Default for Connection {
    fn init() -> Representation {
        Representation {
            form: <Connection as form::Default>::init(),
            style: <Connection as style::Default>::init(),
        }
    }
}
