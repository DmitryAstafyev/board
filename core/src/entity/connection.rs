use crate::{entity::Signature, render::Representation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
pub struct Connection {
    pub sig: Signature,
    pub joint_in: Joint,
    pub joint_out: Joint,
}

impl Connection {
    pub fn count(connections: &[Representation<Connection>], component_id: usize) -> usize {
        connections
            .iter()
            .filter(|c| {
                c.origin().joint_in.component == component_id
                    || c.origin().joint_out.component == component_id
            })
            .count()
    }

    // Returns ids of all linked components to host as (linked IN, linked OUT)
    pub fn linked(
        connections: &[Representation<Connection>],
        host_id: usize,
        ignore: &[usize],
    ) -> (Vec<usize>, Vec<usize>) {
        let mut map: HashMap<usize, (usize, usize)> = HashMap::new();
        connections.iter().for_each(|c| {
            if c.origin().joint_in.component == host_id || c.origin().joint_out.component == host_id
            {
                let in_connection = c.origin().joint_in.component != host_id;
                let connected_comp_id = if in_connection {
                    c.origin().joint_in.component
                } else {
                    c.origin().joint_out.component
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
        let mut connected_in: Vec<(usize, usize)> = vec![];
        let mut connected_out: Vec<(usize, usize)> = vec![];
        map.iter().for_each(|(k, (ins, outs))| {
            if ignore.contains(k) {
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
            joint_in,
            joint_out,
        }
    }
}
