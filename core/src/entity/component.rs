use crate::{
    entity::{Ports, Signature, SignatureGetter},
    render::{options::Options, Representation},
};
use serde::{Deserialize, Serialize};

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
    pub fn get_label(&self, options: &Options, len: usize) -> String {
        self.sig.as_label(options.labels.components_short_name, len)
    }
}
