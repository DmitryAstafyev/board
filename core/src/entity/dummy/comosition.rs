use crate::entity::{
    dummy::{Dummy, SignatureProducer},
    port::PortType,
    Component, Composition, Connection, Joint,
};
use rand::Rng;
use std::ops::RangeInclusive;

impl Dummy<Composition, (RangeInclusive<usize>, RangeInclusive<usize>)> for Composition {
    fn dummy(
        producer: &mut SignatureProducer,
        // (max_components, max_ports_per_component)
        options: (RangeInclusive<usize>, RangeInclusive<usize>),
    ) -> Self {
        let (components, ports) = options;
        let mut instance = Self::new(producer.next());
        let mut connections: Vec<Connection> = vec![];
        let count = rand::thread_rng().gen_range(components);
        for _ in 0..count {
            instance.push_component(Component::dummy(producer, ports.clone()));
        }
        for comp in instance.components.chunks_mut(2) {
            if comp.is_empty() || comp.len() != 2 {
                break;
            }
            let left = &comp[0];
            let right = &comp[1];
            let min = [
                left.origin().ports.origin().len(),
                right.origin().ports.origin().len(),
            ]
            .iter()
            .cloned()
            .min()
            .unwrap_or(0);
            for _ in 0..min {
                let selected: usize = rand::thread_rng().gen_range(0..min);
                comp[0]
                    .origin_mut()
                    .ports
                    .origin_mut()
                    .get_mut(selected)
                    .set_type(PortType::In);
                comp[1]
                    .origin_mut()
                    .ports
                    .origin_mut()
                    .get_mut(selected)
                    .set_type(PortType::Out);
                let left = &comp[0];
                let right = &comp[1];
                connections.push(Connection::new(
                    producer.next(),
                    Joint::new(
                        left.origin().ports.origin().get(selected).sig.id,
                        left.sig().id,
                    ),
                    Joint::new(
                        right.origin().ports.origin().get(selected).sig.id,
                        right.sig().id,
                    ),
                ))
            }
        }
        connections
            .drain(..)
            .for_each(|c| instance.push_connection(c));
        instance
    }
}
