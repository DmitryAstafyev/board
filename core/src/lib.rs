extern crate console_error_panic_hook;
extern crate wasm_bindgen;

mod entity;
mod error;
mod render;
mod state;

use entity::{
    dummy::{Dummy, SignatureProducer},
    Composition, Signature,
};
use error::E;
use render::{options::Options, Grid, Render, Style};
use state::State;
use std::ops::RangeInclusive;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Board {
    render: Render<Composition>,
    context: Option<CanvasRenderingContext2d>,
    canvas: Option<HtmlCanvasElement>,
    width: u32,
    height: u32,
    grid: Grid,
    options: Options,
    state: State,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen]
    pub fn dummy(components: usize, ports: usize) -> Self {
        let mut producer = SignatureProducer::new(0);
        let composition = Composition::dummy(
            &mut producer,
            (
                RangeInclusive::new(components, components + components / 5),
                RangeInclusive::new(ports, ports + ports),
            ),
        );
        let options = Options::default();
        let render = Render::<Composition>::new(composition, &options);
        let mut grid_options = options.grid.clone();
        grid_options.padding = 0;
        let grid = Grid::new(&grid_options);
        Self {
            options,
            render,
            context: None,
            canvas: None,
            width: 0,
            height: 0,
            grid,
            state: State::new(),
        }
    }

    #[wasm_bindgen(constructor)]
    pub fn new(options: JsValue) -> Self {
        let options = match serde_wasm_bindgen::from_value::<Options>(options) {
            Ok(options) => options,
            Err(err) => {
                console_log!("Fail to parse options: {err}; default will be used.");
                Options::default()
            }
        };
        let render = Render::<Composition>::new(Composition::new(Signature::fake()), &options);
        let mut grid_options = options.grid.clone();
        grid_options.padding = 0;
        let grid = Grid::new(&grid_options);
        Self {
            options,
            render,
            context: None,
            canvas: None,
            width: 0,
            height: 0,
            grid,
            state: State::new(),
        }
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
        self.width = canvas.width();
        self.height = canvas.height();
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
        let _ = cx.translate(0.5, 0.5);
        let _ = self.context.insert(cx);
        let _ = self.canvas.insert(canvas);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn update_size(&mut self) -> Result<(), String> {
        let canvas = self
            .canvas
            .as_mut()
            .ok_or(String::from("Canvas isn't attached"))?;
        self.width = canvas.width();
        self.height = canvas.height();
        Ok(())
    }

    #[wasm_bindgen]
    pub fn bind(&mut self, composition: JsValue, expanded: Vec<usize>) -> Result<(), String> {
        let composition = serde_wasm_bindgen::from_value::<Composition>(composition)
            .map_err(|e| E::Serde(e.to_string()))?;
        self.render = Render::<Composition>::new(composition, &self.options);
        let mut grid_options = self.options.grid.clone();
        grid_options.padding = 0;
        self.grid = Grid::new(&grid_options);
        Ok(self.render.calc(
            self.context.as_mut().ok_or(E::NoCanvasContext)?,
            &mut self.grid,
            &expanded,
            &self.state.get_view_relative(),
            &self.options,
        )?)
    }

    #[wasm_bindgen]
    pub fn render(&mut self) -> Result<(), String> {
        let cx = self.context.as_mut().ok_or(E::NoCanvasContext)?;
        cx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        let targets = self.grid.viewport(
            (self.state.x, self.state.y),
            (self.width, self.height),
            self.state.zoom,
        );
        if let Err(e) = self.render.draw(
            cx,
            &self.state.get_view_relative(),
            &targets
                .iter()
                .map(|(id, _, _)| id.parse::<usize>().unwrap())
                .collect(),
            &self.options,
            &self.state,
        ) {
            Err(e)?
        } else {
            let _ = self.grid.draw(cx, &self.state.get_view_relative());
            Ok(())
        }
    }

    #[wasm_bindgen]
    pub fn set_filter(&mut self, filter: Option<String>) -> Result<(), String> {
        self.state
            .set_filtered(self.render.get_filtered_ports(filter));
        self.render()?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn who(&self, target_x: i32, target_y: i32, around: i32) -> Result<JsValue, String> {
        let relative = self.state.get_view_relative();
        let ids = self.grid.point((target_x, target_y), around, &relative);
        let inner = self.render.find(&(target_x, target_y), self.state.zoom)?;
        let ports = self.render.find_ports(
            &self.grid.point(
                (target_x, target_y),
                self.grid.as_px(self.options.grid.cells_space_horizontal),
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
        let relative = self.state.get_view_relative();
        let ports = self.render.get_coors_by_ids(&ids, &relative)?;
        let components = self.grid.get_coors_by_ids(&ids, &relative);
        let elements = [components, ports].concat();
        serde_wasm_bindgen::to_value(&elements).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn get_connection_info(&self, port: usize) -> Result<JsValue, String> {
        let result = self.render.get_connection_info(port);
        serde_wasm_bindgen::to_value(&result).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn get_connections_info_by_port(&self, port: usize) -> Result<JsValue, String> {
        let result = self.render.get_connections_info_by_port(port);
        serde_wasm_bindgen::to_value(&result).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn get_connections_info_by_component(&self, port: usize) -> Result<JsValue, String> {
        let result = self.render.get_connections_info_by_component(port);
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
            if let Err(e) = self.render.draw_by_id(
                &self.grid,
                &mut context,
                &self.state.get_view_relative(),
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
    pub fn get_groupped_ports(&self) -> Result<JsValue, String> {
        let ports: Vec<(usize, Vec<usize>)> = self.render.get_groupped_ports()?;
        serde_wasm_bindgen::to_value(&ports).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn get_size(&mut self) -> Result<JsValue, String> {
        serde_wasm_bindgen::to_value(&self.grid.get_size_px()).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn set_view_state(&mut self, x: i32, y: i32, zoom: f64) {
        self.state.set_view_state(x, y, zoom);
    }

    #[wasm_bindgen]
    pub fn toggle_component(&mut self, id: usize) -> Result<(), String> {
        if let Some(comp) = self.render.origin().get_component(id) {
            let rel_connections = self.render.origin().find_connections_by_component(id);
            let rel_ports = [
                rel_connections
                    .iter()
                    .flat_map(|conn| [&conn.joint_in.port, &conn.joint_out.port])
                    .collect::<Vec<&usize>>(),
                comp.ports
                    .origin()
                    .ports
                    .iter()
                    .map(|port| &port.origin().sig.id)
                    .collect::<Vec<&usize>>(),
            ]
            .concat();
            let components = rel_connections
                .iter()
                .flat_map(|conn| [&conn.joint_in.component, &conn.joint_out.component])
                .collect::<Vec<&usize>>();
            if self.state.is_component_selected(&id) {
                rel_ports.iter().for_each(|id| {
                    self.state.remove_port(id);
                });
                self.state.remove_component(&id);
                components.iter().for_each(|id| {
                    self.state.remove_component(id);
                });
            } else {
                rel_ports.iter().for_each(|id| {
                    self.state.insert_port(id);
                });
                self.state.insert_component(&id);
                components.iter().for_each(|id| {
                    self.state.insert_component(id);
                });
            }
            self.render()?;
        }
        Ok(())
    }

    #[wasm_bindgen]
    pub fn toggle_port(&mut self, id: usize) -> Result<(), String> {
        let connections = self.render.origin().find_connections_by_port(id);
        let inserted = self.state.toggle_port(&id);
        for connection in connections.iter() {
            let rel_port = if connection.joint_in.port == id {
                connection.joint_out.port
            } else {
                connection.joint_in.port
            };
            if inserted {
                // Added
                self.state.insert_port(&rel_port);
                self.state.insert_component(&connection.joint_out.component);
                self.state.insert_component(&connection.joint_in.component);
            } else {
                // Removed
                self.state.remove_port(&rel_port);
                if !self.state.is_any_port_selected(
                    self.render
                        .origin()
                        .find_ports_by_component(connection.joint_in.component),
                ) {
                    self.state.remove_component(&connection.joint_in.component);
                }
                if !self.state.is_any_port_selected(
                    self.render
                        .origin()
                        .find_ports_by_component(connection.joint_out.component),
                ) {
                    self.state.remove_component(&connection.joint_out.component);
                }
            }
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
    pub fn highlight_connection_by_port(&mut self, id: usize) -> Result<(), String> {
        let has_to_be_rendered =
            if let Some(rel_port) = self.render.origin().find_connected_port(id) {
                self.state.highlight_port(&rel_port)
            } else {
                false
            };
        if self.state.highlight_port(&id) || has_to_be_rendered {
            self.render()
        } else {
            Ok(())
        }
    }

    #[wasm_bindgen]
    pub fn unhighlight_connection_by_port(&mut self, id: usize) -> Result<(), String> {
        let has_to_be_rendered =
            if let Some(rel_port) = self.render.origin().find_connected_port(id) {
                self.state.unhighlight_port(&rel_port)
            } else {
                false
            };
        if self.state.unhighlight_port(&id) || has_to_be_rendered {
            self.render()
        } else {
            Ok(())
        }
    }
}
