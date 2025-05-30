use crate::{
    entity::{
        dummy::SignatureProducer, Component, Composition, Connection, IsComponentIncluded,
        IsPortIncluded, Joint, Port, PortType, Ports, Signature, SignatureGetter,
    },
    error::E,
    render::{
        elements,
        form::{Path, Point, Rectangle},
        grid::{ElementCoors, ElementType},
        options::Options,
        Container, Form, Grid, Ratio, Relative, Render, Representation, Style, View,
    },
    state::State,
};
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::console_log;

/// (port,  contains,   comp )
/// (usize, Vec<usize>, usize)
pub type ConnectionData = (usize, Vec<usize>, usize);

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
    pub fn sig(&self) -> &Signature {
        match self {
            Entry::Component(c) => c.sig(),
            Entry::Composition(c) => c.sig(),
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
        .find(|c| c.sig().id == *id)
        .map(Entry::Component)
        .or_else(|| {
            compositions
                .iter()
                .find(|c| c.sig().id == *id)
                .map(Entry::Composition)
        })
}

impl<'a, 'b: 'a> SignatureGetter<'a, 'b> for Render<Composition> {
    fn sig(&'b self) -> &'a Signature {
        &self.origin().sig
    }
}

impl Render<Composition> {
    pub fn new(
        mut entity: Composition,
        root: bool,
        options: &Options,
        sig_producer: &mut SignatureProducer,
    ) -> Self {
        if options.ports.grouping && root {
            group_ports(&mut entity, sig_producer);
        }
        if options.ports.group_unbound {
            group_unbound_ports(&mut entity, sig_producer);
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
                    Representation::Render(Render::<Composition>::new(
                        composition,
                        false,
                        options,
                        sig_producer,
                    ))
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
                    Representation::Render(Render::<Connection>::new(connection, options))
                } else {
                    r
                }
            })
            .collect::<Vec<Representation<Connection>>>();
        entity.ports = if let Representation::Origin(ports) = entity.ports {
            Representation::Render(Render::<Ports>::new(ports, options, false))
        } else {
            entity.ports
        };
        entity.order();
        let id = entity.sig.id;
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
                    style: (&options.scheme.composition_rect).into(),
                },
                elements: Vec::new(),
            },
            hidden: false,
        }
    }

    /// # Returns
    /// (
    ///     Vec<usize> - ids of filtered entities
    ///     Vec<usize> - ids of linked entities (for example if port A selected and has connection to port B, port B will be linked)
    ///     Vec<usize> - owner of filtered entities (for example if port selected, it's parent will be here)
    /// )
    pub fn get_filtered_ports(
        &mut self,
        filter: Option<String>,
    ) -> Option<(Vec<usize>, Vec<usize>, Vec<usize>)> {
        self.entity
            .components
            .retain(|c| c.render().map_or(true, |r| !r.is_composition()));
        self.entity
            .compositions
            .iter_mut()
            .for_each(|c| c.render_mut().unwrap().show());
        filter.as_ref().map(|filter| {
            let filtered = [
                self.entity
                    .components
                    .iter()
                    .flat_map(|c| c.origin().ports.origin().get_filtered_ports(filter))
                    .collect::<Vec<usize>>(),
                self.entity
                    .compositions
                    .iter()
                    .flat_map(|c| c.origin().ports.origin().get_filtered_ports(filter))
                    .collect::<Vec<usize>>(),
                self.entity.ports.origin().get_filtered_ports(filter),
            ]
            .concat();
            let linked = self
                .entity
                .connections
                .iter()
                .filter_map(|c| {
                    let connection = c.origin();
                    if filtered.contains(connection.in_port()) {
                        Some(*connection.out_port())
                    } else if filtered.contains(connection.out_port()) {
                        Some(*connection.in_port())
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();
            let owners = [
                self.entity
                    .compositions
                    .iter()
                    .filter_map(|c| {
                        let ports = &c.origin().ports.origin().ports;
                        if ports.is_empty() {
                            None
                        } else if ports.iter().any(|p| filtered.contains(&p.sig().id))
                            || ports.iter().any(|p| linked.contains(&p.sig().id))
                        {
                            Some(c.sig().id)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<usize>>(),
                self.entity
                    .components
                    .iter()
                    .filter_map(|c| {
                        let ports = &c.origin().ports.origin().ports;
                        if ports.is_empty() {
                            None
                        } else if ports.iter().any(|p| filtered.contains(&p.sig().id))
                            || ports.iter().any(|p| linked.contains(&p.sig().id))
                        {
                            Some(c.sig().id)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<usize>>(),
            ]
            .concat();
            (filtered, linked, owners)
        })
    }

    pub fn get_targeted_components(
        &mut self,
        filter: Option<String>,
    ) -> Option<(Vec<usize>, Vec<usize>)> {
        self.entity
            .components
            .retain(|c| c.render().map_or(true, |r| !r.is_composition()));
        self.entity
            .compositions
            .iter_mut()
            .for_each(|c| c.render_mut().unwrap().show());
        filter.as_ref().map(|filter| {
            let targeted = [
                self.entity
                    .components
                    .iter()
                    .filter_map(|c| {
                        if c.sig()
                            .short_name
                            .to_lowercase()
                            .contains(&filter.to_lowercase())
                        {
                            Some(c.sig().id)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<usize>>(),
                self.entity
                    .compositions
                    .iter()
                    .filter_map(|c| {
                        if c.sig()
                            .short_name
                            .to_lowercase()
                            .contains(&filter.to_lowercase())
                        {
                            Some(c.sig().id)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<usize>>(),
            ]
            .concat();
            let linked = self
                .entity
                .connections
                .iter()
                .filter_map(|c| {
                    let connection = c.origin();
                    if targeted.contains(connection.in_comp()) {
                        Some(*connection.out_comp())
                    } else if targeted.contains(connection.out_comp()) {
                        Some(*connection.in_comp())
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();
            (targeted, linked)
        })
    }

    pub fn get_targeted_components_by_ids(
        &mut self,
        targeted: Vec<usize>,
    ) -> Option<(Vec<usize>, Vec<usize>)> {
        self.entity
            .components
            .retain(|c| c.render().map_or(true, |r| !r.is_composition()));
        self.entity
            .compositions
            .iter_mut()
            .for_each(|c| c.render_mut().unwrap().show());
        let linked = self
            .entity
            .connections
            .iter()
            .filter_map(|c| {
                let connection = c.origin();
                if targeted.contains(connection.in_comp()) {
                    Some(*connection.out_comp())
                } else if targeted.contains(connection.out_comp()) {
                    Some(*connection.in_comp())
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();
        if targeted.is_empty() && linked.is_empty() {
            None
        } else {
            Some((targeted, linked))
        }
    }

    pub fn get_components_linked_to(&self, targeted: Vec<usize>) -> Vec<usize> {
        self.entity
            .connections
            .iter()
            .filter_map(|c| {
                let connection = c.origin();
                if targeted.contains(connection.in_comp()) {
                    Some(*connection.out_comp())
                } else if targeted.contains(connection.out_comp()) {
                    Some(*connection.in_comp())
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>()
    }

    // TODO: we have to remove dupblicates, but it's a wierd fact we have it at all. It should be checked: why
    // we have duplicates of ports
    pub fn get_matches(&mut self, filter: Option<String>) -> Option<Vec<usize>> {
        filter
            .as_ref()
            .map(|filter| {
                [
                    self.entity
                        .components
                        .iter()
                        .flat_map(|c| c.origin().ports.origin().get_filtered_ports(filter))
                        .collect::<Vec<usize>>(),
                    self.entity
                        .compositions
                        .iter()
                        .flat_map(|c| c.origin().ports.origin().get_filtered_ports(filter))
                        .collect::<Vec<usize>>(),
                    self.entity.ports.origin().get_filtered_ports(filter),
                    self.entity
                        .components
                        .iter()
                        .filter(|c| {
                            c.origin()
                                .sig
                                .short_name
                                .to_lowercase()
                                .contains(&filter.to_lowercase())
                        })
                        .map(|c| c.origin().sig().id)
                        .collect::<Vec<usize>>(),
                ]
                .concat()
            })
            .map(|dirty| {
                // We should not sort originaly collected dirty ids, because it's sorted by component/composition
                let mut ids: Vec<usize> = Vec::new();
                dirty.into_iter().for_each(|id| {
                    if !ids.contains(&id) {
                        ids.push(id);
                    }
                });
                ids
            })
    }

    // Vec<(port_id, holder_port_id, component_id)>
    pub fn get_matches_extended(
        &mut self,
        filter: Option<String>,
    ) -> Option<Vec<(usize, Option<usize>, usize)>> {
        let parent_id = self.sig().id;
        filter
            .as_ref()
            .map(|filter| {
                [
                    self.entity
                        .components
                        .iter()
                        .flat_map(|c| {
                            let parent_id = c.sig().id;
                            c.origin()
                                .ports
                                .origin()
                                .get_filtered_ports_expanded(filter)
                                .into_iter()
                                .map(|(port_id, holder_id)| (port_id, holder_id, parent_id))
                                .collect::<Vec<(usize, Option<usize>, usize)>>()
                        })
                        .collect::<Vec<(usize, Option<usize>, usize)>>(),
                    self.entity
                        .compositions
                        .iter()
                        .flat_map(|c| {
                            let parent_id = c.sig().id;
                            c.origin()
                                .ports
                                .origin()
                                .get_filtered_ports_expanded(filter)
                                .into_iter()
                                .map(|(port_id, holder_id)| (port_id, holder_id, parent_id))
                                .collect::<Vec<(usize, Option<usize>, usize)>>()
                        })
                        .collect::<Vec<(usize, Option<usize>, usize)>>(),
                    self.entity
                        .ports
                        .origin()
                        .get_filtered_ports_expanded(filter)
                        .into_iter()
                        .map(|(port_id, holder_id)| (port_id, holder_id, parent_id))
                        .collect::<Vec<(usize, Option<usize>, usize)>>(),
                    self.entity
                        .components
                        .iter()
                        .filter(|c| {
                            c.origin()
                                .sig
                                .short_name
                                .to_lowercase()
                                .contains(&filter.to_lowercase())
                        })
                        .map(|c| (c.origin().sig().id, None, parent_id))
                        .collect::<Vec<(usize, Option<usize>, usize)>>(),
                ]
                .concat()
            })
            .map(|dirty| {
                // We should not sort originaly collected dirty ids, because it's sorted by component/composition
                let mut ids: Vec<(usize, Option<usize>, usize)> = Vec::new();
                let mut collected: Vec<usize> = Vec::new();
                dirty.into_iter().for_each(|(id, holder, owner)| {
                    if !collected.contains(&id) {
                        collected.push(id);
                        ids.push((id, holder, owner));
                    }
                });
                ids
            })
    }

    pub fn align_to_grid(&mut self, grid: &Grid) -> Result<(), E> {
        for comp in self.entity.components.iter_mut() {
            let relative = grid.relative(comp.sig().id);
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

    pub fn setup_connections(
        &mut self,
        _grid: &Grid,
        options: &Options,
        state: &State,
    ) -> Result<(), E> {
        let components = &self.entity.components;
        let compositions = &self.entity.compositions;
        let mut failed: Vec<&Connection> = Vec::new();
        let self_ports = &self.entity.ports;
        let own_relative = self.own_relative();
        for conn in self.entity.connections.iter_mut().filter(|conn| {
            let origin = conn.origin();
            origin.visibility
                && state.is_comp_included(origin.in_comp())
                && state.is_comp_included(origin.out_comp())
        }) {
            let (Some((port_in, in_rel)), Some((port_out, out_rel))) = (
                find(components, compositions, conn.origin().in_comp())
                    .and_then(|parent| {
                        parent
                            .ports()
                            .origin()
                            .find(conn.origin().in_port())
                            .map(|p| (p, parent.own_relative()))
                    })
                    .or_else(|| {
                        self_ports
                            .origin()
                            .find(conn.origin().in_port())
                            .map(|p| (p, Ok(own_relative.clone())))
                    }),
                find(components, compositions, conn.origin().out_comp())
                    .and_then(|parent| {
                        parent
                            .ports()
                            .origin()
                            .find(conn.origin().out_port())
                            .map(|p| (p, parent.own_relative()))
                    })
                    .or_else(|| {
                        self_ports
                            .origin()
                            .find(conn.origin().out_port())
                            .map(|p| (p, Ok(own_relative.clone())))
                    }),
            ) else {
                failed.push(conn.origin());
                continue;
            };
            let coors_port_in = port_in.render()?.view.container.get_coors();
            let coors_port_out = port_out.render()?.view.container.get_coors();
            let relative_inns = in_rel?;
            let relative_outs = out_rel?;
            let size_port_in = port_in.render()?.view.container.get_box_size();
            let size_port_out = port_out.render()?.view.container.get_box_size();
            let is_self_connection = self_ports
                .origin()
                .find(&port_in.sig().id)
                .or_else(|| self_ports.origin().find(&port_out.sig().id))
                .is_some();
            let points: Vec<Point> = if is_self_connection {
                if matches!(port_in.origin().port_type, PortType::Right) {
                    vec![
                        Point {
                            x: relative_outs.x(coors_port_out.0) + size_port_out.0,
                            y: relative_outs.y(coors_port_out.1) + size_port_out.1 / 2,
                        },
                        Point {
                            x: relative_inns.x(coors_port_in.0),
                            y: relative_inns.y(coors_port_in.1) + size_port_in.1 / 2,
                        },
                    ]
                } else {
                    vec![
                        Point {
                            x: relative_inns.x(coors_port_in.0) + size_port_in.0,
                            y: relative_inns.y(coors_port_in.1) + size_port_in.1 / 2,
                        },
                        Point {
                            x: relative_outs.x(coors_port_out.0),
                            y: relative_outs.y(coors_port_out.1) + size_port_out.1 / 2,
                        },
                    ]
                }
            } else if matches!(port_in.origin().port_type, PortType::Right) {
                vec![
                    Point {
                        x: relative_inns.x(coors_port_in.0) + size_port_in.0,
                        y: relative_inns.y(coors_port_in.1) + size_port_in.1 / 2,
                    },
                    Point {
                        x: relative_outs.x(coors_port_out.0),
                        y: relative_outs.y(coors_port_out.1) + size_port_out.1 / 2,
                    },
                ]
            } else {
                vec![
                    Point {
                        x: relative_outs.x(coors_port_out.0) + size_port_out.0,
                        y: relative_outs.y(coors_port_out.1) + size_port_out.1 / 2,
                    },
                    Point {
                        x: relative_inns.x(coors_port_in.0),
                        y: relative_inns.y(coors_port_in.1) + size_port_in.1 / 2,
                    },
                ]
            };
            let path = Path::new(conn.sig().id.to_string(), points, &options.ratio());
            conn.render_mut()?
                .view
                .container
                .set_form(Form::Path(ElementType::Connection, path));
        }
        if !failed.is_empty() {
            console_log!("Fail to find ports for {} connections", failed.len());
        }
        Ok(())
    }

    pub fn calc(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        grid: &mut Grid,
        state: &State,
        options: &Options,
    ) -> Result<(), E> {
        let relative = &state.get_view_relative();
        // Create composition grid
        let mut composition_grid = Grid::new(&options.grid, options.ratio());
        for composition in self.entity.compositions.iter_mut() {
            if !state.is_comp_included(&composition.sig().id) {
                continue;
            }
            self.entity
                .components
                .push(Representation::Render(Render::<Component>::new(
                    composition.origin().to_component(options),
                    options,
                    Some(ElementType::Composition),
                )));
            composition.render_mut()?.hide();
        }
        for component in self.entity.components.iter_mut() {
            if !state.is_comp_included(&component.sig().id) {
                continue;
            }
            component
                .render_mut()?
                .calc(context, relative, options, state, self.entity.sig.id)?;
        }
        // Get dependencies data (list of components with IN / OUT connections)
        let mut dependencies: Vec<(usize, usize)> = Vec::new();
        let mut located: Vec<usize> = Vec::new();
        let ordered_linked = Connection::get_ordered_linked(&self.entity.connections, state);
        for (host_id, _, _) in ordered_linked.iter() {
            if located.contains(host_id) {
                continue;
            }
            let linked = Connection::get_ordered_linked_to(
                &self.entity.connections,
                *host_id,
                &located,
                state,
            );
            if let Some((id, _, _)) = linked.first() {
                dependencies.push((*host_id, *id));
                located = [located, vec![*host_id, *id]].concat();
            }
        }
        // Get pairs grids
        for (a_id, b_id) in dependencies {
            let a = get_forms_by_ids(&self.entity.components, &[a_id])?;
            let b = get_forms_by_ids(&self.entity.components, &[b_id])?;
            let component_grid = Grid::forms_as_pair(a, b, &options.grid, options.ratio())?;
            composition_grid.insert(&component_grid);
        }
        for component in self
            .entity
            .components
            .iter()
            .filter(|c| state.is_comp_included(&c.sig().id))
        {
            if !located.contains(&component.sig().id) {
                let component_grid = Grid::forms_as_pair(
                    get_forms_by_ids(&self.entity.components, &[component.sig().id])?,
                    [].to_vec(),
                    &options.grid,
                    options.ratio(),
                )?;
                composition_grid.insert(&component_grid);
            }
        }
        let grid_size = composition_grid.get_size_px();
        // Caclulcate self ports
        self.entity.ports.render_mut()?.calc(
            context,
            grid_size.0 as i32,
            relative,
            options,
            state,
            self.entity.sig.id,
        )?;
        composition_grid
            .set_min_height(self.entity.ports.render_mut()?.height(state, options) as u32);
        // Align to composition grid
        self.align_to_grid(&composition_grid)?;
        let grid_height_px = composition_grid.set_min_height(50);
        self.view
            .container
            .set_box_size(Some(grid_size.0 as i32), Some(grid_height_px as i32));
        // Add composition as itself into grid
        composition_grid.insert_self(self.entity.sig.id, ElementType::Composition);
        if let Some(container) = self.view.elements.first_mut() {
            container.set_coors(Some(grid_size.0 as i32), None);
        }
        self.setup_connections(&composition_grid, options, state)?;
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
        self.view.render(context, relative, options);
        for component in self
            .entity
            .components
            .iter_mut()
            .filter(|comp| targets.contains(&comp.sig().id))
        {
            component
                .render_mut()?
                .draw(context, relative, options, state, self.entity.sig.id)?;
        }
        for composition in self
            .entity
            .compositions
            .iter_mut()
            .filter(|comp| targets.contains(&comp.sig().id))
        {
            composition
                .render_mut()?
                .draw(context, relative, targets, options, state)?;
        }
        let ratio = options.ratio();
        context.set_stroke_style(&JsValue::from_str(&options.scheme.composition_label.stroke));
        context.set_text_baseline("bottom");
        context.set_font(&format!(
            "{}px {}",
            ratio.get(relative.zoom(12)),
            options.font
        ));
        context.set_fill_style(&JsValue::from_str(&options.scheme.composition_label.fill));
        let _ = context.fill_text(
            &self.origin().get_label(options),
            relative.x(self.view.container.get_coors().0) as f64,
            relative.y(self.view.container.get_coors().1 - ratio.get(3)) as f64,
        );
        self.entity.ports.render_mut()?.draw(
            context,
            relative,
            options,
            state,
            self.entity.sig.id,
        )?;
        for connection in self.entity.connections.iter_mut().filter(|conn| {
            conn.origin().visibility
                && (state.is_port_selected_or_highlighted(conn.origin().in_port())
                    && state.is_port_selected_or_highlighted(conn.origin().out_port()))
        }) {
            connection
                .render_mut()?
                .draw(context, relative, options, state)?;
        }
        // grid.draw(context, &Relative::new(0, 0, Some(relative.get_zoom())))?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
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
            .find(|comp| comp.sig().id == id)
        {
            component.render_mut()?.set_over_style(style);
            component
                .render_mut()?
                .draw(context, relative, options, state, self.entity.sig.id)?;
        }
        grid.draw(context, &Relative::new(0, 0, Some(relative.get_zoom())))?;
        Ok(())
    }

    pub fn find(&self, position: &(i32, i32), zoom: f64) -> Result<Vec<ElementCoors>, E> {
        if self.hidden {
            return Ok(Vec::new());
        }
        let relative = Relative::new(0, 0, Some(zoom));
        let mut found: Vec<ElementCoors> = Vec::new();
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
        state: &State,
    ) -> Result<Vec<ElementCoors>, E> {
        if self.hidden {
            return Ok(Vec::new());
        }
        let mut found: Vec<ElementCoors> = Vec::new();
        for (id, _, _) in owners.iter() {
            if let Ok(id) = id.parse::<usize>() {
                if let Some(entry) = self.find_entity(&id) {
                    let mut relative = entry.own_relative()?;
                    relative.set_zoom(state.zoom);
                    found = [
                        found,
                        entry.ports().render()?.find(
                            &(position.0 - relative.x(0), position.1 - relative.y(0)),
                            &relative,
                            state,
                        )?,
                    ]
                    .concat();
                }
            }
        }
        // Add also matches with self ports
        let mut relative = self.own_relative();
        relative.set_zoom(state.zoom);
        found = [
            found,
            self.entity.ports.render()?.find(
                &(position.0 - relative.x(0), position.1 - relative.y(0)),
                &relative,
                state,
            )?,
        ]
        .concat();
        Ok(found)
    }

    pub fn get_coors_by_ids(
        &self,
        ids: &[usize],
        relative: &Relative,
        ratio: &Ratio,
    ) -> Result<Vec<ElementCoors>, E> {
        if self.hidden {
            return Ok(Vec::new());
        }
        let mut stored: Vec<usize> = Vec::new();
        fn scan(
            found: &mut Vec<ElementCoors>,
            stored: &mut Vec<usize>,
            ports: &Representation<Ports>,
            ids: &[usize],
            own_relative: Relative,
            relative: &Relative,
            ratio: &Ratio,
        ) -> Result<(), E> {
            ports
                .render()?
                .entity
                .ports
                .iter()
                .filter(|p| ids.contains(&p.sig().id))
                .for_each(|p| {
                    if let Ok(render) = p.render() {
                        let view = &render.view.container;
                        let (x, y) = view.get_coors_with_zoom(relative);
                        let (w, h) = view.get_box_size();
                        if stored.contains(&p.sig().id) {
                            return;
                        }
                        stored.push(p.sig().id);
                        found.push((
                            p.sig().id.to_string(),
                            ElementType::Port,
                            (
                                ratio.invert(relative.x(own_relative.x(0)) + x),
                                ratio.invert(relative.y(own_relative.y(0)) + y),
                                ratio.invert(relative.x(own_relative.x(0)) + x + w),
                                ratio.invert(relative.y(own_relative.y(0)) + y + h),
                            ),
                        ));
                    }
                });
            Ok(())
        }
        let mut found: Vec<ElementCoors> = Vec::new();
        for component in self.entity.components.iter() {
            scan(
                &mut found,
                &mut stored,
                &component.origin().ports,
                ids,
                component.render()?.own_relative(),
                relative,
                ratio,
            )?;
        }
        for composition in self.entity.compositions.iter() {
            scan(
                &mut found,
                &mut stored,
                &composition.origin().ports,
                ids,
                composition.render()?.own_relative(),
                relative,
                ratio,
            )?;
        }
        scan(
            &mut found,
            &mut stored,
            &self.origin().ports,
            ids,
            self.own_relative(),
            relative,
            ratio,
        )?;
        Ok(found)
    }

    pub fn get_grouped_ports(&self) -> Result<Vec<(usize, Vec<usize>)>, E> {
        let mut ports: Vec<(usize, Vec<usize>)> = Vec::new();
        for component in self.entity.components.iter() {
            ports = [ports, component.origin().ports.origin().get_grouped()].concat();
        }
        for composition in self.entity.compositions.iter() {
            ports = [ports, composition.origin().ports.origin().get_grouped()].concat();
        }
        Ok(ports)
    }

    pub fn get_port(&self, id: usize) -> Option<&Port> {
        if let Some(port) = self.origin().get_port(&id) {
            return Some(port);
        }
        for composition in self.entity.compositions.iter() {
            if let Some(port) = composition.origin().get_port(&id) {
                return Some(port);
            }
        }
        for component in self.entity.components.iter() {
            if let Some(port) = component.origin().get_port(&id) {
                return Some(port);
            }
        }
        None
    }

    fn connection_to_connection_data(&self, conn: &Connection) -> (ConnectionData, ConnectionData) {
        let port_out = self.find_port(conn.out_comp(), conn.out_port());
        let port_in = self.find_port(conn.in_comp(), conn.in_port());
        if let (Some(port_out), Some(port_in)) = (port_out, port_in) {
            (
                (
                    *conn.out_port(),
                    port_out.contains.clone(),
                    *conn.out_comp(),
                ),
                (*conn.in_port(), port_in.contains.clone(), *conn.in_comp()),
            )
        } else {
            (
                (*conn.out_port(), Vec::new(), *conn.out_comp()),
                (*conn.in_port(), Vec::new(), *conn.in_comp()),
            )
        }
    }
    /// Returns information about single connection
    pub fn get_connection(&self, port: usize) -> Option<(ConnectionData, ConnectionData)> {
        self.entity
            .connections
            .iter()
            .find(|c| (&port).included_as_port(*c))
            .map(|c| self.connection_to_connection_data(c.origin()))
    }

    /// Returns information about all connections related to port
    pub fn get_connections(&self, port: usize) -> Vec<(ConnectionData, ConnectionData)> {
        self.entity
            .connections
            .iter()
            .filter(|c| (&port).included_as_port(*c))
            .map(|c| self.connection_to_connection_data(c.origin()))
            .collect()
    }

    /// Returns information about all connections related to component
    pub fn get_connections_by_component(
        &self,
        component: usize,
    ) -> Vec<(ConnectionData, ConnectionData)> {
        self.entity
            .connections
            .iter()
            .filter(|c| (&component).included_as_component(*c))
            .map(|c| self.connection_to_connection_data(c.origin()))
            .collect()
    }

    /// Returns information about all connections of current composition
    pub fn get_all_connections(&self) -> Vec<(ConnectionData, ConnectionData)> {
        self.entity
            .connections
            .iter()
            .map(|c| self.connection_to_connection_data(c.origin()))
            .collect()
    }

    /// Returns list of all components (including owned compositions)
    pub fn get_all_components(&self) -> Vec<usize> {
        self.entity.components.iter().map(|c| c.sig().id).collect()
    }

    fn find_entity<'a>(&'a self, id: &usize) -> Option<Entry<'a>> {
        self.entity
            .components
            .iter()
            .find(|c| c.sig().id == *id)
            .map(Entry::Component)
            .or_else(|| {
                self.entity
                    .compositions
                    .iter()
                    .find(|c| c.sig().id == *id)
                    .map(Entry::Composition)
            })
    }

    fn find_port<'a>(&'a self, parent_id: &usize, port_id: &usize) -> Option<&'a Port> {
        if self.sig().id == *parent_id {
            self.get_port(*port_id)
        } else {
            self.find_entity(parent_id)
                .and_then(|entry| entry.ports().origin().find(port_id).map(|p| p.origin()))
        }
    }
}

fn get_forms_by_ids<'a>(
    components: &'a [Representation<Component>],
    ids: &[usize],
) -> Result<Vec<&'a Form>, E> {
    let mut found: Vec<&Form> = Vec::new();
    for comp in components.iter() {
        if ids.contains(&comp.sig().id) {
            found.push(&comp.render()?.view.container.form);
        }
    }
    Ok(found)
}

pub fn group_ports(entity: &mut Composition, sig_producer: &mut SignatureProducer) {
    let mut added_connections: Vec<Representation<Connection>> = Vec::new();
    let mut added_ports: Vec<(usize, Representation<Port>)> = Vec::new();
    let mut grouped: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    // Find ports connected to only 1 component
    let mut ports: HashMap<usize, usize> = HashMap::new();
    let connections = entity
        .connections
        .iter()
        .collect::<Vec<&Representation<Connection>>>();
    connections.iter().for_each(|connection| {
        ports
            .entry(*connection.origin().in_port())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        ports
            .entry(*connection.origin().out_port())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });
    // Exclude ports with more than 1 connection and self ports with outside connections
    let self_ports = entity.ports.origin();
    ports.retain(|port_id, count| {
        if *count == 1 {
            let Some(port) = self_ports.find(port_id) else {
                return true;
            };
            port.origin().connected.len() == 1
        } else {
            false
        }
    });

    // Take only related connections
    connections
        .iter()
        .filter(|conn| {
            ports.contains_key(conn.origin().in_port())
                && ports.contains_key(conn.origin().out_port())
        })
        .for_each(|conn| {
            let uuid = (*conn.origin().in_comp(), *conn.origin().out_comp());
            grouped
                .entry(uuid)
                .and_modify(|ports| {
                    ports.push((*conn.origin().in_port(), *conn.origin().out_port()))
                })
                .or_insert(vec![(*conn.origin().in_port(), *conn.origin().out_port())]);
        });
    grouped
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
            let in_self_connection = ports_in.iter().any(|port_id| {
                self_ports
                    .find(port_id)
                    .or_else(|| self_ports.find(port_id))
                    .is_some()
            });
            let out_self_connection = ports_out.iter().any(|port_id| {
                self_ports
                    .find(port_id)
                    .or_else(|| self_ports.find(port_id))
                    .is_some()
            });
            let mut connected = HashMap::new();
            connected.insert(entity.sig().id, ports_in.len());
            let joined_port_in = Port {
                provided_interface: None,
                provided_required_interface: None,
                required_interface: None,
                sig: sig_producer.next_for("joined port IN"),
                port_type: if in_self_connection {
                    PortType::Left
                } else {
                    PortType::Right
                },
                contains: ports_in,
                connected,
                label: if &entity.sig().id == comp_joint_out {
                    Some(entity.sig().short_name.to_owned())
                } else {
                    find(&entity.components, &entity.compositions, comp_joint_out)
                        .map(|en| en.sig().short_name.to_owned())
                },
                visibility: true,
            };
            let mut connected = HashMap::new();
            connected.insert(entity.sig().id, ports_out.len());
            let joined_port_out = Port {
                provided_interface: None,
                provided_required_interface: None,
                required_interface: None,
                sig: sig_producer.next_for("joined port OUT"),
                port_type: if out_self_connection {
                    PortType::Right
                } else {
                    PortType::Left
                },
                contains: ports_out,
                connected,
                label: if &entity.sig().id == comp_joint_in {
                    Some(entity.sig().short_name.to_owned())
                } else {
                    find(&entity.components, &entity.compositions, comp_joint_in)
                        .map(|en| en.sig().short_name.to_owned())
                },
                visibility: true,
            };
            added_connections.push(Representation::Origin(Connection {
                joint_in: Joint {
                    component: *comp_joint_in,
                    port: joined_port_in.sig.id,
                    grouped: None,
                },
                joint_out: Joint {
                    component: *comp_joint_out,
                    port: joined_port_out.sig.id,
                    grouped: None,
                },
                sig: sig_producer.next_for("joined connection"),
                visibility: true,
            }));
            added_ports.push((*comp_joint_in, Representation::Origin(joined_port_in)));
            added_ports.push((*comp_joint_out, Representation::Origin(joined_port_out)));
        });
    entity.connections.iter_mut().for_each(|conn| {
        let port_in = added_ports
            .iter()
            .find(|(_, p)| p.origin().contains.contains(conn.origin().in_port()))
            .map(|(_, p)| p);
        let port_out = added_ports
            .iter()
            .find(|(_, p)| p.origin().contains.contains(conn.origin().out_port()))
            .map(|(_, p)| p);
        if let (Some(port_in), true) = (port_in, port_out.is_none()) {
            conn.origin_mut().joint_in.grouped = Some(port_in.sig().id);
        } else if let (Some(port_out), true) = (port_out, port_in.is_none()) {
            conn.origin_mut().joint_out.grouped = Some(port_out.sig().id);
        } else if port_in.is_some() && port_out.is_some() {
            conn.origin_mut().hide();
        }
    });
    entity.connections.extend(added_connections);
    while let Some((component_id, added_port)) = added_ports.pop() {
        if let Some(component) = entity
            .components
            .iter_mut()
            .find(|c| c.sig().id == component_id)
        {
            component
                .origin_mut()
                .ports
                .origin_mut()
                .hide(&added_port.origin().contains);
            component
                .origin_mut()
                .ports
                .origin_mut()
                .add(added_port, None);
        } else if let Some(composition) = entity
            .compositions
            .iter_mut()
            .find(|c| c.sig().id == component_id)
        {
            composition
                .origin_mut()
                .ports
                .origin_mut()
                .hide(&added_port.origin().contains);
            composition
                .origin_mut()
                .ports
                .origin_mut()
                .add(added_port, None);
        } else if entity.sig.id == component_id {
            entity
                .ports
                .origin_mut()
                .hide(&added_port.origin().contains);
            entity.ports.origin_mut().add(added_port, None);
        }
    }
}

pub fn group_unbound_ports(entity: &mut Composition, sig_producer: &mut SignatureProducer) {
    let unbound_ports = entity
        .ports
        .origin()
        .ports
        .iter()
        .filter(|p| p.origin().connected.is_empty() && p.origin().contains.is_empty())
        .map(|p| p.sig().id)
        .collect::<Vec<usize>>();
    if !unbound_ports.is_empty() && unbound_ports.len() != 1 {
        entity.ports.origin_mut().hide(&unbound_ports);
        entity.ports.origin_mut().add(
            Representation::Origin(Port {
                provided_interface: None,
                provided_required_interface: None,
                required_interface: None,
                sig: sig_producer.next_for("unbound grouped"),
                port_type: PortType::Left,
                contains: unbound_ports,
                connected: HashMap::new(),
                visibility: true,
                label: None,
            }),
            Some(0),
        );
    }
    for component in entity.components.iter_mut() {
        let unbound_ports = component
            .origin()
            .ports
            .origin()
            .ports
            .iter()
            .filter(|p| p.origin().connected.is_empty() && p.origin().contains.is_empty())
            .map(|p| p.sig().id)
            .collect::<Vec<usize>>();
        if unbound_ports.is_empty() || unbound_ports.len() == 1 {
            continue;
        }
        component
            .origin_mut()
            .ports
            .origin_mut()
            .hide(&unbound_ports);
        component.origin_mut().ports.origin_mut().add(
            Representation::Origin(Port {
                sig: sig_producer.next_for("unbound grouped"),
                provided_interface: None,
                provided_required_interface: None,
                required_interface: None,
                port_type: PortType::Left,
                contains: unbound_ports,
                connected: HashMap::new(),
                visibility: true,
                label: None,
            }),
            Some(0),
        );
    }
}
