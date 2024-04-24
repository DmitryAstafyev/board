use crate::{
    entity::{Signature, SignatureGetter},
    render::Representation,
    state::State,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait IsComponentIncluded<T> {
    fn included_as_component(&self, connection: &T) -> bool;
}

impl IsComponentIncluded<Connection> for &usize {
    fn included_as_component(&self, connection: &Connection) -> bool {
        &&connection.joint_in.component == self || &&connection.joint_out.component == self
    }
}

impl IsComponentIncluded<Representation<Connection>> for &usize {
    fn included_as_component(&self, connection: &Representation<Connection>) -> bool {
        &&connection.origin().joint_in.component == self
            || &&connection.origin().joint_out.component == self
    }
}

pub trait GetIncludedComponent<'a, I, O> {
    fn get_included_component(&self, connection: &'a I) -> Option<&'a O>;
}

impl<'a> GetIncludedComponent<'a, Connection, Connection> for &usize {
    fn get_included_component(&self, connection: &'a Connection) -> Option<&'a Connection> {
        if &&connection.joint_in.component == self || &&connection.joint_out.component == self {
            Some(connection)
        } else {
            None
        }
    }
}

impl<'a> GetIncludedComponent<'a, Representation<Connection>, Connection> for &usize {
    fn get_included_component(
        &self,
        connection: &'a Representation<Connection>,
    ) -> Option<&'a Connection> {
        if &&connection.origin().joint_in.component == self
            || &&connection.origin().joint_out.component == self
        {
            Some(connection.origin())
        } else {
            None
        }
    }
}

pub trait IsPortIncluded<T> {
    fn included_as_port(&self, connection: &T) -> Option<usize>;
}

impl IsPortIncluded<Connection> for &usize {
    fn included_as_port(&self, connection: &Connection) -> Option<usize> {
        if &&connection.joint_in.port == self || &&connection.joint_out.port == self {
            Some(**self)
        } else {
            None
        }
    }
}

impl IsPortIncluded<Representation<Connection>> for &usize {
    fn included_as_port(&self, connection: &Representation<Connection>) -> Option<usize> {
        if &&connection.origin().joint_in.port == self
            || &&connection.origin().joint_out.port == self
        {
            Some(**self)
        } else {
            None
        }
    }
}

pub trait IsInputPort<T> {
    fn is_input_port(&self, connection: &T) -> bool;
}

impl IsInputPort<Connection> for &usize {
    fn is_input_port(&self, connection: &Connection) -> bool {
        &&connection.joint_in.port == self
    }
}

impl IsInputPort<Representation<Connection>> for &usize {
    fn is_input_port(&self, connection: &Representation<Connection>) -> bool {
        &&connection.origin().joint_in.port == self
    }
}

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Connection {
    pub sig: Signature,
    pub joint_in: Joint,
    pub joint_out: Joint,
    pub visibility: bool,
}

impl<'a, 'b: 'a> SignatureGetter<'a, 'b> for Connection {
    fn sig(&'b self) -> &'a Signature {
        &self.sig
    }
}

impl Connection {
    pub fn in_port(&self) -> &usize {
        if let Some(id) = self.joint_in.grouped.as_ref() {
            id
        } else {
            &self.joint_in.port
        }
    }
    pub fn out_port(&self) -> &usize {
        if let Some(id) = self.joint_out.grouped.as_ref() {
            id
        } else {
            &self.joint_out.port
        }
    }
    pub fn in_comp(&self) -> &usize {
        &self.joint_in.component
    }
    pub fn out_comp(&self) -> &usize {
        &self.joint_out.component
    }
    pub fn get_ports(&self) -> [&usize; 2] {
        [&self.joint_in.port, &self.joint_out.port]
    }
    pub fn count(connections: &[Representation<Connection>], component_id: &usize) -> usize {
        connections
            .iter()
            .filter(|c| component_id.included_as_component(c.origin()))
            .count()
    }

    pub fn get_ordered_linked(
        connections: &[Representation<Connection>],
        state: &State,
        // id, IN, OUT
    ) -> Vec<(usize, usize, usize)> {
        let mut map: HashMap<usize, (usize, usize, usize)> = HashMap::new();
        connections.iter().for_each(|c| {
            if state.is_port_owner_filtered(&c.origin().joint_out.component)
                && state.is_port_owner_filtered(&c.origin().joint_in.component)
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
                    .or_insert((c.origin().joint_in.component, 1, 0));
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
        state: &State,
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
                if state.is_port_owner_filtered(&connected_comp_id) {
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
            }
        });
        let mut components: Vec<(usize, usize, usize)> =
            map.into_values().collect::<Vec<(usize, usize, usize)>>();
        components
            .sort_by(|(_, a_in, a_out), (_, b_in, b_out)| (b_in + b_out).cmp(&(a_in + a_out)));
        components
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
