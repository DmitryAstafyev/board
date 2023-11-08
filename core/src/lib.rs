extern crate wasm_bindgen;

mod elements;
mod entity;
mod error;
mod representation;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello(input: &str) -> Result<String, String> {
    Ok(format!("{input}{input}"))
}
