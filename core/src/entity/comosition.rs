use crate::{
    entity::{Component, Connection, Ports, Signature},
    render::{Render, Representation},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Composition {
    pub sig: Signature,
    pub components: Vec<Representation<Component>>,
    pub connections: Vec<Representation<Connection>>,
    pub compositions: Vec<Representation<Composition>>,
    pub ports: Representation<Ports>,
}

impl Composition {
    pub fn new(sig: Signature) -> Self {
        Self {
            sig,
            components: vec![],
            connections: vec![],
            compositions: vec![],
            ports: Representation::Origin(Ports::new()),
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

    pub fn to_component(&self) -> Component {
        Component {
            sig: self.sig.clone(),
            ports: Representation::Render(Render::<Ports>::new(self.ports.origin().clone())),
            composition: true,
        }
    }
}
