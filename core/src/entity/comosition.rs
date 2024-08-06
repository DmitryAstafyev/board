use crate::{
    entity::{
        Component, Connection, GetIncludedComponent, IsPortIncluded, Port, Ports, Signature,
        SignatureEqual, SignatureGetter,
    },
    render::{options::Options, Render, Representation},
};
use serde::{Deserialize, Serialize};

use super::EntityProps;

#[derive(Debug, Deserialize, Serialize)]
pub struct Composition {
    pub sig: Signature,
    pub components: Vec<Representation<Component>>,
    pub connections: Vec<Representation<Connection>>,
    pub compositions: Vec<Representation<Composition>>,
    pub ports: Representation<Ports>,
    pub parent: Option<usize>,
}

impl<'a, 'b: 'a> SignatureGetter<'a, 'b> for Composition {
    fn sig(&'b self) -> &'a Signature {
        &self.sig
    }
}

impl Composition {
    pub fn new(sig: Signature) -> Self {
        Self {
            sig,
            components: Vec::new(),
            connections: Vec::new(),
            compositions: Vec::new(),
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
            Connection::count(connections, &a.sig().id)
                .cmp(&Connection::count(connections, &b.sig().id))
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

    pub fn find_connections_by_port(&self, id: &usize) -> Vec<&Connection> {
        self.connections
            .iter()
            .filter(|c| id.included_as_port(*c).is_some())
            .map(|rep| rep.origin())
            .collect::<Vec<&Connection>>()
    }

    // pub fn find_connected_port(&self, id: &usize) -> Option<usize> {
    //     self.connections.iter().find_map(|c| id.included_as_port(c))
    // }

    // pub fn find_ports_by_component(&self, id: &usize) -> Vec<&usize> {
    //     self.connections
    //         .iter()
    //         .filter(|c| id.included_as_component(*c))
    //         .flat_map(|conn| conn.origin().get_ports())
    //         .collect::<Vec<&usize>>()
    // }

    pub fn find_connections_by_component(&self, id: &usize) -> Vec<&Connection> {
        self.connections
            .iter()
            .filter_map(|c| id.get_included_component(c))
            .collect()
    }

    pub fn get_component(&self, id: &usize) -> Option<&Component> {
        self.components
            .iter()
            .find_map(|c| id.get_if_equal(c.origin()))
    }

    pub fn get_port(&self, port_id: &usize) -> Option<&Port> {
        self.ports.origin().find(port_id).map(|r| r.origin())
    }

    pub fn get_ports_props(&self) -> EntityProps {
        let mut props = self.ports.origin().get_props();
        self.components.iter().for_each(|c| {
            props.merge(c.origin().get_ports_props());
        });
        props.dedup()
    }

    pub fn get_comps_props(&self) -> EntityProps {
        let mut props = EntityProps::default();
        self.components.iter().for_each(|c| {
            if !props.class_name.contains(&c.sig().class_name) {
                props.class_name.push(c.sig().class_name.clone());
            }
            if !props.short_name.contains(&c.sig().short_name) {
                props.short_name.push(c.sig().short_name.clone());
            }
        });
        props.dedup()
    }

    pub fn get_label(&self, options: &Options) -> String {
        self.sig.as_label(
            options.labels.composition_short_name,
            options.labels.comp_label_max_len,
        )
    }
}
