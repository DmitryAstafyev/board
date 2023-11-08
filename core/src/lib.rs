extern crate wasm_bindgen;

mod entity;
mod representation;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello(input: &str) -> Result<String, String> {
    Ok(format!("{input}{input}"))
}
