use wasm_bindgen_test::console_log;

use crate::{
    entity::{Component, Composition, Connection, Ports},
    error::E,
    render::{
        elements,
        entity::port,
        form::{button, Button, Path, Point, Rectangle},
        grid::Layout,
        Container, Form, Grid, Relative, Render, Representation, Style, View,
    },
};

impl Render<Composition> {
    pub fn new(mut entity: Composition) -> Self {
        entity.components = entity
            .components
            .drain(..)
            .map(|r| {
                if let Representation::Origin(component) = r {
                    Representation::Render(Render::<Component>::new(component))
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
                    Representation::Render(Render::<Composition>::new(composition))
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
            Representation::Render(Render::<Ports>::new(ports))
        } else {
            entity.ports
        };
        let id = entity.sig.id;
        let parent = entity.parent;
        Self {
            entity,
            view: View {
                container: Container {
                    form: Form::Rectangle(Rectangle {
                        id: id.to_string(),
                        x: 0,
                        y: 0,
                        w: 100,
                        h: 100,
                    }),
                    style: Style {
                        stroke_style: String::from("rgb(0,0,0)"),
                        fill_style: String::from("rgb(200,200,230)"),
                    },
                    hover: None,
                },
                elements: if let Some(id) = parent {
                    vec![Container {
                        form: Form::Button(Button {
                            id: format!("back::{id}"),
                            x: 0,
                            y: 0,
                            w: 0,
                            h: 0,
                            label: id.to_string(),
                            align: button::Align::Right,
                            padding: 3,
                        }),
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
        // Setup connections
        for conn in self.entity.connections.iter_mut() {
            if let (Some(ins), Some(outs)) = (
                self.entity
                    .components
                    .iter()
                    .find(|comp| comp.origin().sig.id == conn.origin().joint_in.component),
                self.entity
                    .components
                    .iter()
                    .find(|comp| comp.origin().sig.id == conn.origin().joint_out.component),
            ) {
                if let (Some(port_in), Some(port_out)) = (
                    ins.origin()
                        .ports
                        .origin()
                        .find(conn.origin().joint_in.port),
                    outs.origin()
                        .ports
                        .origin()
                        .find(conn.origin().joint_out.port),
                ) {
                    let port_in = port_in.render()?.view.container.get_coors();
                    let port_out = port_out.render()?.view.container.get_coors();
                    let relative_inns = ins.render()?.own_relative();
                    let relative_outs = outs.render()?.own_relative();
                    let offset = port::PORT_SIDE / 2;
                    let path = Path::new(
                        conn.origin().sig.id.to_string(),
                        vec![
                            Point {
                                x: relative_inns.x(port_in.0) + offset,
                                y: relative_inns.y(port_in.1) + offset,
                            },
                            Point {
                                x: relative_outs.x(port_out.0) + offset,
                                y: relative_outs.y(port_out.1) + offset,
                            },
                        ],
                    );
                    conn.render_mut()?.view.container.set_form(Form::Path(path));
                } else {
                    console_log!("No ports has been found :/");
                }
            }
        }
        Ok(())
    }

    pub fn calc(&mut self, grid: &mut Grid, expanded: &[usize]) -> Result<(), E> {
        // Create composition grid
        let mut composition_grid = Grid::new(1);
        // Order components by connections number
        self.entity.order();
        for composition in self.entity.compositions.iter_mut() {
            if expanded.contains(&composition.origin().sig.id) {
                composition
                    .render_mut()?
                    .calc(&mut composition_grid, expanded)?;
                composition.render_mut()?.show();
            } else {
                self.entity
                    .components
                    .push(Representation::Render(Render::<Component>::new(
                        composition.origin().to_component(),
                    )));
                composition.render_mut()?.hide();
            }
        }
        for component in self.entity.components.iter_mut() {
            component.render_mut()?.calc()?;
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
            let component_grid = Grid::from(Layout::Pair(a, b))?;
            composition_grid.insert(&component_grid);
        }
        for component in self.entity.components.iter() {
            if !located.contains(&component.origin().sig.id) {
                let component_grid = Grid::from(Layout::Pair(
                    get_forms_by_ids(&self.entity.components, &[component.origin().sig.id])?,
                    [].to_vec(),
                ))?;
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
        self.entity
            .ports
            .render_mut()?
            .calc(self.view.container.get_box_size().0)?;
        // Add composition as itself into grid
        composition_grid.insert_self(self.entity.sig.id);
        if let Some(container) = self.view.elements.first_mut() {
            container.set_coors(Some(grid_size.0 as i32), None);
        }
        // Add into global
        grid.insert(&composition_grid);
        Ok(())
    }

    pub fn draw(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
        targets: &Vec<usize>,
    ) -> Result<(), E> {
        if !targets.contains(&self.entity.sig.id) || self.hidden {
            return Ok(());
        }
        self.view.render(context, relative);
        let self_relative = self.relative(relative);
        self.entity
            .ports
            .render_mut()?
            .draw(context, &self_relative)?;
        for component in self
            .entity
            .components
            .iter_mut()
            .filter(|comp| targets.contains(&comp.origin().sig.id))
        {
            component.render_mut()?.draw(context, relative)?;
        }
        for composition in self
            .entity
            .compositions
            .iter_mut()
            .filter(|comp| targets.contains(&comp.origin().sig.id))
        {
            composition.render_mut()?.draw(context, relative, targets)?;
        }
        for connection in self.entity.connections.iter_mut().filter(|conn| {
            targets.contains(&conn.origin().joint_in.component)
                || targets.contains(&conn.origin().joint_out.component)
        }) {
            connection.render_mut()?.draw(context, relative)?;
        }
        let _ = context.stroke_text(
            &self.origin().sig.id.to_string(),
            relative.x(self.view.container.get_coors().0) as f64,
            relative.y(self.view.container.get_coors().1 - 4) as f64,
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
    ) -> Result<(), E> {
        if let Some(component) = self
            .entity
            .components
            .iter_mut()
            .find(|comp| comp.origin().sig.id == id)
        {
            component.render_mut()?.set_over_style(style);
            component.render_mut()?.draw(context, relative)?;
        }
        grid.draw(context, &Relative::new(0, 0, Some(relative.get_zoom())))?;
        Ok(())
    }

    pub fn find(&self, position: &(i32, i32), relative: &Relative) -> Result<Vec<String>, E> {
        if self.hidden {
            return Ok(vec![]);
        }
        let mut found: Vec<String> = vec![];
        let rel_position = (relative.x(position.0), relative.y(position.1));
        for el in self.view.elements.iter() {
            let (x, y) = el.form.get_coors();
            let (w, h) = el.form.get_box_size();
            let area = (
                relative.x(x) as u32,
                relative.y(y) as u32,
                (relative.x(x) + w) as u32,
                (relative.y(y) + h) as u32,
            );
            // console_log!(
            //     "POS: {position:?}; REL_POS: {rel_position:?};COORS ({}): {x}, {y}; AREA: {area:?} ({}, {})",
            //     el.form.id(),
            //     relative.x(x),
            //     relative.y(y)
            // );
            if elements::is_point_in(&(rel_position.0 as u32, rel_position.1 as u32), &area) {
                found.push(el.id());
            }
        }
        for nested in self.entity.compositions.iter() {
            found = [found, nested.render()?.find(&rel_position, relative)?].concat();
        }
        Ok(found)
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
