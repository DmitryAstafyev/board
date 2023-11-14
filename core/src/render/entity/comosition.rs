use wasm_bindgen_test::console_log;

use crate::{
    entity::{Component, Composition, Connection, Port, PortType},
    error::E,
    render::{form::Rectangle, grid::Layout, Form, Grid, Relative, Render, Representation, Style},
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
        let id = entity.sig.id;
        Self {
            entity,
            form: Form::Rectangle(Rectangle {
                id,
                x: 200,
                y: 20,
                w: 100,
                h: 100,
            }),
            style: Style {
                stroke_style: String::from("rgb(0,0,0)"),
                fill_style: String::from("rgb(230,230,230)"),
            },
            grid: Some(Grid::new()),
        }
    }

    pub fn calc(&mut self) -> Result<(), E> {
        // Order components by connections number
        self.entity.order();
        for component in self.entity.components.iter_mut() {
            component.render_mut()?.calc()?;
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
            dependencies.push((host_id, linked_in.to_vec(), linked_out.to_vec()));
            located = [located, linked_in, linked_out, vec![host_id]].concat();
        }
        let mut grids: Vec<Grid> = vec![];
        for (host_id, linked_in, linked_out) in dependencies {
            console_log!("host: {host_id}; in: {linked_in:?}; out: {linked_out:?}");
            let on_right = get_forms_by_ids(&self.entity.components, &linked_in)?;
            let on_left = get_forms_by_ids(&self.entity.components, &linked_out)?;
            let on_center = get_forms_by_ids(&self.entity.components, &[host_id])?;
            let grid = Grid::from(Layout::WithFormsBySides((on_left, on_center, on_right)))?;
            console_log!("{grid:?}");
            grids.push(grid);
        }
        // Create common grid
        let grid = Grid::from(Layout::GridsRow(&grids))?;
        self.grid = Some(grid);
        Ok(())
    }

    pub fn draw(
        &self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
    ) -> Result<(), E> {
        self.style.apply(context);
        self.form.render(context, relative);
        for component in self.entity.components.iter() {
            let component_relative: Option<Relative> = self
                .grid
                .as_ref()
                .map(|grid| relative.from_base(&grid.relative(component.origin().sig.id)));
            component.render()?.draw(
                context,
                if let Some(relative) = component_relative.as_ref() {
                    console_log!("comp relative is used: {relative:?}");
                    relative
                } else {
                    relative
                },
            )?;
        }
        if let Some(grid) = self.grid.as_ref() {
            console_log!(">>>> RELATIVE: {relative:?}");
            grid.draw(context, relative)?;
        }
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
