use crate::{
    entity::{
        dummy::{Dummy, SignatureProducer},
        Component, Ports,
    },
    render::Representation,
};
use std::ops::RangeInclusive;

impl Dummy<Component, RangeInclusive<usize>> for Component {
    fn dummy(producer: &mut SignatureProducer, ports: RangeInclusive<usize>) -> Self {
        Self {
            sig: producer.next(),
            ports: Representation::Origin(Ports::dummy(producer, ports)),
            composition: false,
        }
    }
}
