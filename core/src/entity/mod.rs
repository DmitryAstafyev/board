mod comosition;
mod component;
mod connection;
mod port;

pub use comosition::Composition;
pub use component::Component;
pub use connection::{Connection, Joint};
pub use port::{Port, Ports};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Signature {
    pub id: usize,
    pub class_name: String,
}

pub struct SignatureProducer {
    current: usize,
}

impl SignatureProducer {
    pub fn new() -> Self {
        Self { current: 0 }
    }
    // This method is used for testing only with Dummy<T>
    pub fn next(&mut self) -> Signature {
        self.current += 1;
        Signature {
            id: self.current,
            class_name: format!("DummyClass_{}", self.current),
        }
    }
}

pub trait Dummy<T> {
    fn dummy(producer: &mut SignatureProducer) -> T;
}

// pub fn dummy(
//     compositions: usize,
//     components_per_composition: (usize, usize),
//     ports_per_component: (usize, usize),
//     min_connections_per_component: usize,
// ) -> Vec<Composition> {
//     let mut sig_prod = SignatureProducer::new();
//     let mut stored_components: Vec<Vec<Component>> = vec![];
//     for _ in 0..compositions {
//         let mut components: Vec<Component> = vec![];
//         let components_count = rand::thread_rng()
//             .gen_range(components_per_composition.0..components_per_composition.1);
//         for _ in 0..components_count {
//             components.push(Component::new(sig_prod.next()));
//         }
//         let mut ports: Vec<Port> = vec![];
//         components.iter().for_each(|component| {
//             let ports_count =
//                 rand::thread_rng().gen_range(ports_per_component.0..ports_per_component.1);
//             for _ in 0..ports_count {
//                 ports.push(Port::dummy(&mut sig_prod));
//             }

//             // for _ in 0..connections_count {
//             //     if used_port >= ports.len() {
//             //         break;
//             //     }
//             //     let joint_in = Joint::new(&ports[used_port], &component);
//             //     connections.push(Connection::new(sig_prod.next(), joint_in, joint_out));
//             // }
//         });
//         let mut used_ports: usize = 0;
//         let ports_per_component = ports.len() / components_count;
//         let mut connections: Vec<Connection> = vec![];
//         for comp_pair in components.chunks(2) {
//             let take_ports = rand::thread_rng()
//                 .gen_range(min_connections_per_component * 2..ports_per_component);
//             if used_ports + take_ports >= ports.len() {
//                 break;
//             }
//             let ports = &ports[used_ports..used_ports + take_ports];
//             used_ports += take_ports + 1;
//             for ports_pair in ports.chunks(2) {
//                 connections.push(Connection::new(
//                     sig_prod.next(),
//                     Joint::new(&ports_pair[0], &comp_pair[0]),
//                     Joint::new(&ports_pair[1], &comp_pair[1]),
//                 ));
//             }
//         }
//         let mut composition = Composition::new(sig_prod.next());
//         composition.link_components(&mut components);
//         composition.link_connections(&mut connections);
//         stored_components.push(components);
//     }
//     vec![]
// }
