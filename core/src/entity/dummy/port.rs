use std::ops::RangeInclusive;

use crate::entity::{
    dummy::{Dummy, SignatureProducer},
    port::PortType,
    Port, Ports,
};
use rand::Rng;

impl Dummy<Port, ()> for Port {
    fn dummy(producer: &mut SignatureProducer, _: ()) -> Port {
        Port {
            provided_interface: None,
            provided_required_interface: None,
            required_interface: None,
            sig: producer.next(),
            port_type: if rand::random() {
                PortType::In
            } else {
                PortType::Out
            },
            contains: Vec::new(),
            p_connected: 0,
            r_connected: 0,
            visibility: true,
        }
    }
}

impl Dummy<Ports, RangeInclusive<usize>> for Ports {
    fn dummy(producer: &mut SignatureProducer, ports: RangeInclusive<usize>) -> Ports {
        let count = rand::thread_rng().gen_range(ports);
        let mut instance = Self::new();
        for _ in 0..count {
            instance.push(Port::dummy(producer, ()));
        }
        instance
    }
}
