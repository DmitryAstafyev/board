use crate::{entity::Signature, render::Representation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Joint {
    pub port: usize,
    pub component: usize,
    #[serde(skip_serializing, skip_deserializing)]
    pub grouped: Option<usize>,
}

impl Joint {
    pub fn new(port_id: usize, component_id: usize) -> Self {
        Self {
            port: port_id,
            component: component_id,
            grouped: None,
        }
    }
}
// Function order connection by center like:
//        =
//    = = = = =
//  = = = = = = =
// In center will be bigest and go lowest to left and right
fn _order_connections(src: &mut [(usize, usize)]) -> Vec<usize> {
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
    pub visibility: bool,
}

impl Connection {
    pub fn get_joint_in_port(&self) -> &usize {
        if let Some(id) = self.joint_in.grouped.as_ref() {
            id
        } else {
            &self.joint_in.port
        }
    }
    pub fn get_joint_out_port(&self) -> &usize {
        if let Some(id) = self.joint_out.grouped.as_ref() {
            id
        } else {
            &self.joint_out.port
        }
    }
    pub fn count(connections: &[Representation<Connection>], component_id: usize) -> usize {
        connections
            .iter()
            .filter(|c| {
                c.origin().joint_in.component == component_id
                    || c.origin().joint_out.component == component_id
            })
            .count()
    }

    pub fn get_ordered_linked(
        connections: &[Representation<Connection>],
        ignore: &[usize],
        // id, IN, OUT
    ) -> Vec<(usize, usize, usize)> {
        let mut map: HashMap<usize, (usize, usize, usize)> = HashMap::new();
        connections.iter().for_each(|c| {
            if !ignore.contains(&c.origin().joint_out.component)
                && !ignore.contains(&c.origin().joint_in.component)
            {
                map.entry(c.origin().joint_out.component)
                    .and_modify(|(_, _, outs)| {
                        *outs += 1;
                    })
                    .or_insert((c.origin().joint_out.component, 0, 1));
                map.entry(c.origin().joint_in.component)
                    .and_modify(|(_, ins, _)| {
                        *ins += 1;
                    })
                    .or_insert((c.origin().joint_in.component, 0, 1));
            }
        });
        let mut components: Vec<(usize, usize, usize)> =
            map.into_values().collect::<Vec<(usize, usize, usize)>>();
        components
            .sort_by(|(_, a_in, a_out), (_, b_in, b_out)| (b_in + b_out).cmp(&(a_in + a_out)));
        components
    }

    pub fn get_ordered_linked_to(
        connections: &[Representation<Connection>],
        host_id: usize,
        ignore: &[usize],
        // id, IN, OUT
    ) -> Vec<(usize, usize, usize)> {
        let mut map: HashMap<usize, (usize, usize, usize)> = HashMap::new();
        connections.iter().for_each(|c| {
            if (c.origin().joint_in.component == host_id
                && !ignore.contains(&c.origin().joint_out.component))
                || (c.origin().joint_out.component == host_id
                    && !ignore.contains(&c.origin().joint_in.component))
            {
                let in_connection = c.origin().joint_in.component != host_id;
                let connected_comp_id = if in_connection {
                    c.origin().joint_in.component
                } else {
                    c.origin().joint_out.component
                };
                let entry = map.entry(connected_comp_id);
                entry
                    .and_modify(|(_, ins, outs)| {
                        if in_connection {
                            *ins += 1;
                        } else {
                            *outs += 1;
                        }
                    })
                    .or_insert(if in_connection {
                        (connected_comp_id, 1, 0)
                    } else {
                        (connected_comp_id, 0, 1)
                    });
            }
        });
        let mut components: Vec<(usize, usize, usize)> =
            map.into_values().collect::<Vec<(usize, usize, usize)>>();
        components
            .sort_by(|(_, a_in, a_out), (_, b_in, b_out)| (b_in + b_out).cmp(&(a_in + a_out)));
        components
    }

    // Returns ids of all linked components to host as (linked IN, linked OUT)
    pub fn _linked(
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
        connected_in.sort_by(|(_, a), (_, b)| b.cmp(a));
        connected_out.sort_by(|(_, a), (_, b)| b.cmp(a));
        (
            connected_in.iter().map(|(k, _)| *k).collect::<Vec<usize>>(),
            connected_out
                .iter()
                .map(|(k, _)| *k)
                .collect::<Vec<usize>>(),
        )
    }

    pub fn hide(&mut self) {
        self.visibility = false;
    }

    pub fn new(sig: Signature, joint_in: Joint, joint_out: Joint) -> Self {
        Self {
            sig,
            joint_in,
            joint_out,
            visibility: true,
        }
    }
}
