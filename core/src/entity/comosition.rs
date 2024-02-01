use crate::{
    entity::{Component, Connection, Ports, Signature},
    render::{options::Options, Render, Representation},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Composition {
    pub sig: Signature,
    pub components: Vec<Representation<Component>>,
    pub connections: Vec<Representation<Connection>>,
    pub compositions: Vec<Representation<Composition>>,
    pub ports: Representation<Ports>,
    pub parent: Option<usize>,
}

impl Composition {
    pub fn new(sig: Signature) -> Self {
        Self {
            sig,
            components: vec![],
            connections: vec![],
            compositions: vec![],
            ports: Representation::Origin(Ports::new()),
            parent: None,
        }
    }

    pub fn push_component(&mut self, component: Component) {
        self.components.push(Representation::Origin(component))
    }

    pub fn push_connection(&mut self, connection: Connection) {
        self.connections.push(Representation::Origin(connection))
    }

    pub fn order(&mut self) {
        let connections = &self.connections;
        self.components.sort_by(|a, b| {
            Connection::count(connections, a.origin().sig.id)
                .cmp(&Connection::count(connections, b.origin().sig.id))
        });
    }

    pub fn to_component(&self, options: &Options) -> Component {
        Component {
            sig: self.sig.clone(),
            ports: Representation::Render(Render::<Ports>::new(
                self.ports.origin().clone(),
                options,
            )),
            composition: true,
        }
    }

    pub fn find_connection_by_port(&self, port_id: usize) -> Option<&Connection> {
        self.connections
            .iter()
            .find(|conn| {
                conn.origin().joint_in.port == port_id || conn.origin().joint_out.port == port_id
            })
            .map(|rep| rep.origin())
    }

    pub fn find_connected_port(&self, port_id: usize) -> Option<usize> {
        self.connections
            .iter()
            .find(|conn| {
                conn.origin().joint_in.port == port_id || conn.origin().joint_out.port == port_id
            })
            .map(|conn| {
                if conn.origin().joint_in.port == port_id {
                    conn.origin().joint_out.port
                } else {
                    conn.origin().joint_in.port
                }
            })
    }

    pub fn find_ports_by_component(&self, component_id: usize) -> Vec<&usize> {
        self.connections
            .iter()
            .filter(|conn| {
                conn.origin().joint_in.component == component_id
                    || conn.origin().joint_out.component == component_id
            })
            .flat_map(|conn| [&conn.origin().joint_in.port, &conn.origin().joint_out.port])
            .collect::<Vec<&usize>>()
    }

    pub fn find_connections_by_component(&self, component_id: usize) -> Vec<&Connection> {
        self.connections
            .iter()
            .filter(|conn| {
                conn.origin().joint_in.component == component_id
                    || conn.origin().joint_out.component == component_id
            })
            .map(|rep| rep.origin())
            .collect()
    }

    pub fn get_component(&self, id: usize) -> Option<&Component> {
        self.components
            .iter()
            .find(|comp| comp.origin().sig.id == id)
            .map(|comp| comp.origin())
    }
}
