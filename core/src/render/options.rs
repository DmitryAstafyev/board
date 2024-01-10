use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum PortsRepresentation {
    Blocks,
    Labels,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ports {
    pub representation: PortsRepresentation,
    pub grouping: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ConnectionsAlign {
    Streamlined,
    Straight,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Connections {
    pub align: ConnectionsAlign,
    pub hide: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GridOptions {
    pub cell_size_px: u32,
    pub cells_space_vertical: u32,
    pub cells_space_horizontal: u32,
    pub visible: bool,
    pub padding: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    pub ports: Ports,
    pub connections: Connections,
    pub grid: GridOptions,
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
            grid: GridOptions {
                padding: 3,
                cell_size_px: 25,
                cells_space_vertical: 3,
                cells_space_horizontal: 3,
                visible: true,
            },
        }
    }
}
