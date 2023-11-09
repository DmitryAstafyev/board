use std::{collections::HashMap, ops::RangeInclusive};

use crate::{
    entity::{port::PortType, Component, Connection, Joint, Signature, SignatureProducer},
    representation::{
        self,
        form::{self, rectangle::Rectangle, Form},
        style::{self, Style},
        Default, Representation,
    },
};
use rand::Rng;

const VERTICAL_OFFSET_BETWEEN_COMPS: i32 = 24;

#[derive(Debug)]
pub struct Composition {
    pub sig: Signature,
    pub components: Vec<Component>,
    pub connections: Vec<Connection>,
    pub repr: Representation,
}

impl Composition {
    pub fn new(sig: Signature) -> Self {
        Self {
            sig,
            components: vec![],
            connections: vec![],
            repr: Composition::init(),
        }
    }

    pub fn dummy(
        producer: &mut SignatureProducer,
        components: RangeInclusive<usize>,
        ports: RangeInclusive<usize>,
    ) -> Self {
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

    // pub fn find_most_linked_component(&self, sig: &Signature) -> Option<&'a Component> {
    //     let mut map: HashMap<usize, (usize, usize)> = HashMap::new();
    //     // Create map first
    //     self.connections.iter().for_each(|connection| {
    //         if let Some((port_type, comp)) = connection.get_linked_to(sig) {
    //             let entry = map.entry(comp.sig.id);
    //             entry
    //                 .and_modify(|(ins, outs)| match port_type {
    //                     PortType::In => *ins += 1,
    //                     PortType::Out => *outs += 1,
    //                 })
    //                 .or_insert(match port_type {
    //                     PortType::In => (1, 0),
    //                     PortType::Out => (0, 1),
    //                 });
    //         }
    //     });
    //     None
    // }
}

impl form::Default for Composition {
    fn init() -> Form {
        Form::Rectangle(Rectangle {
            x: 0,
            y: 0,
            w: 200,
            h: 200,
        })
    }
}

impl style::Default for Composition {
    fn init() -> Style {
        Style {
            stroke_style: String::from("#000000"),
            fill_style: String::from("#FAFAFA"),
        }
    }
}

impl representation::Default for Composition {
    fn init() -> Representation {
        Representation {
            form: <Composition as form::Default>::init(),
            style: <Composition as style::Default>::init(),
        }
    }
}

impl representation::Virtualization for Composition {
    fn calc(&mut self) {
        self.components.iter_mut().for_each(|comp| comp.calc());
        let mut cursor: i32 = 0;
        self.components.iter_mut().for_each(|comp| {
            comp.repr.form.set_coors(None, Some(cursor));
            cursor += (comp.repr.form.box_height() + VERTICAL_OFFSET_BETWEEN_COMPS);
        });
    }
}

impl representation::Rendering for Composition {
    fn render(
        &self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &crate::elements::relative::Relative,
    ) {
        self.repr.style.apply(context);
        self.repr.form.render(context, relative);
        self.components
            .iter()
            .for_each(|c| c.render(context, relative));
    }
}
