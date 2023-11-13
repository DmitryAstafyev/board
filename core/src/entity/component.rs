use crate::entity::{Ports, Signature};

#[derive(Debug)]
pub struct Component {
    pub sig: Signature,
    pub ports: Ports,
}

impl Component {
    pub fn new(sig: Signature) -> Self {
        Self {
            sig,
            ports: Ports::new(),
        }
    }
}
