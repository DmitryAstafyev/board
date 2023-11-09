pub mod path;
pub mod rectangle;

use std::collections::{HashMap, HashSet};

pub use path::Path;
pub use rectangle::Rectangle;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::console_log;

use crate::elements::relative::Relative;

pub trait Default {
    fn init() -> Form;
}

#[derive(Debug)]
pub enum StartPoint {
    TL,
    TR,
    BL,
    BR,
}

#[derive(Debug)]
pub enum Form {
    Rectangle(Rectangle),
    Path(Path),
}

impl Form {
    // Returns box size: (x,y,w,h)
    pub fn box_size(forms: &[&Form]) -> Option<(i32, i32, i32, i32)> {
        // TODO: needs to be optimized
        let x0 = forms.iter().map(|f| f.get_coors().0).min();
        let y0 = forms.iter().map(|f| f.get_coors().1).min();
        let x1 = forms.iter().map(|f| f.box_width() + f.get_coors().0).max();
        let y1 = forms.iter().map(|f| f.box_height() + f.get_coors().1).max();
        if let (Some(x), Some(y), Some(x1), Some(y1)) = (x0, y0, x1, y1) {
            Some((x, y, x1 - x, y1 - y))
        } else {
            None
        }
    }

    // Area (x,y,x1,y1)
    pub fn is_area_busy(area: (i32, i32, i32, i32), forms: &[&Form]) -> bool {
        for form in forms.iter() {
            let (x, y, x1, y1) = area;
            let (f_x, f_y) = form.get_coors();
            let (f_x1, f_y1) = (form.box_width() + f_x, form.box_height() + f_y);
            if (f_x..f_x1).contains(&x) && (f_y..f_y1).contains(&y) {
                return true;
            }
            if (f_x..f_x1).contains(&x1) && (f_y..f_y1).contains(&y1) {
                return true;
            }
            if (x..x1).contains(&f_x) && (y..y1).contains(&f_y) {
                return true;
            }
            if (x..x1).contains(&f_x1) && (y..y1).contains(&f_y1) {
                return true;
            }
        }
        false
    }

    pub fn box_map(
        forms: &[&Form],
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &crate::elements::relative::Relative,
    ) {
        if let Some((x, y, w, h)) = Form::box_size(forms) {
            let cell = 10;
            let cells_x_count = w / cell;
            let cells_y_count = h / cell;
            //(hor_x, ver_y)
            let mut cells: HashSet<(i32, i32)> = HashSet::new();
            for cur_x in 0..cells_x_count {
                for cur_y in 0..cells_y_count {
                    let x_ = x + cur_x * cell;
                    let y_ = y + cur_y * cell;
                    if !Form::is_area_busy((x_, y_, x_ + cell, y_ + cell), forms) {
                        cells.insert((cur_x, cur_y));
                    }
                }
            }
            let max_size = if cells_x_count < cells_y_count {
                cells_x_count
            } else {
                cells_y_count
            };
            let mut areas: Vec<(StartPoint, i32, (i32, i32, i32, i32))> = vec![];
            // Check from point (x,y)
            if cells.contains(&(0, 0)) {
                let mut busy = false;
                let mut steps = 0;
                for s in 0..max_size {
                    for cur_x in 0..s {
                        for cur_y in 0..s {
                            if !cells.contains(&(cur_x, cur_y)) {
                                busy = true;
                                break;
                            }
                        }
                        if busy {
                            break;
                        }
                    }
                    if busy {
                        break;
                    } else {
                        steps += 1;
                    }
                }
                console_log!(
                    "T-L: steps: {steps}: (0,0) - ({}, {})",
                    x + steps * cell,
                    y + steps * cell
                );
                context.set_stroke_style(&JsValue::from_str("rgb(255,200,0)"));
                context.stroke_rect(
                    relative.x(x) as f64,
                    relative.y(y) as f64,
                    (steps * cell) as f64,
                    (steps * cell) as f64,
                );
                context.stroke();
                areas.push((
                    StartPoint::TL,
                    steps * cell * steps * cell,
                    (x, y, x + steps * cell, y + steps * cell),
                ));
            }
            // Check from point (x,y1)
            if cells.contains(&(0, cells_y_count - 1)) {
                let mut busy = false;
                let mut steps = 0;
                for s in 0..max_size {
                    for cur_x in 0..s {
                        for cur_y in cells_y_count - s..cells_y_count {
                            if !cells.contains(&(cur_x, cur_y)) {
                                busy = true;
                                break;
                            }
                        }
                        if busy {
                            break;
                        }
                    }
                    if busy {
                        break;
                    } else {
                        steps += 1;
                    }
                }
                console_log!(
                    "B-L: steps: {steps}: (0,{}) - ({}, {})",
                    y + h,
                    x + steps * cell,
                    y + h - steps * cell
                );
                context.set_stroke_style(&JsValue::from_str("rgb(255,150,0)"));
                context.stroke_rect(
                    relative.x(x) as f64,
                    relative.y(y + h - steps * cell) as f64,
                    (steps * cell) as f64,
                    (steps * cell) as f64,
                );
                context.stroke();
                areas.push((
                    StartPoint::BL,
                    steps * cell * steps * cell,
                    (x, y + h - steps * cell, x + steps * cell, y + h),
                ));
            }
            // Check from point (x1,y)
            if cells.contains(&(cells_x_count - 1, 0)) {
                let mut busy = false;
                let mut steps = 0;
                for s in 0..max_size {
                    for cur_x in cells_x_count - s..s {
                        for cur_y in 0..s {
                            if !cells.contains(&(cur_x, cur_y)) {
                                busy = true;
                                break;
                            }
                        }
                        if busy {
                            break;
                        }
                    }
                    if busy {
                        break;
                    } else {
                        steps += 1;
                    }
                }
                console_log!(
                    "T-R: steps: {steps}: ({},0) - ({}, {})",
                    x + w,
                    x + w - steps * cell,
                    y + steps * cell
                );
                context.set_stroke_style(&JsValue::from_str("rgb(255,100,0)"));
                context.stroke_rect(
                    relative.x(x + w - steps * cell) as f64,
                    relative.y(y) as f64,
                    (steps * cell) as f64,
                    (steps * cell) as f64,
                );
                context.stroke();
                areas.push((
                    StartPoint::TR,
                    steps * cell * steps * cell,
                    (x + w - steps * cell, y, x + w, y + steps * cell),
                ));
            }
            // Check from point (x1,y1)
            if cells.contains(&(cells_x_count - 1, cells_y_count - 1)) {
                let mut busy = false;
                let mut steps = 0;
                for s in 0..max_size {
                    for cur_x in cells_x_count - s..s {
                        for cur_y in cells_y_count - s..s {
                            if !cells.contains(&(cur_x, cur_y)) {
                                busy = true;
                                break;
                            }
                        }
                        if busy {
                            break;
                        }
                    }
                    if busy {
                        break;
                    } else {
                        steps += 1;
                    }
                }
                console_log!(
                    "B-R: steps: {steps}: ({},{}) - ({}, {})",
                    x + w,
                    y + h,
                    x + w - steps * cell,
                    y + h - steps * cell
                );
                context.set_stroke_style(&JsValue::from_str("rgb(255,0,0)"));
                context.stroke_rect(
                    relative.x(x + w - steps * cell) as f64,
                    relative.y(y + h - steps * cell) as f64,
                    (steps * cell) as f64,
                    (steps * cell) as f64,
                );
                context.stroke();
                areas.push((
                    StartPoint::BR,
                    steps * cell * steps * cell,
                    (x + w - steps * cell, y + h - steps * cell, x + w, y + h),
                ));
            }
            console_log!("MAP ({cells_x_count}, {cells_y_count}): {cells:?}");
        }
    }
    pub fn box_height(&self) -> i32 {
        match self {
            Self::Rectangle(figure) => figure.box_height(),
            Self::Path(figure) => figure.box_height(),
        }
    }
    pub fn box_width(&self) -> i32 {
        match self {
            Self::Rectangle(figure) => figure.box_width(),
            Self::Path(figure) => figure.box_width(),
        }
    }
    pub fn set_box_height(&mut self, h: i32) {
        match self {
            Self::Rectangle(figure) => figure.set_box_height(h),
            Self::Path(_) => { /* Ignore */ }
        }
    }
    pub fn set_coors(&mut self, x: Option<i32>, y: Option<i32>) {
        match self {
            Self::Rectangle(figure) => figure.set_coors(x, y),
            Self::Path(_) => { /* Ignore */ }
        }
    }

    pub fn get_coors(&self) -> (i32, i32) {
        match self {
            Self::Rectangle(figure) => figure.get_coors(),
            Self::Path(_) => {
                /* Ignore */
                (0, 0)
            }
        }
    }
    pub fn render(&self, context: &mut web_sys::CanvasRenderingContext2d, relative: &Relative) {
        match self {
            Self::Rectangle(figure) => figure.render(context, relative),
            Self::Path(figure) => figure.render(context, relative),
        }
    }

    pub fn relative(&self) -> Relative {
        match self {
            Self::Rectangle(figure) => figure.relative(),
            Self::Path(_) => {
                todo!("Implement render for path")
            }
        }
    }
}
