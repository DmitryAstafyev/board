use crate::render::{grid, Ratio, Relative};
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct Params {
    pub cell: u32,
    pub min_w: i32,
    pub pad_v: i32,
    pub pad_h: i32,
}

impl Params {
    pub fn new(ratio: &Ratio) -> Self {
        Self {
            cell: ratio.get(grid::CELL),
            min_w: ratio.get(64),
            pad_v: ratio.get(3),
            pad_h: ratio.get(8),
        }
    }
}

#[derive(Debug)]
pub enum Align {
    Left,
    Right,
}
#[derive(Debug)]
pub struct Label {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub label: String,
    pub subtitle: Option<String>,
    pub badge: Option<(String, String, String)>,
    pub subbadge: Option<(String, String, String)>,
    pub padding: i32,
    pub id: String,
    pub align: Align,
    pub params: Params,
}

impl Label {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        label: String,
        subtitle: Option<String>,
        // value, bk_color, fg_color
        badge: Option<(String, String, String)>,
        // value, bk_color, fg_color
        subbadge: Option<(String, String, String)>,
        padding: i32,
        id: String,
        align: Align,
        ratio: &Ratio,
    ) -> Self {
        let params = Params::new(ratio);
        Self {
            x,
            y,
            w,
            h,
            label,
            subtitle,
            badge,
            subbadge,
            padding: ratio.get(padding),
            id,
            align,
            params,
        }
    }
    pub fn get_box_size(&self) -> (i32, i32) {
        (self.w, self.h)
    }
    pub fn set_coors(&mut self, x: Option<i32>, y: Option<i32>) {
        if let Some(x) = x {
            self.x = x;
        }
        if let Some(y) = y {
            self.y = y;
        }
    }
    pub fn get_coors(&self) -> (i32, i32) {
        match self.align {
            Align::Left => (self.x + self.padding, self.y),
            Align::Right => (self.x - self.w - self.padding, self.y),
        }
    }

    pub fn get_coors_with_zoom(&self, relative: &Relative) -> (i32, i32) {
        match self.align {
            Align::Left => (relative.zoom(self.x + self.padding), relative.zoom(self.y)),
            Align::Right => (
                relative.zoom(self.x - self.padding) - self.w,
                relative.zoom(self.y),
            ),
        }
    }

    pub fn calc(&mut self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        let text_hor_padding = relative.zoom(self.params.pad_h) as f64;
        self.h = relative.zoom((self.params.cell as f64 * 0.75).floor() as i32);
        context.set_text_baseline("top");
        context.set_font(&format!(
            "{}px serif",
            (self.h as f64 * if self.subtitle.is_some() { 0.55 } else { 0.7 }).round()
        ));
        self.w = if let Ok(metric) = context.measure_text(&self.label) {
            metric.width() as i32
        } else {
            self.params.min_w
        } + (text_hor_padding as i32) * 2;
    }
    // Take into account self.w already condiser zooming, because it's calculated by
    // render and already reflects zoom-factor.
    pub fn render(&mut self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        self.calc(context, relative);
        let text_hor_padding = relative.zoom(self.params.pad_h) as f64;
        let text_ver_padding = relative.zoom(self.params.pad_v) as f64;
        let x = match self.align {
            Align::Left => relative.x(self.x + self.padding),
            Align::Right => relative.x(self.x - self.padding) - self.w,
        } as f64;
        let y = relative.y(self.y) as f64;
        context.fill_rect(x, y, self.w as f64, self.h as f64);
        context.stroke_rect(x, y, self.w as f64, self.h as f64);
        context.set_fill_style(&JsValue::from_str("rgb(0,0,0)"));
        if let Some(subtitle) = self.subtitle.as_ref() {
            let _ = context.fill_text(
                &self.label,
                x + text_hor_padding,
                y + text_ver_padding * 0.6,
            );
            context.set_font(&format!("{}px serif", (self.h as f64 * 0.4).round()));
            context.set_fill_style(&JsValue::from_str("rgb(40,40,40)"));
            let _ = context.fill_text(subtitle, x + text_hor_padding, y + self.h as f64 * 0.6);
        } else {
            let _ = context.fill_text(&self.label, x + text_hor_padding, y + text_ver_padding);
        }
        if let Some((badge, bk_c, fg_c)) = &self.badge {
            context.set_font(&format!("{}px serif", (self.h as f64 * 0.4).round()));
            let bw = if let Ok(metric) = context.measure_text(badge) {
                metric.width()
            } else {
                36f64
            };
            let h = self.h as f64 * 0.7;
            let p = self.h as f64 * 0.15;
            let x = match self.align {
                Align::Left => x - bw - p * 2.0,
                Align::Right => x + self.w as f64,
            };
            context.set_fill_style(&JsValue::from_str(bk_c));
            context.fill_rect(x, y + p, bw + p * 2.0, h);
            context.set_fill_style(&JsValue::from_str(fg_c));
            let _ = context.fill_text(badge, x + p, y + p * 2.0);
        }
        if let Some((badge, bk_c, fg_c)) = &self.subbadge {
            context.set_font(&format!("{}px serif", (self.h as f64 * 0.4).round()));
            let bw = if let Ok(metric) = context.measure_text(badge) {
                metric.width()
            } else {
                36f64
            };
            let h = self.h as f64 * 0.7;
            let p = self.h as f64 * 0.15;
            let x = match self.align {
                Align::Left => x + self.w as f64 + p,
                Align::Right => x - bw - p * 3.0,
            };
            context.set_fill_style(&JsValue::from_str(bk_c));
            context.fill_rect(x, y + p, bw + p * 2.0, h);
            // context.set_stroke_style(&JsValue::from_str(fg_c));
            // context.stroke_rect(x, y + p, bw + p * 2.0, h);
            context.set_fill_style(&JsValue::from_str(fg_c));
            let _ = context.fill_text(badge, x + p, y + p * 2.0);
        }
    }
}
