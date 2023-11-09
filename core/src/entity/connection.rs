use std::collections::HashMap;

use crate::{
    entity::{port::PortType, Component, Port, Signature},
    representation::{
        self,
        form::{self, path::Path, Form},
        style::{self, Style},
        Default, Representation,
    },
};
use wasm_bindgen_test::console_log;

#[derive(Debug)]
pub struct Joint {
    pub port: usize,
    pub component: usize,
}

impl Joint {
    pub fn new(port_id: usize, component_id: usize) -> Self {
        Self {
            port: port_id,
            component: component_id,
        }
    }
}
// Function order connection by center like:
//        =
//    = = = = =
//  = = = = = = =
// In center will be bigest and go lowest to left and right
fn order_connections(src: &mut [(usize, usize)]) -> Vec<usize> {
    src.sort_by(|(_, count_a), (_, count_b)| count_a.cmp(count_b));
    let mut ordered: Vec<usize> = vec![];
    src.iter().enumerate().for_each(|(i, (id, _))| {
        if i % 2 == 0 {
            ordered.push(*id);
        } else {
            ordered.insert(0, *id);
        }
    });
    ordered
}
#[derive(Debug)]
pub struct Connection {
    pub sig: Signature,
    pub joint_in: Joint,
    pub joint_out: Joint,
    pub repr: Representation,
}

impl Connection {
    pub fn count(connections: &[Connection], component_id: usize) -> usize {
        connections
            .iter()
            .filter(|c| {
                c.joint_in.component == component_id || c.joint_out.component == component_id
            })
            .count()
    }

    // Returns all linked components to host as (linked IN, linked OUT)
    pub fn linked(
        connections: &[Connection],
        host_id: usize,
        ignore: &[usize],
    ) -> (Vec<usize>, Vec<usize>) {
        let mut map: HashMap<usize, (usize, usize)> = HashMap::new();
        connections.iter().for_each(|c| {
            if c.joint_in.component == host_id || c.joint_out.component == host_id {
                let in_connection = c.joint_in.component != host_id;
                let connected_comp_id = if in_connection {
                    c.joint_in.component
                } else {
                    c.joint_out.component
                };
                let entry = map.entry(connected_comp_id);
                entry
                    .and_modify(|(ins, outs)| {
                        if in_connection {
                            *ins += 1;
                        } else {
                            *outs += 1;
                        }
                    })
                    .or_insert(if in_connection { (1, 0) } else { (0, 1) });
            }
        });
        console_log!("Found: {} linked components", map.len());
        let mut connected_in: Vec<(usize, usize)> = vec![];
        let mut connected_out: Vec<(usize, usize)> = vec![];
        map.iter().for_each(|(k, (ins, outs))| {
            if ignore.contains(k) {
                console_log!("Key: {k} ignored ({ignore:?})");
                return;
            }
            if ins > outs {
                connected_in.push((*k, *ins));
            } else {
                connected_out.push((*k, *ins));
            }
        });
        (
            order_connections(&mut connected_in),
            order_connections(&mut connected_out),
        )
    }

    pub fn new(sig: Signature, joint_in: Joint, joint_out: Joint) -> Self {
        Self {
            sig,
            repr: Connection::init(),
            joint_in,
            joint_out,
        }
    }
}

impl form::Default for Connection {
    fn init() -> Form {
        Form::Path(Path { points: vec![] })
    }
}

impl style::Default for Connection {
    fn init() -> Style {
        Style {
            stroke_style: String::from("#222222"),
            fill_style: String::from("#FFFFFF"),
        }
    }
}

impl representation::Default for Connection {
    fn init() -> Representation {
        Representation {
            form: <Connection as form::Default>::init(),
            style: <Connection as style::Default>::init(),
        }
    }
}

impl representation::Rendering for Connection {
    fn render(
        &self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &crate::elements::relative::Relative,
    ) {
        self.repr.style.apply(context);
        self.repr.form.render(context, relative);
    }
}
