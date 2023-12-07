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
use render::{Grid, Relative, Render, Style};
use std::ops::RangeInclusive;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct Board {
    render: Render<Composition>,
    context: Option<CanvasRenderingContext2d>,
    width: u32,
    height: u32,
    grid: Grid,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen]
    pub fn dummy(components: usize, ports: usize) -> Self {
        let mut producer = SignatureProducer::new();
        let composition = Composition::dummy(
            &mut producer,
            (
                RangeInclusive::new(components, components + components / 5),
                RangeInclusive::new(ports, ports + ports),
            ),
        );
        Self {
            render: Render::<Composition>::new(composition),
            context: None,
            width: 0,
            height: 0,
            grid: Grid::new(1),
        }
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            render: Render::<Composition>::new(Composition::new(Signature::fake())),
            context: None,
            width: 0,
            height: 0,
            grid: Grid::new(1),
        }
    }

    #[wasm_bindgen]
    pub fn init(&mut self, composition: JsValue, expanded: Vec<usize>) -> Result<(), String> {
        let composition = serde_wasm_bindgen::from_value::<Composition>(composition)
            .map_err(|e| E::Serde(e.to_string()))?;
        self.render = Render::<Composition>::new(composition);
        self.grid = Grid::new(1);
        Ok(self.render.calc(&mut self.grid, &expanded)?)
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
            if let Err(e) =
                self.render
                    .draw(&mut context, &Relative::new(x, y, Some(zoom)), &targets)
            {
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
    pub fn who(
        &self,
        target_x: u32,
        target_y: u32,
        width: u32,
        height: u32,
        zoom: f64,
    ) -> Result<Vec<usize>, String> {
        Ok(self.grid.in_area(
            (target_x, target_y, target_x + width, target_y + height),
            zoom,
        ))
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
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
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
