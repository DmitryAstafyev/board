use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum PortsRepresentation {
    Blocks,
    Labels,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ports {
    representation: PortsRepresentation,
    grouping: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ConnectionsAlign {
    Streamlined,
    Straight,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Connections {
    align: ConnectionsAlign,
    hide: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    ports: Ports,
    connections: Connections,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            ports: Ports {
                representation: PortsRepresentation::Blocks,
                grouping: true,
            },
            connections: Connections {
                align: ConnectionsAlign::Streamlined,
                hide: false,
            },
        }
    }
}
