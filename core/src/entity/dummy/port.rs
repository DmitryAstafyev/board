use std::ops::RangeInclusive;

use crate::{
    entity::{
        dummy::{Dummy, SignatureProducer},
        port::PortType,
        Port, Ports,
    },
    representation::Default,
};
use rand::Rng;

impl Dummy<Port, ()> for Port {
    fn dummy(producer: &mut SignatureProducer, _: ()) -> Port {
        Port {
            sig: producer.next(),
            port_type: if rand::random() {
                PortType::In
            } else {
                PortType::Out
            },
            repr: Port::init(),
        }
    }
}

impl Dummy<Ports, RangeInclusive<usize>> for Ports {
    fn dummy(producer: &mut SignatureProducer, ports: RangeInclusive<usize>) -> Ports {
        let count = rand::thread_rng().gen_range(ports);
        let mut instance = Self::new();
        for _ in 0..count {
            instance.ports.push(Port::dummy(producer, ()));
        }
        instance
    }
}
