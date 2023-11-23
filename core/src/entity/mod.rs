mod comosition;
mod component;
mod connection;
pub mod dummy;
mod port;

pub use comosition::Composition;
pub use component::Component;
pub use connection::{Connection, Joint};
pub use port::{Port, PortType, Ports};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signature {
    pub id: usize,
    pub class_name: String,
}

impl Signature {
    pub fn fake() -> Self {
        Self {
            id: 0,
            class_name: "fake".to_string(),
        }
    }
}
