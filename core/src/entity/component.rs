use crate::{
    entity::{Port, Ports, Signature, SignatureGetter},
    render::{options::Options, Representation},
};
use serde::{Deserialize, Serialize};

use super::EntityProps;

#[derive(Debug, Deserialize, Serialize)]
pub struct Component {
    pub sig: Signature,
    pub ports: Representation<Ports>,
    pub composition: bool,
}

impl<'a, 'b: 'a> SignatureGetter<'a, 'b> for Component {
    fn sig(&'b self) -> &'a Signature {
        &self.sig
    }
}

impl Component {
    pub fn get_label(&self, options: &Options) -> String {
        self.sig.as_label(
            options.labels.components_short_name,
            options.labels.comp_label_max_len,
        )
    }
    pub fn get_port(&self, port_id: &usize) -> Option<&Port> {
        self.ports.origin().find(port_id).map(|r| r.origin())
    }
    pub fn get_ports_props(&self) -> EntityProps {
        self.ports.origin().get_props()
    }
}
