mod comosition;
mod component;
mod connection;
mod port;

pub use comosition::Composition;
pub use component::Component;
pub use connection::Connection;
pub use port::{Port, Ports};

#[derive(Debug)]
pub struct Signature {
    pub id: usize,
    pub class_name: String,
}
