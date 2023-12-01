use wasm_bindgen_test::console_log;

use crate::{
    entity::{Component, Composition, Connection, Ports},
    error::E,
    render::{
        entity::port,
        form::{Path, Point, Rectangle},
        grid::Layout,
        Form, Grid, Relative, Render, Representation, Style,
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
        Self {
            entity,
            form: Form::Rectangle(Rectangle {
                id,
                x: 0,
                y: 0,
                w: 100,
                h: 100,
            }),
            style: Style {
                stroke_style: String::from("rgb(0,0,0)"),
                fill_style: String::from("rgb(200,200,230)"),
            },
            over_style: None,
        }
    }

    pub fn align_to_grid(&mut self, grid: &Grid) -> Result<(), E> {
        for comp in self.entity.components.iter_mut() {
            let relative = grid.relative(comp.origin().sig.id);
            comp.render_mut()?
                .form
                .set_coors(Some(relative.x(0)), Some(relative.y(0)));
        }
        // Align to grid nested compositions
        for composition in self.entity.compositions.iter_mut() {
            composition.render_mut()?.align_to_grid(grid)?;
        }
        let relative = grid.relative(self.entity.sig.id);
        self.form
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
                    let port_in = port_in.render()?.form.get_coors();
                    let port_out = port_out.render()?.form.get_coors();
                    let relative_inns = ins.render()?.own_relative();
                    let relative_outs = outs.render()?.own_relative();
                    let offset = port::PORT_SIDE / 2;
                    let path = Path::new(
                        conn.origin().sig.id,
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
                    conn.render_mut()?.form = Form::Path(path);
                } else {
                    console_log!("No ports has been found :/");
                }
            }
        }
        Ok(())
    }

    pub fn calc(&mut self, grid: &mut Grid) -> Result<(), E> {
        // Create composition grid
        let mut composition_grid = Grid::new(1);
        // Order components by connections number
        self.entity.order();
        for component in self.entity.components.iter_mut() {
            component.render_mut()?.calc()?;
        }
        for composition in self.entity.compositions.iter_mut() {
            composition.render_mut()?.calc(&mut composition_grid)?;
        }
        // Get dependencies data (list of components with IN / OUT connections)
        let mut dependencies: Vec<(usize, Vec<usize>, Vec<usize>)> = vec![];
        let mut located: Vec<usize> = vec![];
        for component in self.entity.components.iter() {
            let host_id = component.origin().sig.id;
            if located.contains(&host_id) {
                continue;
            }
            let (linked_in, linked_out) =
                Connection::linked(&self.entity.connections, host_id, &located);
            dependencies.push((host_id, linked_out.to_vec(), linked_in.to_vec()));
            located = [located, linked_in, linked_out, vec![host_id]].concat();
        }
        // Get components grids
        for (host_id, linked_in, linked_out) in dependencies {
            let on_right = get_forms_by_ids(&self.entity.components, &linked_in)?;
            let on_left = get_forms_by_ids(&self.entity.components, &linked_out)?;
            let on_center = get_forms_by_ids(&self.entity.components, &[host_id])?;
            let component_grid =
                Grid::from(Layout::WithFormsBySides((on_left, on_center, on_right)))?;
            composition_grid.insert(&component_grid);
        }
        // Align to composition grid
        self.align_to_grid(&composition_grid)?;
        let grid_size = composition_grid.get_size_px();
        let grid_height_px =
            composition_grid.set_min_height(self.entity.ports.render()?.height() as u32);
        self.form
            .set_box_size(Some(grid_size.0 as i32), Some(grid_height_px as i32));
        // Calc ports
        self.entity
            .ports
            .render_mut()?
            .calc(self.form.get_box_size().0)?;
        // Add composition as itself into grid
        composition_grid.insert_self(self.entity.sig.id);
        // Add into global
        grid.insert(&composition_grid);
        Ok(())
    }

    pub fn draw(
        &self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
        targets: &Vec<usize>,
    ) -> Result<(), E> {
        if !targets.contains(&self.entity.sig.id) {
            Ok(())?;
        }
        self.style.apply(context);
        self.form.render(context, relative);
        let self_relative = self.relative(relative);
        self.entity.ports.render()?.draw(context, &self_relative)?;
        for component in self
            .entity
            .components
            .iter()
            .filter(|comp| targets.contains(&comp.origin().sig.id))
        {
            component.render()?.draw(context, relative)?;
        }
        for composition in self
            .entity
            .compositions
            .iter()
            .filter(|comp| targets.contains(&comp.origin().sig.id))
        {
            composition.render()?.draw(context, relative, targets)?;
        }
        for connection in self.entity.connections.iter().filter(|conn| {
            targets.contains(&conn.origin().joint_in.component)
                || targets.contains(&conn.origin().joint_out.component)
        }) {
            connection.render()?.draw(context, relative)?;
        }
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
            component.render()?.draw(context, relative)?;
        }
        grid.draw(context, &Relative::new(0, 0, Some(relative.get_zoom())))?;
        Ok(())
    }
}

fn get_forms_by_ids<'a>(
    components: &'a [Representation<Component>],
    ids: &[usize],
) -> Result<Vec<&'a Form>, E> {
    let mut found: Vec<&Form> = vec![];
    for comp in components.iter() {
        if ids.contains(&comp.origin().sig.id) {
            found.push(&comp.render()?.form);
        }
    }
    Ok(found)
}
