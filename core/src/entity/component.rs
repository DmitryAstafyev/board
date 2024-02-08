use crate::{
    entity::{Ports, Signature},
    render::{options::Options, Representation},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Component {
    pub sig: Signature,
    pub ports: Representation<Ports>,
    pub composition: bool,
}

impl Component {
    pub fn get_label(&self, options: &Options, len: usize) -> String {
        if options.labels.components_short_name {
            if self.sig.class_name == "unknown" && self.sig.short_name == "unknown" {
                self.sig.id.to_string()
            } else if self.sig.short_name != "unknown" {
                format!(
                    "{:.len$}{}",
                    self.sig.short_name,
                    if self.sig.short_name.len() > len {
                        "..."
                    } else {
                        ""
                    }
                )
            } else {
                format!(
                    "{:.len$}{}",
                    self.sig.class_name,
                    if self.sig.class_name.len() > len {
                        "..."
                    } else {
                        ""
                    }
                )
            }
        } else {
            self.sig.id.to_string()
        }
    }
}
