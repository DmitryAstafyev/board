extern crate console_error_panic_hook;
extern crate wasm_bindgen;

mod entity;
mod error;
mod render;
mod state;

use entity::{
    dummy::{Dummy, SignatureProducer},
    Composition, IsInputPort, Signature,
};
use error::E;
use render::{options::Options, Grid, Ratio, Render, Style};
use serde::{Deserialize, Serialize};
use state::State;
use std::ops::RangeInclusive;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Debug, Deserialize, Serialize)]
struct Active {
    pub grid: Grid,
    pub composition: Render<Composition>,
}

impl Active {
    pub fn new(options: &Options, composition: Render<Composition>) -> Self {
        let mut grid_options = options.grid.clone();
        grid_options.vpadding = 0;
        grid_options.hpadding = 0;
        Self {
            grid: Grid::new(&grid_options, options.ratio()),
            composition,
        }
    }
    pub fn drop_grid(&mut self, options: &Options) {
        let mut grid_options = options.grid.clone();
        grid_options.vpadding = 0;
        grid_options.hpadding = 0;
        self.grid = Grid::new(&grid_options, options.ratio());
        self.grid.apply_margin();
    }

    pub fn calc(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        state: &State,
        options: &Options,
    ) -> Result<(), E> {
        self.composition
            .calc(context, &mut self.grid, state, options)?;
        self.grid.apply_margin();
        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct SnapshotSe<'a> {
    active: &'a Active,
    options: &'a Options,
    state: &'a State,
    ratio: &'a Ratio,
    sig_producer: &'a SignatureProducer,
}

#[derive(Debug, Deserialize)]
struct SnapshotDe {
    active: Active,
    options: Options,
    state: State,
    ratio: Ratio,
    sig_producer: SignatureProducer,
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Board {
    active: Active,
    context: Option<CanvasRenderingContext2d>,
    canvas: Option<HtmlCanvasElement>,
    options: Options,
    state: State,
    ratio: Ratio,
    sig_producer: SignatureProducer,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen]
    pub fn dummy(components: usize, ports: usize, selcb: js_sys::Function) -> Self {
        let mut sig_producer = SignatureProducer::new(0);
        let composition = Composition::dummy(
            &mut sig_producer,
            (
                RangeInclusive::new(components, components + components / 5),
                RangeInclusive::new(ports, ports + ports),
            ),
        );
        let options = Options::default();
        let composition =
            Render::<Composition>::new(composition, true, &options, &mut sig_producer);
        let active = Active::new(&options, composition);
        let state = State::new(
            active.grid.as_px(options.grid.hmargin),
            active.grid.as_px(options.grid.vmargin),
            selcb,
        );
        let ratio = options.ratio();
        Self {
            options,
            active,
            context: None,
            canvas: None,
            state,
            ratio,
            sig_producer,
        }
    }

    #[wasm_bindgen(constructor)]
    pub fn new(options: JsValue, selcb: js_sys::Function) -> Self {
        let options = match serde_wasm_bindgen::from_value::<Options>(options) {
            Ok(options) => options,
            Err(err) => {
                console_log!("Fail to parse options: {err}; default will be used.");
                Options::default()
            }
        };
        let mut sig_producer = SignatureProducer::new(0);
        let composition = Render::<Composition>::new(
            Composition::new(Signature::default()),
            true,
            &options,
            &mut sig_producer,
        );
        let active = Active::new(&options, composition);
        let state = State::new(
            active.grid.as_px(options.grid.hmargin),
            active.grid.as_px(options.grid.vmargin),
            selcb,
        );
        let ratio = options.ratio();
        Self {
            options,
            active,
            context: None,
            canvas: None,
            state,
            ratio,
            sig_producer,
        }
    }

    #[wasm_bindgen]
    pub fn set_options(&mut self, options: JsValue) -> Result<(), String> {
        let options =
            serde_wasm_bindgen::from_value::<Options>(options).map_err(|e| e.to_string())?;
        self.options = options;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn attach(&mut self, canvas_el_id: &str) -> Result<(), String> {
        let document = web_sys::window()
            .ok_or(E::Dom("Window object isn't found".to_string()))?
            .document()
            .ok_or(E::Dom("Document object isn't found".to_string()))?;
        let canvas = document
            .get_element_by_id(canvas_el_id)
            .ok_or(E::Dom(format!(
                "Fail to find canvas with id[{canvas_el_id}]"
            )))?;
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().map_err(|e| {
            E::Dom(format!(
                "Fail to convert into HtmlCanvasElement: {}",
                e.to_string()
            ))
        })?;
        let cx = canvas
            .get_context("2d")
            .map_err(|_| E::Dom("Fail to get context from canvas".to_string()))?
            .ok_or(E::Dom("Document isn't found".to_string()))?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .map_err(|e| {
                E::Dom(format!(
                    "Fail to convert into 2d context; error: {}",
                    e.to_string()
                ))
            })?;
        cx.set_transform(
            self.options.ratio as f64,
            0.0,
            0.0,
            self.options.ratio as f64,
            0.0,
            0.0,
        )
        .map_err(|e| E::Dom(format!("Fail to transform; error: {e:?}")))?;
        let _ = self.context.insert(cx);
        let _ = self.canvas.insert(canvas);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn bind(&mut self, composition: JsValue) -> Result<(), String> {
        let composition = serde_wasm_bindgen::from_value::<Composition>(composition)
            .map_err(|e| E::Serde(e.to_string()))?;
        self.state.drop();
        self.active = Active::new(
            &self.options,
            Render::<Composition>::new(composition, true, &self.options, &mut self.sig_producer),
        );
        self.active.calc(
            self.context.as_mut().ok_or(E::NoCanvasContext)?,
            &self.state,
            &self.options,
        )?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn save_snapshot(&self) -> Result<Vec<u8>, String> {
        let snapshot = SnapshotSe {
            active: &self.active,
            options: &self.options,
            state: &self.state,
            ratio: &self.ratio,
            sig_producer: &self.sig_producer,
        };
        bincode::serialize(&snapshot).map_err(|e| format!("Fail to convert state into bytes: {e}"))
    }

    #[wasm_bindgen]
    pub fn load_snapshot(
        &mut self,
        snapshot: Vec<u8>,
        selcb: js_sys::Function,
    ) -> Result<(), String> {
        let snapshot: SnapshotDe = bincode::deserialize(&snapshot)
            .map_err(|e| format!("Fail to convert state into bytes: {e}"))?;
        self.active = snapshot.active;
        self.state = snapshot.state;
        self.options = snapshot.options;
        self.ratio = snapshot.ratio;
        self.sig_producer = snapshot.sig_producer;
        self.state.selection.set_selcb(selcb);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn recalc(&mut self) -> Result<(), String> {
        self.active.drop_grid(&self.options);
        let zoom = self.state.zoom;
        // Calculation goes without considering zoom factor. During calculation zoom factor should be 1.0
        self.state.zoom = 1.0;
        self.active.calc(
            self.context.as_mut().ok_or(E::NoCanvasContext)?,
            &self.state,
            &self.options,
        )?;
        self.state.zoom = zoom;
        self.render()
    }

    #[wasm_bindgen]
    pub fn render(&mut self) -> Result<(), String> {
        let cx = self.context.as_mut().ok_or(E::NoCanvasContext)?;
        let cw = self
            .canvas
            .as_ref()
            .ok_or(String::from("Board isn't inited; no context"))?
            .width();
        let ch = self
            .canvas
            .as_ref()
            .ok_or(String::from("Board isn't inited; no context"))?
            .height();
        cx.clear_rect(0.0, 0.0, cw as f64, ch as f64);
        let targets = self.active.grid.viewport(
            (self.state.x_margin(), self.state.y_margin()),
            (cw, ch),
            self.state.zoom,
        );
        let relative = self.state.get_grid_relative();
        if let Err(e) = self.active.composition.draw(
            cx,
            &relative,
            &targets
                .iter()
                .map(|(id, _, _)| id.parse::<usize>().unwrap())
                .collect(),
            &self.options,
            &self.state,
        ) {
            Err(e)?
        } else {
            let _ = self.active.grid.draw(cx, &relative);
            Ok(())
        }
    }

    #[wasm_bindgen]
    pub fn is_in_viewport(&self, id: usize) -> bool {
        self.active.grid.is_in_viewport(&id)
    }

    #[wasm_bindgen]
    pub fn set_filter(&mut self, filter: Option<String>) {
        self.state
            .set_filtered(self.active.composition.get_filtered_ports(filter));
    }

    #[wasm_bindgen]
    pub fn get_filtered(&self) -> Result<JsValue, String> {
        let empty = Vec::new();
        let filtered = self.state.get_filtered().unwrap_or(&empty);
        serde_wasm_bindgen::to_value(&filtered).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn set_targeted(&mut self, filter: Option<String>) {
        self.state
            .set_targeted(self.active.composition.get_targeted_components(filter));
    }

    #[wasm_bindgen]
    pub fn set_targeted_by_ids(&mut self, ids: Vec<usize>) {
        self.state
            .set_targeted(self.active.composition.get_targeted_components_by_ids(ids));
    }

    #[wasm_bindgen]
    pub fn get_targeted(&self) -> Result<JsValue, String> {
        let empty = (Vec::new(), Vec::new());
        let filtered = self.state.get_targeted().unwrap_or(&empty);
        serde_wasm_bindgen::to_value(&filtered).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn get_all_components(&self) -> Result<JsValue, String> {
        serde_wasm_bindgen::to_value(&self.active.composition.get_all_components())
            .map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn get_components_linked_to(&self, ids: Vec<usize>) -> Result<JsValue, String> {
        serde_wasm_bindgen::to_value(&self.active.composition.get_components_linked_to(ids))
            .map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn set_matches(&mut self, filter: Option<String>) {
        self.state.set_matches(
            self.active.composition.get_matches(filter.clone()),
            self.active.composition.get_matches_extended(filter),
        );
    }

    #[wasm_bindgen]
    pub fn get_matches(&self) -> Result<JsValue, String> {
        let empty = Vec::new();
        let matches = self.state.get_matches().unwrap_or(&empty);
        if let Some(filtered) = self.state.get_filtered() {
            serde_wasm_bindgen::to_value(
                &matches
                    .iter()
                    .filter(|id| filtered.contains(id))
                    .collect::<Vec<&usize>>(),
            )
            .map_err(|e| e.to_string())
        } else {
            serde_wasm_bindgen::to_value(&matches).map_err(|e| e.to_string())
        }
    }

    #[wasm_bindgen]
    pub fn get_extended_matches(&self) -> Result<JsValue, String> {
        let empty = Vec::new();
        let matches = self.state.get_extended_matches().unwrap_or(&empty);
        if let Some(filtered) = self.state.get_filtered() {
            serde_wasm_bindgen::to_value(
                &matches
                    .iter()
                    .filter(|(id, host, _owner)| {
                        host.as_ref()
                            .map(|id| filtered.contains(id))
                            .unwrap_or(filtered.contains(id))
                    })
                    .collect::<Vec<&(usize, Option<usize>, usize)>>(),
            )
            .map_err(|e| e.to_string())
        } else {
            serde_wasm_bindgen::to_value(&matches).map_err(|e| e.to_string())
        }
    }

    #[wasm_bindgen]
    pub fn get_ports_props(&self) -> Result<JsValue, String> {
        serde_wasm_bindgen::to_value(&self.active.composition.origin().get_ports_props())
            .map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn get_comps_props(&self) -> Result<JsValue, String> {
        serde_wasm_bindgen::to_value(&self.active.composition.origin().get_comps_props())
            .map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn set_highlighted(&mut self, highlighted: Option<Vec<usize>>) {
        self.state.set_highlighted(highlighted);
    }

    #[wasm_bindgen]
    pub fn get_highlighted(&self) -> Vec<usize> {
        self.state.get_highlighted().cloned().unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn who(&self, target_x: i32, target_y: i32, around: i32) -> Result<JsValue, String> {
        let relative = self.state.get_grid_relative();
        let around = self.ratio.get(around);
        let target_x = self.state.with_hmargin(self.ratio.get(target_x));
        let target_y = self.state.with_vmargin(self.ratio.get(target_y));
        let ids = self
            .active
            .grid
            .point((target_x, target_y), around, &relative);
        let inner = self
            .active
            .composition
            .find(&(target_x, target_y), self.state.zoom)?;
        let ports = self.active.composition.find_ports(
            &self.active.grid.point(
                (target_x, target_y),
                self.active
                    .grid
                    .as_px(self.options.grid.cells_space_horizontal),
                &relative,
            ),
            &(target_x, target_y),
            &self.state,
        )?;
        let elements = [ids, inner, ports].concat();
        serde_wasm_bindgen::to_value(&elements).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn get_coors_by_ids(&self, ids: Vec<usize>) -> Result<JsValue, String> {
        let relative = self.state.get_grid_relative();
        let ports = self
            .active
            .composition
            .get_coors_by_ids(&ids, &relative, &self.ratio)?;
        let components = self
            .active
            .grid
            .get_coors_by_ids(&ids, &relative, &self.ratio);
        let elements = [components, ports].concat();
        serde_wasm_bindgen::to_value(&elements).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn get_connection(&self, port: usize) -> Result<JsValue, String> {
        let result = self.active.composition.get_connection(port);
        serde_wasm_bindgen::to_value(&result).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn get_connections(&self, port: usize) -> Result<JsValue, String> {
        let result = self.active.composition.get_connections(port);
        serde_wasm_bindgen::to_value(&result).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn get_connections_by_component(&self, component: usize) -> Result<JsValue, String> {
        let result = self
            .active
            .composition
            .get_connections_by_component(component);
        serde_wasm_bindgen::to_value(&result).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn get_all_connections(&self) -> Result<JsValue, String> {
        let result = self.active.composition.get_all_connections();
        serde_wasm_bindgen::to_value(&result).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn draw_by_id(
        &mut self,
        id: usize,
        stroke_style: Option<String>,
        fill_style: Option<String>,
    ) -> Result<(), String> {
        if let Some(mut context) = self.context.take() {
            if let Err(e) = self.active.composition.draw_by_id(
                &self.active.grid,
                &mut context,
                &self.state.get_grid_relative(),
                if let (Some(stroke_style), Some(fill_style)) = (stroke_style, fill_style) {
                    Some(Style {
                        stroke_style,
                        fill_style,
                    })
                } else {
                    None
                },
                id,
                &self.options,
                &self.state,
            ) {
                self.context = Some(context);
                Err(e)?
            } else {
                self.context = Some(context);
                Ok(())
            }
        } else {
            Err(E::NoCanvasContext)?
        }
    }

    #[wasm_bindgen]
    pub fn get_grouped_ports(&self) -> Result<JsValue, String> {
        let ports: Vec<(usize, Vec<usize>)> = self.active.composition.get_grouped_ports()?;
        serde_wasm_bindgen::to_value(&ports).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn get_port(&self, id: usize) -> Result<JsValue, String> {
        let port: Option<&entity::Port> = self.active.composition.get_port(id);
        serde_wasm_bindgen::to_value(&port).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn get_size(&mut self) -> Result<JsValue, String> {
        serde_wasm_bindgen::to_value(&self.active.grid.get_size_invert_px())
            .map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn set_view_state(&mut self, x: i32, y: i32, zoom: f64) {
        self.state
            .set_view_state(self.ratio.get(x), self.ratio.get(y), zoom);
    }

    #[wasm_bindgen]
    pub fn unselect_all(&mut self) -> Result<(), String> {
        self.state.unselect_all(false);
        self.render()?;
        Ok(())
    }
    #[wasm_bindgen]
    pub fn toggle_component(&mut self, id: usize, selfishly: bool) -> Result<(), String> {
        let all = |id: &usize| {
            if let Some(comp) = self.active.composition.origin().get_component(id) {
                [
                    self.active
                        .composition
                        .origin()
                        .find_connections_by_component(id)
                        .iter()
                        .flat_map(|conn| conn.get_ports())
                        .collect::<Vec<&usize>>(),
                    comp.ports
                        .origin()
                        .iter()
                        .map(|port| &port.sig().id)
                        .collect::<Vec<&usize>>(),
                ]
                .concat()
            } else {
                Vec::new()
            }
        };
        let own = |id: &usize| {
            if let Some(comp) = self.active.composition.origin().get_component(id) {
                comp.ports
                    .origin()
                    .iter()
                    .map(|port| &port.sig().id)
                    .collect::<Vec<&usize>>()
            } else {
                Vec::new()
            }
        };
        let linked = |id: &usize| {
            let own = own(id);
            self.active
                .composition
                .origin()
                .find_connections_by_component(id)
                .iter()
                .flat_map(|conn: &&entity::Connection| conn.get_ports())
                .filter(|p| !own.contains(p))
                .collect::<Vec<&usize>>()
        };
        let insert = |state: &mut State, id: &usize, own: Vec<&usize>, linked: Vec<&usize>| {
            own.iter().for_each(|id| {
                state.insert_port(id);
            });
            linked.iter().for_each(|id| {
                state.highlight_port(id);
            });
            state.insert_component(id);
        };
        let remove = |state: &mut State, id: &usize, ports: Vec<&usize>| {
            ports.iter().for_each(|id| {
                state.remove_port(id);
                state.unhighlight_port(id);
            });
            state.remove_component(id);
        };
        let ports = all(&id);
        if self.state.is_component_selected(&id) {
            remove(&mut self.state, &id, ports);
            self.state.components.to_vec().iter().for_each(|id| {
                insert(&mut self.state, id, own(id), linked(id));
            });
            self.state.selection.remove_component(&id).notify();
        } else {
            if selfishly {
                self.state.unselect_all(true);
            }
            insert(&mut self.state, &id, own(&id), linked(&id));
            self.state.selection.insert_component(&id).notify();
        }
        self.render()?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn toggle_port(&mut self, id: usize, selfishly: bool) -> Result<(), String> {
        let connections = self
            .active
            .composition
            .origin()
            .find_connections_by_port(&id);
        if selfishly && !self.state.is_port_selected(&id) {
            self.state.unselect_all(true);
        }
        let inserted = self.state.toggle_port(&id);
        for connection in connections.iter() {
            let rel_port = if (&id).is_input_port(*connection) {
                connection.out_port()
            } else {
                connection.in_port()
            };
            if inserted {
                // Added
                self.state.highlight_port(rel_port);
            } else {
                // Removed
                self.state.unhighlight_port(rel_port);
            }
        }
        self.render()
    }

    #[wasm_bindgen]
    pub fn show_connections_by_ports(
        &mut self,
        left: &[usize],
        right: &[usize],
    ) -> Result<(), String> {
        self.state.unselect_all(true);
        let grouped = self.active.composition.get_grouped_ports()?;
        for (n, id) in left.iter().enumerate() {
            let pair = (*id, right[n]);
            let pair = (
                if let Some(id) = grouped
                    .iter()
                    .find(|(_, inners)| inners.contains(&pair.0))
                    .map(|(p, _)| p)
                {
                    *id
                } else {
                    pair.0
                },
                if let Some(id) = grouped
                    .iter()
                    .find(|(_, inners)| inners.contains(&pair.1))
                    .map(|(p, _)| p)
                {
                    *id
                } else {
                    pair.1
                },
            );
            self.state.insert_port(&pair.0);
            self.state.insert_port(&pair.1);
        }
        self.render()
    }

    #[wasm_bindgen]
    pub fn insert_component(&mut self, id: usize) -> Result<(), String> {
        if self.state.insert_component(&id) {
            self.render()
        } else {
            Ok(())
        }
    }

    #[wasm_bindgen]
    pub fn remove_component(&mut self, id: usize) -> Result<(), String> {
        if self.state.remove_component(&id) {
            self.render()
        } else {
            Ok(())
        }
    }

    #[wasm_bindgen]
    pub fn insert_port(&mut self, id: usize) -> Result<(), String> {
        if self.state.insert_port(&id) {
            self.render()
        } else {
            Ok(())
        }
    }

    #[wasm_bindgen]
    pub fn remove_port(&mut self, id: usize) -> Result<(), String> {
        if self.state.remove_port(&id) {
            self.render()
        } else {
            Ok(())
        }
    }

    #[wasm_bindgen]
    pub fn hover(&mut self, id: usize) -> bool {
        self.state.hover(&id)
    }

    #[wasm_bindgen]
    pub fn unhover(&mut self) -> bool {
        self.state.unhover()
    }

    #[wasm_bindgen]
    pub fn highlight_connection_by_port(&mut self, _id: usize) -> bool {
        // self.state.highlight_port(&id)
        //     || if let Some(rel_port) = self.active.composition.origin().find_connected_port(&id) {
        //         self.state.highlight_port(&rel_port)
        //     } else {
        //         false
        //     }
        false
    }

    #[wasm_bindgen]
    pub fn unhighlight_connection_by_port(&mut self, _id: usize) -> bool {
        false
        // self.state.unhighlight_port(&id)
        //     || if let Some(rel_port) = self.active.composition.origin().find_connected_port(&id) {
        //         self.state.unhighlight_port(&rel_port)
        //     } else {
        //         false
        //     }
    }
}
