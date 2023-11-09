use crate::{
    entity::{Component, Connection, Signature},
    representation::{
        self,
        form::{self, path::Point, rectangle::Rectangle, Form},
        style::{self, Style},
        Default, Representation,
    },
};

use wasm_bindgen_test::console_log;

const VERTICAL_OFFSET_BETWEEN_COMPS: i32 = 24;
const HORIZONTAL_OFFSET_BETWEEN_COMPS: i32 = 64;

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

    pub fn order(&mut self) {
        let connections = &self.connections;
        self.components.sort_by(|a, b| {
            Connection::count(connections, a.sig.id).cmp(&Connection::count(connections, b.sig.id))
        });
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
        console_log!("Calc of composition");
        self.components.iter_mut().for_each(|comp| comp.calc());
        // let mut cursor: i32 = 0;
        // self.components.iter_mut().for_each(|comp| {
        //     comp.repr.form.set_coors(None, Some(cursor));
        //     cursor += comp.repr.form.box_height() + VERTICAL_OFFSET_BETWEEN_COMPS;
        // });
        console_log!("Ordered");
        self.order();
        let connections = &self.connections;
        let mut calculated: Vec<usize> = vec![];
        console_log!("Starting");
        let data = self
            .components
            .iter()
            .map(|comp| {
                (
                    comp.sig.id,
                    comp.repr.form.get_coors(),
                    (comp.repr.form.box_width(), comp.repr.form.box_height()),
                )
            })
            .collect::<Vec<(usize, (i32, i32), (i32, i32))>>();
        for (current_id, (current_x, current_y), (current_w, current_h)) in data.iter() {
            // if calculated.contains(current_id) {
            //     console_log!("Next: skip");
            //     continue;
            // }
            let mut in_box: Vec<usize> = vec![*current_id];
            console_log!("Next: {}", current_id);
            let (linked_in, linked_out) = Connection::linked(connections, *current_id, &calculated);
            let current_width = current_w + VERTICAL_OFFSET_BETWEEN_COMPS;
            // Order on a right side
            let mut total_height = 0;
            self.components.iter().for_each(|c| {
                if linked_in.contains(&c.sig.id) {
                    total_height += c.repr.form.box_height() + VERTICAL_OFFSET_BETWEEN_COMPS;
                }
            });
            let mut y = current_y - (total_height - VERTICAL_OFFSET_BETWEEN_COMPS - current_h) / 2;
            self.components.iter_mut().for_each(|comp| {
                if linked_in.contains(&comp.sig.id) {
                    comp.repr.form.set_coors(
                        Some(current_x + current_width + HORIZONTAL_OFFSET_BETWEEN_COMPS),
                        Some(y),
                    );
                    y += comp.repr.form.box_height() + VERTICAL_OFFSET_BETWEEN_COMPS;
                    calculated.push(comp.sig.id);
                    in_box.push(comp.sig.id);
                }
            }); // Order of a left side
            let mut total_height = 0;
            self.components.iter().for_each(|c| {
                if linked_out.contains(&c.sig.id) {
                    total_height += c.repr.form.box_height() + VERTICAL_OFFSET_BETWEEN_COMPS;
                }
            });
            let mut y = current_y - (total_height - VERTICAL_OFFSET_BETWEEN_COMPS - current_h) / 2;
            self.components.iter_mut().for_each(|comp| {
                if linked_out.contains(&comp.sig.id) {
                    comp.repr.form.set_coors(
                        Some(current_x - current_width - HORIZONTAL_OFFSET_BETWEEN_COMPS),
                        Some(y),
                    );
                    y += comp.repr.form.box_height() + VERTICAL_OFFSET_BETWEEN_COMPS;
                    calculated.push(comp.sig.id);
                    in_box.push(comp.sig.id);
                }
            });
            calculated.push(*current_id);
        }
        console_log!("Link ports");
        self.connections.iter_mut().for_each(|conn| {
            if let Form::Path(path) = &mut conn.repr.form {
                if let (Some(ins), Some(outs)) = (
                    self.components
                        .iter()
                        .find(|comp| comp.sig.id == conn.joint_in.component),
                    self.components
                        .iter()
                        .find(|comp| comp.sig.id == conn.joint_out.component),
                ) {
                    if let (Some(port_in), Some(port_out)) = (
                        ins.ports.find(conn.joint_in.port),
                        outs.ports.find(conn.joint_out.port),
                    ) {
                        let port_in = port_in.repr.form.get_coors();
                        let port_out = port_out.repr.form.get_coors();
                        path.points.push(Point {
                            x: ins.relative().x(port_in.0),
                            y: ins.relative().y(port_in.1),
                        });
                        path.points.push(Point {
                            x: outs.relative().x(port_out.0),
                            y: outs.relative().y(port_out.1),
                        });
                    } else {
                        console_log!("No ports has been found :/");
                    }
                }
            }
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
        self.connections
            .iter()
            .for_each(|c| c.render(context, relative));
    }
}
