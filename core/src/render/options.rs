use crate::render::Ratio;
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
    pub group_unbound: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Connections {
    pub hide: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GridOptions {
    pub cell_size_px: u32,
    pub cells_space_vertical: u32,
    pub cells_space_horizontal: u32,
    pub visible: bool,
    pub vpadding: u32,
    pub hpadding: u32,
    pub vmargin: u32,
    pub hmargin: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Labels {
    pub ports_short_name: bool,
    pub components_short_name: bool,
    pub composition_short_name: bool,
    pub port_label_max_len: usize,
    pub comp_label_max_len: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    pub ports: Ports,
    pub connections: Connections,
    pub grid: GridOptions,
    pub labels: Labels,
    pub ratio: u8,
    pub font: String,
}

impl Options {
    pub fn ratio(&self) -> Ratio {
        Ratio { ratio: self.ratio }
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            ports: Ports {
                representation: PortsRepresentation::Blocks,
                grouping: true,
                group_unbound: true,
            },
            connections: Connections { hide: false },
            grid: GridOptions {
                vpadding: 3,
                hpadding: 5,
                vmargin: 0,
                hmargin: 5,
                cell_size_px: 25,
                cells_space_vertical: 3,
                cells_space_horizontal: 3,
                visible: true,
            },
            labels: Labels {
                ports_short_name: true,
                components_short_name: true,
                composition_short_name: true,
                port_label_max_len: 16,
                comp_label_max_len: 12,
            },
            ratio: 1,
            font: String::from("Roboto, sans-serif"),
        }
    }
}
