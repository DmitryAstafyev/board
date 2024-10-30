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
pub struct RectColor {
    pub stroke: String,
    pub fill: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorScheme {
    pub composition_rect: RectColor,
    pub composition_label: RectColor,
    pub composition_as_component_rect: RectColor,
    pub component_rect: RectColor,
    pub selected_rect: RectColor,
    pub highlighted_rect: RectColor,
    pub matched_rect: RectColor,
    pub hovered_rect: RectColor,
    pub connection_line: RectColor,
    pub port_highlighted_rect: RectColor,
    pub port_rect: RectColor,
    pub port_unlinked_rect: RectColor,
    pub port_linked_rect: RectColor,
    pub port_grouped_rect: RectColor,
    pub port_pri_bagde: RectColor,
    pub port_pi_bagde: RectColor,
    pub port_ri_bagde: RectColor,
    pub port_index_label: RectColor,
    pub port_subbagde: RectColor,
    pub label_subtitle: RectColor,
    pub label: RectColor,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            composition_rect: RectColor {
                stroke: String::from("rgb(0,0,0)"),
                fill: String::from("rgb(200,200,230)"),
            },
            composition_label: RectColor {
                stroke: String::from("rgb(30,30,30)"),
                fill: String::from("rgb(0,0,0)"),
            },
            composition_as_component_rect: RectColor {
                stroke: String::from("rgb(0,0,0)"),
                fill: String::from("rgb(250,200,200)"),
            },
            component_rect: RectColor {
                stroke: String::from("rgb(0,0,0)"),
                fill: String::from("rgb(240,240,240)"),
            },
            selected_rect: RectColor {
                stroke: String::from("rgb(0,0,0)"),
                fill: String::from("rgb(100,150,100)"),
            },
            highlighted_rect: RectColor {
                stroke: String::from("rgb(50,50,50)"),
                fill: String::from("rgb(185,230,255)"),
            },
            port_highlighted_rect: RectColor {
                stroke: String::from("rgb(0,0,0)"),
                fill: String::from("rgb(220,250,220)"),
            },
            matched_rect: RectColor {
                stroke: String::from("rgb(50,50,50)"),
                fill: String::from("rgb(195,190,190)"),
            },
            hovered_rect: RectColor {
                stroke: String::from("rgb(0,0,0)"),
                fill: String::from("rgb(200,200,200)"),
            },
            connection_line: RectColor {
                stroke: String::from("rgb(30,30,30)"),
                fill: String::from("rgb(30,30,30)"),
            },
            port_rect: RectColor {
                stroke: String::from("rgb(50,50,50)"),
                fill: String::from("rgb(240,240,240)"),
            },
            port_unlinked_rect: RectColor {
                stroke: String::from("rgb(50,50,50)"),
                fill: String::from("rgb(200,200,240)"),
            },
            port_linked_rect: RectColor {
                stroke: String::from("rgb(150,150,150)"),
                fill: String::from("rgb(250,250,250)"),
            },
            port_grouped_rect: RectColor {
                stroke: String::from("rgb(50,50,50)"),
                fill: String::from("rgb(255,255,200)"),
            },
            port_pri_bagde: RectColor {
                stroke: String::from("rgb(40,140,40)"),
                fill: String::from("rgb(255,255,255)"),
            },
            port_pi_bagde: RectColor {
                stroke: String::from("rgb(200,200,200)"),
                fill: String::from("rgb(0,0,0)"),
            },
            port_ri_bagde: RectColor {
                stroke: String::from("rgb(100,100,100)"),
                fill: String::from("rgb(255,255,255)"),
            },
            port_index_label: RectColor {
                stroke: String::from("rgb(0,0,0)"),
                fill: String::from("rgb(0,0,0)"),
            },
            port_subbagde: RectColor {
                stroke: String::from("rgb(240,240,240)"),
                fill: String::from("rgb(25,25,25)"),
            },
            label_subtitle: RectColor {
                stroke: String::from("rgb(40,40,40)"),
                fill: String::from("rgb(40,40,40)"),
            },
            label: RectColor {
                stroke: String::from("rgb(0,0,0)"),
                fill: String::from("rgb(0,0,0)"),
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    pub ports: Ports,
    pub connections: Connections,
    pub grid: GridOptions,
    pub labels: Labels,
    pub ratio: u8,
    pub font: String,
    pub scheme: ColorScheme,
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
            scheme: ColorScheme::default(),
        }
    }
}
