use crate::{entity::Signature, render::Representation};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PortType {
    In,
    Out,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Port {
    pub sig: Signature,
    pub port_type: PortType,
}

impl Port {
    pub fn set_type(&mut self, port_type: PortType) {
        self.port_type = port_type;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ports {
    pub ports: Vec<Representation<Port>>,
}

impl Ports {
    pub fn new() -> Self {
        Self { ports: vec![] }
    }

    pub fn len(&self) -> usize {
        self.ports.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ports.is_empty()
    }

    pub fn push(&mut self, port: Port) {
        self.ports.push(Representation::Origin(port));
    }

    pub fn get(&self, index: usize) -> &Port {
        // TODO: not safe!
        self.ports[index].origin()
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Port {
        // TODO: not safe!
        self.ports[index].origin_mut()
    }

    pub fn filter(&self, port_type: PortType) -> Vec<&Representation<Port>> {
        self.ports
            .iter()
            .filter(|r| r.origin().port_type == port_type)
            .collect::<Vec<&Representation<Port>>>()
    }

    pub fn filter_mut(&mut self, port_type: PortType) -> Vec<&mut Representation<Port>> {
        self.ports
            .iter_mut()
            .filter(|r| r.origin().port_type == port_type)
            .collect::<Vec<&mut Representation<Port>>>()
    }

    pub fn find(&self, port_id: usize) -> Option<&Representation<Port>> {
        self.ports.iter().find_map(|p| {
            if p.origin().sig.id == port_id {
                Some(p)
            } else {
                None
            }
        })
    }
}
