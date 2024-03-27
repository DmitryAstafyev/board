use std::usize;

use crate::{
    entity::{Signature, SignatureGetter},
    render::{options::Options, Representation},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PortType {
    In,
    Out,
    Unbound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Port {
    pub sig: Signature,
    pub port_type: PortType,
    pub contains: Vec<usize>,
    pub visibility: bool,
}

impl<'a, 'b: 'a> SignatureGetter<'a, 'b> for Port {
    fn sig(&'b self) -> &'a Signature {
        &self.sig
    }
}

impl Port {
    pub fn set_type(&mut self, port_type: PortType) {
        self.port_type = port_type;
    }
    pub fn get_label(&self, options: &Options, len: usize) -> String {
        self.sig.as_label(options.labels.ports_short_name, len)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ports {
    pub ports: Vec<Representation<Port>>,
    // #[serde(skip_serializing, skip_deserializing)]
    pub hide_invisible: bool,
    pub sig: Signature,
}

impl Default for Ports {
    fn default() -> Self {
        Self {
            ports: Vec::new(),
            hide_invisible: true,
            sig: Signature::default(),
        }
    }
}

impl<'a, 'b: 'a> SignatureGetter<'a, 'b> for Ports {
    fn sig(&'b self) -> &'a Signature {
        &self.sig
    }
}

impl Ports {
    pub fn new() -> Self {
        Self {
            ports: Vec::new(),
            hide_invisible: true,
            sig: Signature::default(),
        }
    }
    pub fn iter(&self) -> core::slice::Iter<'_, Representation<Port>> {
        self.ports.iter()
    }
    pub fn len(&self) -> usize {
        self.ports.len()
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

    pub fn filter(&self, targets: &[PortType]) -> Vec<&Representation<Port>> {
        self.ports
            .iter()
            .filter(|r| targets.contains(&r.origin().port_type))
            .collect::<Vec<&Representation<Port>>>()
    }

    pub fn filter_mut(&mut self, targets: &[PortType]) -> Vec<&mut Representation<Port>> {
        self.ports
            .iter_mut()
            .filter(|r| targets.contains(&r.origin().port_type))
            .collect::<Vec<&mut Representation<Port>>>()
    }

    pub fn find(&self, port_id: &usize) -> Option<&Representation<Port>> {
        self.ports.iter().find(|p| &p.sig().id == port_id)
    }

    pub fn cloned_ports(&self) -> Vec<Representation<Port>> {
        self.ports
            .iter()
            .map(|r| Representation::Origin(r.origin().clone()))
            .collect::<Vec<Representation<Port>>>()
    }

    pub fn clone(&self) -> Ports {
        Ports {
            ports: self.cloned_ports(),
            hide_invisible: self.hide_invisible,
            sig: Signature::default(),
        }
    }

    pub fn hide(&mut self, ids: &[usize]) {
        self.ports.iter_mut().for_each(|port| {
            if ids.contains(&port.sig().id) {
                port.origin_mut().visibility = false;
            }
        });
    }

    pub fn add(&mut self, port: Representation<Port>, pos: Option<usize>) {
        if let Some(pos) = pos {
            self.ports.insert(pos, port);
        } else {
            self.ports.push(port);
        }
    }

    pub fn get_groupped(&self) -> Vec<(usize, Vec<usize>)> {
        let mut ports: Vec<(usize, Vec<usize>)> = Vec::new();
        self.ports.iter().for_each(|p| {
            if p.origin().contains.is_empty() {
                return;
            }
            ports.push((p.sig().id, p.origin().contains.clone()));
        });
        ports
    }

    pub fn get_filtered_ports(&self, filter: &str) -> Vec<usize> {
        fn is_filtered(filter: &str, origin: &Port) -> bool {
            origin
                .sig
                .short_name
                .to_lowercase()
                .contains(&filter.to_lowercase())
        }
        let ports = &self.ports;
        ports
            .iter()
            .filter(|port| {
                let origin = port.origin();
                if origin.contains.is_empty() {
                    is_filtered(filter, origin)
                } else {
                    let mut found = false;
                    for port_id in origin.contains.iter() {
                        if let Some(port) = ports.iter().find(|p| &p.sig().id == port_id) {
                            if is_filtered(filter, port.origin()) {
                                found = true;
                                break;
                            }
                        }
                    }
                    found
                }
            })
            .map(|p| p.sig().id)
            .collect()
    }
}
