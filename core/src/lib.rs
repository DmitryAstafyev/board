extern crate console_error_panic_hook;
extern crate wasm_bindgen;

mod entity;
mod error;
mod render;
use entity::{
    dummy::{Dummy, SignatureProducer},
    Composition, Signature,
};
use error::E;
use render::{options::Options, Grid, Relative, Render, Style};
use std::ops::RangeInclusive;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct Board {
    render: Render<Composition>,
    context: Option<CanvasRenderingContext2d>,
    width: u32,
    height: u32,
    grid: Grid,
    options: Options,
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
            width: 0,
            height: 0,
            grid,
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
            width: 0,
            height: 0,
            grid,
        }
    }

    #[wasm_bindgen]
    pub fn init(&mut self, composition: JsValue, expanded: Vec<usize>) -> Result<(), String> {
        let composition = serde_wasm_bindgen::from_value::<Composition>(composition)
            .map_err(|e| E::Serde(e.to_string()))?;
        self.render = Render::<Composition>::new(composition, &self.options);
        let mut grid_options = self.options.grid.clone();
        grid_options.padding = 0;
        self.grid = Grid::new(&grid_options);
        Ok(self.render.calc(&mut self.grid, &expanded, &self.options)?)
    }

    #[wasm_bindgen]
    pub fn bind(&mut self, canvas_el_id: &str) -> Result<(), String> {
        let document = web_sys::window()
            .ok_or(E::Dom("Window object isn't found".to_string()))?
            .document()
            .ok_or(E::Dom("Document object isn't found".to_string()))?;
        let canvas = document
            .get_element_by_id(canvas_el_id)
            .ok_or(E::Dom(format!(
                "Fail to find canvas with id[{canvas_el_id}]"
            )))?;
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|e| {
                E::Dom(format!(
                    "Fail to convert into HtmlCanvasElement: {}",
                    e.to_string()
                ))
            })?;
        self.width = canvas.width();
        self.height = canvas.height();
        let _ = self.context.insert(
            canvas
                .get_context("2d")
                .map_err(|_| E::Dom("Fail to get context from canvas".to_string()))?
                .ok_or(E::Dom("Document isn't found".to_string()))?
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .map_err(|e| {
                    E::Dom(format!(
                        "Fail to convert into 2d context; error: {}",
                        e.to_string()
                    ))
                })?,
        );
        Ok(())
    }

    #[wasm_bindgen]
    pub fn render(&mut self, x: i32, y: i32, zoom: f64) -> Result<(), String> {
        if let Some(mut context) = self.context.take() {
            context.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
            let targets = self.grid.viewport((x, y), (self.width, self.height), zoom);
            let relative = Relative::new(x, y, Some(zoom));
            if let Err(e) = self.render.draw(
                &mut context,
                &relative,
                &targets
                    .iter()
                    .map(|(id, _, _)| id.parse::<usize>().unwrap())
                    .collect(),
                &self.options,
            ) {
                self.context = Some(context);
                Err(e)?
            } else {
                let _ = self.grid.draw(&mut context, &relative);
                self.context = Some(context);
                Ok(())
            }
        } else {
            Err(E::NoCanvasContext)?
        }
    }
    #[wasm_bindgen]
    pub fn who(
        &self,
        x: i32,
        y: i32,
        target_x: i32,
        target_y: i32,
        around: i32,
        zoom: f64,
    ) -> Result<JsValue, String> {
        let relative = Relative::new(x, y, Some(zoom));
        let ids = self.grid.point((target_x, target_y), around, &relative);
        let inner = self.render.find(&(target_x, target_y), zoom)?;
        let ports = self.render.find_ports(&ids, &(target_x, target_y))?;
        let elements = [ids, inner, ports].concat();
        serde_wasm_bindgen::to_value(&elements).map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn get_coors_by_ids(
        &self,
        x: i32,
        y: i32,
        zoom: f64,
        ids: Vec<usize>,
    ) -> Result<JsValue, String> {
        let relative = Relative::new(x, y, Some(zoom));
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
    pub fn draw_by_id(
        &mut self,
        id: usize,
        stroke_style: Option<String>,
        fill_style: Option<String>,
        x: i32,
        y: i32,
        zoom: f64,
    ) -> Result<(), String> {
        if let Some(mut context) = self.context.take() {
            if let Err(e) = self.render.draw_by_id(
                &self.grid,
                &mut context,
                &Relative::new(x, y, Some(zoom)),
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
}

// #[wasm_bindgen]
// pub fn dummy(canvas_el_id: &str, components: usize, ports: usize) {
//     panic::set_hook(Box::new(console_error_panic_hook::hook));
//     let document = web_sys::window().unwrap().document().unwrap();
//     let canvas = document.get_element_by_id(canvas_el_id).unwrap();
//     let canvas: web_sys::HtmlCanvasElement = canvas
//         .dyn_into::<web_sys::HtmlCanvasElement>()
//         .map_err(|_| ())
//         .unwrap();
//     let mut context = canvas
//         .get_context("2d")
//         .unwrap()
//         .unwrap()
//         .dyn_into::<web_sys::CanvasRenderingContext2d>()
//         .unwrap();
//     let mut producer = SignatureProducer::new();
//     let mut composition = Composition::dummy(
//         &mut producer,
//         (
//             RangeInclusive::new(components, components + components / 5),
//             RangeInclusive::new(ports, ports + ports),
//         ),
//     );
//     let mut render = Render::<Composition>::new(composition);
//     if let Err(err) = render.calc() {
//         console_log!("Opps, error: {err}");
//     }
//     if let Err(err) = render.draw(&mut context, &Relative::new(50, 50)) {
//         console_log!("Opps, error: {err}");
//     }
// }
