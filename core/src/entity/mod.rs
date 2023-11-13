mod comosition;
mod component;
mod connection;
pub mod dummy;
mod port;

pub use comosition::Composition;
pub use component::Component;
pub use connection::{Connection, Joint};
pub use port::{Port, PortType, Ports};

#[derive(Debug, Clone)]
pub struct Signature {
    pub id: usize,
    pub class_name: String,
}
