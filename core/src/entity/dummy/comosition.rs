use std::ops::RangeInclusive;

use crate::entity::{
    dummy::{Dummy, SignatureProducer},
    Component, Composition, Connection, Joint,
};
use rand::Rng;

impl Dummy<Composition, (RangeInclusive<usize>, RangeInclusive<usize>)> for Composition {
    fn dummy(
        producer: &mut SignatureProducer,
        options: (RangeInclusive<usize>, RangeInclusive<usize>),
    ) -> Self {
        let (components, ports) = options;
        let mut instance = Self::new(producer.next());
        let count = rand::thread_rng().gen_range(components);
        for _ in 0..count {
            instance
                .components
                .push(Component::dummy(producer, ports.clone()));
        }
        for comp in instance.components.chunks(2) {
            if comp.is_empty() || comp.len() != 2 {
                break;
            }
            let left = &comp[0];
            let right = &comp[1];
            let min = [left.ports.len(), right.ports.len()]
                .iter()
                .cloned()
                .min()
                .unwrap_or(0);
            let count = rand::thread_rng().gen_range(0..min);
            for _ in 0..count {
                let selected: usize = rand::thread_rng().gen_range(0..min);
                instance.connections.push(Connection::new(
                    producer.next(),
                    Joint::new(left.ports.get(selected).sig.id, left.sig.id),
                    Joint::new(right.ports.get(selected).sig.id, right.sig.id),
                ))
            }
        }
        instance
    }
}
