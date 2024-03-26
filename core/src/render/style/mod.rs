use wasm_bindgen::JsValue;

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
