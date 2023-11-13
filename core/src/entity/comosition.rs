use crate::{
    entity::{Component, Connection, Signature},
    render::Representation,
};

#[derive(Debug)]
pub struct Composition {
    pub sig: Signature,
    pub components: Vec<Representation<Component>>,
    pub connections: Vec<Representation<Connection>>,
}

impl Composition {
    pub fn new(sig: Signature) -> Self {
        Self {
            sig,
            components: vec![],
            connections: vec![],
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
}
