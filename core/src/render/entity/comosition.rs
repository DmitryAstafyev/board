use wasm_bindgen_test::console_log;

use crate::{
    entity::{
        dummy::SignatureProducer, Component, Composition, Connection, Joint, Port, PortType, Ports,
    },
    error::E,
    render::{
        elements,
        form::{button, Button, Path, Point, Rectangle},
        grid::{Cell, ElementCoors, ElementType, Layout, CELL},
        options::{ConnectionsAlign, Options},
        Container, Form, Grid, Relative, Render, Representation, Style, View,
    },
    state::State,
};
use std::collections::HashMap;

enum Entry<'a> {
    Component(&'a Representation<Component>),
    Composition(&'a Representation<Composition>),
}

impl<'a> Entry<'a> {
    pub fn ports(&self) -> &'a Representation<Ports> {
        match self {
            Entry::Component(c) => &c.origin().ports,
            Entry::Composition(c) => &c.origin().ports,
        }
    }
    pub fn own_relative(&self) -> Result<Relative, E> {
        Ok(match self {
            Entry::Component(c) => c.render()?.own_relative(),
            Entry::Composition(c) => c.render()?.own_relative(),
        })
    }
    pub fn _id(&self) -> usize {
        match self {
            Entry::Component(c) => c.origin().sig.id,
            Entry::Composition(c) => c.origin().sig.id,
        }
    }
}

fn find<'a>(
    components: &'a [Representation<Component>],
    compositions: &'a [Representation<Composition>],
    id: &usize,
) -> Option<Entry<'a>> {
    components
        .iter()
        .find(|c| c.origin().sig.id == *id)
        .map(Entry::Component)
        .or_else(|| {
            compositions
                .iter()
                .find(|c| c.origin().sig.id == *id)
                .map(Entry::Composition)
        })
}

fn find_port<'a>(
    components: &'a [Representation<Component>],
    compositions: &'a [Representation<Composition>],
    parent_id: &usize,
    port_id: &usize,
) -> Option<&'a Port> {
    find(components, compositions, parent_id)
        .and_then(|entry| entry.ports().origin().find(port_id).map(|p| p.origin()))
}

impl Render<Composition> {
    pub fn new(mut entity: Composition, options: &Options) -> Self {
        let mut sig_producer = SignatureProducer::new(100000000);
        let (added_connections, mut added_ports) =
            group_ports(&entity.connections, &mut sig_producer);
        entity.connections.extend(added_connections);
        while let Some((component_id, added_port)) = added_ports.pop() {
            if let Some(component) = entity
                .components
                .iter_mut()
                .find(|c| c.origin().sig.id == component_id)
            {
                component
                    .origin_mut()
                    .ports
                    .origin_mut()
                    .hide(&added_port.origin().contains);
                component.origin_mut().ports.origin_mut().add(added_port);
            }
        }
        entity.components = entity
            .components
            .drain(..)
            .map(|r| {
                if let Representation::Origin(component) = r {
                    Representation::Render(Render::<Component>::new(component, options, None))
                } else {
                    r
                }
            })
            .collect::<Vec<Representation<Component>>>();
        entity.compositions = entity
            .compositions
            .drain(..)
            .map(|r| {
                if let Representation::Origin(composition) = r {
                    Representation::Render(Render::<Composition>::new(composition, options))
                } else {
                    r
                }
            })
            .collect::<Vec<Representation<Composition>>>();
        entity.connections = entity
            .connections
            .drain(..)
            .map(|r| {
                if let Representation::Origin(connection) = r {
                    Representation::Render(Render::<Connection>::new(connection))
                } else {
                    r
                }
            })
            .collect::<Vec<Representation<Connection>>>();
        entity.ports = if let Representation::Origin(ports) = entity.ports {
            Representation::Render(Render::<Ports>::new(ports, options))
        } else {
            entity.ports
        };

        let id = entity.sig.id;
        let parent = entity.parent;
        Self {
            entity,
            view: View {
                container: Container {
                    form: Form::Rectangle(
                        ElementType::Composition,
                        Rectangle {
                            id: id.to_string(),
                            x: 0,
                            y: 0,
                            w: 100,
                            h: 100,
                        },
                    ),
                    style: Style {
                        stroke_style: String::from("rgb(0,0,0)"),
                        fill_style: String::from("rgb(200,200,230)"),
                    },
                    hover: None,
                },
                elements: if let Some(id) = parent {
                    vec![Container {
                        form: Form::Button(
                            ElementType::Element,
                            Button {
                                id: format!("back::{id}"),
                                x: 0,
                                y: 0,
                                w: 0,
                                h: 0,
                                label: id.to_string(),
                                align: button::Align::Right,
                                padding: 3,
                            },
                        ),
                        style: Style {
                            stroke_style: String::from("rgb(0,0,0)"),
                            fill_style: String::from("rgb(100,150,255)"),
                        },
                        hover: None,
                    }]
                } else {
                    vec![]
                },
            },
            hidden: false,
        }
    }

    pub fn align_to_grid(&mut self, grid: &Grid) -> Result<(), E> {
        for comp in self.entity.components.iter_mut() {
            let relative = grid.relative(comp.origin().sig.id);
            comp.render_mut()?
                .view
                .container
                .set_coors(Some(relative.x(0)), Some(relative.y(0)));
        }
        // Align to grid nested compositions
        for composition in self.entity.compositions.iter_mut() {
            let render = composition.render_mut()?;
            if !render.hidden {
                render.align_to_grid(grid)?;
            }
        }
        let relative = grid.relative(self.entity.sig.id);
        self.view
            .container
            .set_coors(Some(relative.x(0)), Some(relative.y(0)));
        Ok(())
    }

    pub fn setup_connections(&mut self, grid: &Grid, options: &Options) -> Result<(), E> {
        let components = &self.entity.components;
        let compositions = &self.entity.compositions;
        let mut failed: Vec<&Connection> = vec![];
        for conn in self.entity.connections.iter_mut() {
            if let (Some(ins), Some(outs)) = (
                find(components, compositions, &conn.origin().joint_in.component),
                find(components, compositions, &conn.origin().joint_out.component),
            ) {
                if let (Some(port_in), Some(port_out)) = (
                    ins.ports()
                        .origin()
                        .find_visible(conn.origin().joint_in.port),
                    outs.ports()
                        .origin()
                        .find_visible(conn.origin().joint_out.port),
                ) {
                    let coors_port_in = port_in.render()?.view.container.get_coors();
                    let coors_port_out = port_out.render()?.view.container.get_coors();
                    let relative_inns = ins.own_relative()?;
                    let relative_outs = outs.own_relative()?;
                    let points: Vec<Point> = match options.connections.align {
                        ConnectionsAlign::Straight => {
                            let size_port_in = port_in.render()?.view.container.get_box_size();
                            let size_port_out = port_out.render()?.view.container.get_box_size();

                            vec![
                                Point {
                                    x: relative_inns.x(coors_port_in.0)
                                        + if matches!(port_in.origin().port_type, PortType::Out) {
                                            size_port_in.0
                                        } else {
                                            0
                                        },
                                    y: relative_inns.y(coors_port_in.1) + size_port_in.1 / 2,
                                },
                                Point {
                                    x: relative_outs.x(coors_port_out.0)
                                        + if matches!(port_out.origin().port_type, PortType::Out) {
                                            size_port_out.0
                                        } else {
                                            0
                                        },
                                    y: relative_outs.y(coors_port_out.1) + size_port_out.1 / 2,
                                },
                            ]
                        }
                        ConnectionsAlign::Streamlined => {
                            let a = Cell::new(
                                relative_inns.x(coors_port_in.0) as u32,
                                relative_inns.y(coors_port_in.1) as u32,
                                grid,
                            )?;
                            let b = Cell::new(
                                relative_outs.x(coors_port_out.0) as u32,
                                relative_outs.y(coors_port_out.1) as u32,
                                grid,
                            )?;
                            let (left, right) = if a.x < b.x { (a, b) } else { (b, a) };
                            let a = (left.x, left.y);
                            let b = (right.x, left.y);
                            let c = (right.x, right.y);
                            let mut coors: Vec<(u32, u32)> = vec![];
                            if let Err(e) = Cell::normalize(&a, &b, grid, &mut coors) {
                                console_log!("Error: {e}");
                                return Ok(());
                            }
                            if let Err(e) = Cell::normalize(&b, &c, grid, &mut coors) {
                                console_log!("Error: {e}");
                                return Ok(());
                            }
                            fn coors_to_px(cell: &u32) -> i32 {
                                (*cell as i32 * CELL as i32) + ((CELL as f64) / 2.0).ceil() as i32
                            }
                            coors
                                .iter()
                                .map(|(x, y)| Point {
                                    x: coors_to_px(x),
                                    y: coors_to_px(y),
                                })
                                .collect::<Vec<Point>>()
                        }
                    };
                    let path = Path::new(conn.origin().sig.id.to_string(), points);
                    conn.render_mut()?
                        .view
                        .container
                        .set_form(Form::Path(ElementType::Connection, path));
                } else {
                    failed.push(conn.origin());
                }
            }
        }
        if !failed.is_empty() {
            console_log!(
                "Fail to find ports for connections: {:?}",
                failed
                    .iter()
                    .map(|c| format!("conn: {}/{};", c.sig.id, c.sig.class_name))
            );
        }
        Ok(())
    }

    pub fn calc(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        grid: &mut Grid,
        expanded: &[usize],
        relative: &Relative,
        options: &Options,
    ) -> Result<(), E> {
        // Create composition grid
        let mut composition_grid = Grid::new(&options.grid);
        // Order components by connections number
        self.entity.order();
        for composition in self.entity.compositions.iter_mut() {
            if expanded.contains(&composition.origin().sig.id) {
                composition.render_mut()?.calc(
                    context,
                    &mut composition_grid,
                    expanded,
                    relative,
                    options,
                )?;
                composition.render_mut()?.show();
            } else {
                self.entity
                    .components
                    .push(Representation::Render(Render::<Component>::new(
                        composition.origin().to_component(options),
                        options,
                        Some(ElementType::Composition),
                    )));
                composition.render_mut()?.hide();
            }
        }
        for component in self.entity.components.iter_mut() {
            component.render_mut()?.calc(context, relative, options)?;
        }
        // Get dependencies data (list of components with IN / OUT connections)
        let mut dependencies: Vec<(usize, usize)> = vec![];
        let mut located: Vec<usize> = vec![];
        let ordered_linked = Connection::get_ordered_linked(&self.entity.connections, &[]);
        for (host_id, _, _) in ordered_linked.iter() {
            if located.contains(host_id) {
                continue;
            }
            let linked =
                Connection::get_ordered_linked_to(&self.entity.connections, *host_id, &located);
            if let Some((id, _, _)) = linked.first() {
                dependencies.push((*host_id, *id));
                located = [located, vec![*host_id, *id]].concat();
            }
        }
        // Get pairs grids
        for (a_id, b_id) in dependencies {
            let a = get_forms_by_ids(&self.entity.components, &[a_id])?;
            let b = get_forms_by_ids(&self.entity.components, &[b_id])?;
            let component_grid = Grid::from(Layout::Pair(a, b), &options.grid)?;
            composition_grid.insert(&component_grid);
        }
        for component in self.entity.components.iter() {
            if !located.contains(&component.origin().sig.id) {
                let component_grid = Grid::from(
                    Layout::Pair(
                        get_forms_by_ids(&self.entity.components, &[component.origin().sig.id])?,
                        [].to_vec(),
                    ),
                    &options.grid,
                )?;
                composition_grid.insert(&component_grid);
            }
        }
        // Align to composition grid
        self.align_to_grid(&composition_grid)?;
        let grid_size = composition_grid.get_size_px();
        let grid_height_px =
            composition_grid.set_min_height(self.entity.ports.render()?.height() as u32);
        self.view
            .container
            .set_box_size(Some(grid_size.0 as i32), Some(grid_height_px as i32));
        // Calc ports
        let self_relative = self.relative(relative);
        self.entity.ports.render_mut()?.calc(
            context,
            self.view.container.get_box_size().0,
            &self_relative,
            options,
        )?;
        // Add composition as itself into grid
        composition_grid.insert_self(self.entity.sig.id, ElementType::Composition);
        if let Some(container) = self.view.elements.first_mut() {
            container.set_coors(Some(grid_size.0 as i32), None);
        }
        self.setup_connections(&composition_grid, options)?;
        // Add into global
        grid.insert(&composition_grid);
        Ok(())
    }

    pub fn draw(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
        targets: &Vec<usize>,
        options: &Options,
        state: &State,
    ) -> Result<(), E> {
        if !targets.contains(&self.entity.sig.id) || self.hidden {
            return Ok(());
        }
        self.view.render(context, relative);
        let self_relative = self.relative(relative);
        for component in self
            .entity
            .components
            .iter_mut()
            .filter(|comp| targets.contains(&comp.origin().sig.id))
        {
            component
                .render_mut()?
                .draw(context, relative, options, state)?;
        }
        for composition in self
            .entity
            .compositions
            .iter_mut()
            .filter(|comp| targets.contains(&comp.origin().sig.id))
        {
            composition
                .render_mut()?
                .draw(context, relative, targets, options, state)?;
        }
        for connection in self.entity.connections.iter_mut().filter(|conn| {
            state.is_port_selected(&conn.origin().joint_in.port)
                || state.is_port_selected(&conn.origin().joint_out.port)
        }) {
            connection.render_mut()?.draw(context, relative)?;
        }
        self.entity
            .ports
            .render_mut()?
            .draw(context, &self_relative, options, state)?;
        context.set_text_baseline("bottom");
        context.set_font(&format!("{}px serif", relative.zoom(12)));
        let _ = context.stroke_text(
            &self.origin().sig.id.to_string(),
            relative.x(self.view.container.get_coors().0) as f64,
            relative.y(self.view.container.get_coors().1 - 3) as f64,
        );
        // grid.draw(context, &Relative::new(0, 0, Some(relative.get_zoom())))?;
        Ok(())
    }

    pub fn draw_by_id(
        &mut self,
        grid: &Grid,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
        style: Option<Style>,
        id: usize,
        options: &Options,
        state: &State,
    ) -> Result<(), E> {
        if let Some(component) = self
            .entity
            .components
            .iter_mut()
            .find(|comp| comp.origin().sig.id == id)
        {
            component.render_mut()?.set_over_style(style);
            component
                .render_mut()?
                .draw(context, relative, options, state)?;
        }
        grid.draw(context, &Relative::new(0, 0, Some(relative.get_zoom())))?;
        Ok(())
    }

    pub fn find(&self, position: &(i32, i32), zoom: f64) -> Result<Vec<ElementCoors>, E> {
        if self.hidden {
            return Ok(vec![]);
        }
        let relative = Relative::new(0, 0, Some(zoom));
        let mut found: Vec<ElementCoors> = vec![];
        for el in self.view.elements.iter() {
            let (x, y) = el.form.get_coors_with_zoom(&relative);
            let (w, h) = el.form.get_box_size();
            let area = (x, y, (x + w), (y + h));
            if elements::is_point_in(position, &area) {
                found.push((el.id(), ElementType::Element, area));
            }
        }
        for nested in self.entity.compositions.iter() {
            found = [found, nested.render()?.find(position, zoom)?].concat();
        }
        Ok(found)
    }

    pub fn find_ports(
        &self,
        owners: &[ElementCoors],
        position: &(i32, i32),
        zoom: f64,
    ) -> Result<Vec<ElementCoors>, E> {
        if self.hidden {
            return Ok(vec![]);
        }
        let mut found: Vec<ElementCoors> = vec![];
        let components = &self.entity.components;
        let compositions = &self.entity.compositions;
        for (id, _, _) in owners.iter() {
            if let Ok(id) = id.parse::<usize>() {
                if let Some(entry) = find(components, compositions, &id) {
                    let mut relative = entry.own_relative()?;
                    relative.set_zoom(zoom);
                    found = [
                        found,
                        entry.ports().render()?.find(
                            &(position.0 - relative.x(0), position.1 - relative.y(0)),
                            &relative,
                        )?,
                    ]
                    .concat();
                }
            }
        }
        Ok(found)
    }

    pub fn get_coors_by_ids(
        &self,
        ids: &[usize],
        relative: &Relative,
    ) -> Result<Vec<ElementCoors>, E> {
        if self.hidden {
            return Ok(vec![]);
        }
        fn scan(
            found: &mut Vec<ElementCoors>,
            ports: &Representation<Ports>,
            ids: &[usize],
            own_relative: Relative,
            relative: &Relative,
        ) -> Result<(), E> {
            ports
                .render()?
                .entity
                .ports
                .iter()
                .filter(|p| ids.contains(&p.origin().sig.id))
                .for_each(|p| {
                    if let Ok(render) = p.render() {
                        let view = &render.view.container;
                        let (x, y) = view.get_coors_with_zoom(relative);
                        let (w, h) = view.get_box_size();
                        found.push((
                            p.origin().sig.id.to_string(),
                            ElementType::Port,
                            (
                                relative.x(own_relative.x(0)) + x,
                                relative.y(own_relative.y(0)) + y,
                                relative.x(own_relative.x(0)) + x + w,
                                relative.y(own_relative.y(0)) + y + h,
                            ),
                        ));
                    }
                });
            Ok(())
        }
        let mut found: Vec<ElementCoors> = vec![];
        for component in self.entity.components.iter() {
            scan(
                &mut found,
                &component.origin().ports,
                ids,
                component.render()?.own_relative(),
                relative,
            )?;
        }
        for composition in self.entity.compositions.iter() {
            scan(
                &mut found,
                &composition.origin().ports,
                ids,
                composition.render()?.own_relative(),
                relative,
            )?;
        }
        Ok(found)
    }

    pub fn get_groupped_ports(&self) -> Result<Vec<(usize, Vec<usize>)>, E> {
        let mut ports: Vec<(usize, Vec<usize>)> = vec![];
        for component in self.entity.components.iter() {
            ports = [ports, component.origin().ports.origin().get_groupped()].concat();
        }
        for composition in self.entity.compositions.iter() {
            ports = [ports, composition.origin().ports.origin().get_groupped()].concat();
        }
        Ok(ports)
    }

    pub fn get_connection_info(
        &self,
        port: usize,
    ) -> Option<((usize, Vec<usize>, usize), (usize, Vec<usize>, usize))> {
        let components = &self.entity.components;
        let compositions = &self.entity.compositions;
        self.entity
            .connections
            .iter()
            .find(|c| c.origin().joint_in.port == port || c.origin().joint_out.port == port)
            .map(|c| {
                let origin = c.origin();
                let port_out = find_port(
                    &components,
                    &compositions,
                    &origin.joint_out.component,
                    &origin.joint_out.port,
                );
                let port_in = find_port(
                    &components,
                    &compositions,
                    &origin.joint_in.component,
                    &origin.joint_in.port,
                );
                if let (Some(port_out), Some(port_in)) = (port_out, port_in) {
                    (
                        (
                            origin.joint_out.port,
                            port_out.contains.clone(),
                            origin.joint_out.component,
                        ),
                        (
                            origin.joint_in.port,
                            port_in.contains.clone(),
                            origin.joint_in.component,
                        ),
                    )
                } else {
                    (
                        (origin.joint_out.port, vec![], origin.joint_out.component),
                        (origin.joint_in.port, vec![], origin.joint_in.component),
                    )
                }
            })
    }
}

fn get_forms_by_ids<'a>(
    components: &'a [Representation<Component>],
    ids: &[usize],
) -> Result<Vec<&'a Form>, E> {
    let mut found: Vec<&Form> = vec![];
    for comp in components.iter() {
        if ids.contains(&comp.origin().sig.id) {
            found.push(&comp.render()?.view.container.form);
        }
    }
    Ok(found)
}

type GrouppedPorts = (
    Vec<Representation<Connection>>,
    Vec<(usize, Representation<Port>)>,
);
pub fn group_ports(
    connections: &[Representation<Connection>],
    sig_producer: &mut SignatureProducer,
) -> GrouppedPorts {
    let mut added_connections: Vec<Representation<Connection>> = vec![];
    let mut added_ports: Vec<(usize, Representation<Port>)> = vec![];
    let mut groupped: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    connections.iter().for_each(|conn| {
        let uuid = (
            conn.origin().joint_in.component,
            conn.origin().joint_out.component,
        );
        groupped
            .entry(uuid)
            .and_modify(|ports| {
                ports.push((conn.origin().joint_in.port, conn.origin().joint_out.port))
            })
            .or_insert(vec![(
                conn.origin().joint_in.port,
                conn.origin().joint_out.port,
            )]);
    });
    groupped
        .iter()
        .for_each(|((comp_joint_in, comp_joint_out), ports)| {
            let ports_in = ports.iter().map(|(l, _)| *l).collect::<Vec<usize>>();
            let ports_out = ports.iter().map(|(_, r)| *r).collect::<Vec<usize>>();
            if ports_in.is_empty()
                || ports_out.is_empty()
                || (ports_in.len() == 1 && ports_out.len() == 1)
            {
                return;
            }
            let joined_port_in = Port {
                sig: sig_producer.next_for("joined port IN"),
                port_type: PortType::In,
                contains: ports_in,
                visibility: true,
            };
            let joined_port_out = Port {
                sig: sig_producer.next_for("joined port OUT"),
                port_type: PortType::Out,
                contains: ports_out,
                visibility: true,
            };
            added_connections.push(Representation::Origin(Connection {
                joint_in: Joint {
                    component: *comp_joint_in,
                    port: joined_port_in.sig.id,
                },
                joint_out: Joint {
                    component: *comp_joint_out,
                    port: joined_port_out.sig.id,
                },
                sig: sig_producer.next_for("joined connection"),
            }));
            added_ports.push((*comp_joint_in, Representation::Origin(joined_port_in)));
            added_ports.push((*comp_joint_out, Representation::Origin(joined_port_out)));
        });
    (added_connections, added_ports)
}
