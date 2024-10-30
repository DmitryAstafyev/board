use wasm_bindgen::JsValue;

use super::options::RectColor;

#[derive(Debug)]
pub struct Style {
    pub stroke_style: String,
    pub fill_style: String,
}

impl Style {
    pub fn apply(&self, context: &mut web_sys::CanvasRenderingContext2d) {
        context.set_fill_style(&JsValue::from_str(self.fill_style.as_str()));
        context.set_stroke_style(&JsValue::from_str(self.stroke_style.as_str()));
    }
}

impl From<&RectColor> for Style {
    fn from(value: &RectColor) -> Self {
        Self {
            stroke_style: value.stroke.clone(),
            fill_style: value.fill.clone(),
        }
    }
}
