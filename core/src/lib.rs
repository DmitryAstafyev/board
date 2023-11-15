extern crate console_error_panic_hook;
extern crate wasm_bindgen;

mod entity;
mod error;
mod render;

use entity::{
    dummy::{Dummy, SignatureProducer},
    Composition,
};
use render::{Relative, Render};
use std::{ops::RangeInclusive, panic};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;

#[wasm_bindgen]
pub fn hello(input: &str) -> Result<String, String> {
    Ok(format!("{input}{input}"))
}

#[wasm_bindgen]
pub fn dummy(canvas_el_id: &str, components: usize, ports: usize) {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_el_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let mut context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let mut producer = SignatureProducer::new();
    let mut composition = Composition::dummy(
        &mut producer,
        (
            RangeInclusive::new(components, components + components / 5),
            RangeInclusive::new(ports, ports + ports),
        ),
    );
    let mut render = Render::<Composition>::new(composition);
    if let Err(err) = render.calc() {
        console_log!("Opps, error: {err}");
    }
    if let Err(err) = render.draw(&mut context, &Relative::new(50, 50)) {
        console_log!("Opps, error: {err}");
    }
}
