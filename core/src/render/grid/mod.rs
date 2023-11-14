use std::collections::HashMap;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::console_log;

use crate::{
    error::E,
    render::{Form, Relative},
};

pub const CELL: u32 = 25;
pub const SPACE_IN_VERTICAL: u32 = 1;
pub const SPACE_IN_HORIZONT: u32 = 3;

#[derive(Debug)]
pub enum Layout<'a> {
    // forms in center and forms on left and right sides
    WithFormsBySides((Vec<&'a Form>, Vec<&'a Form>, Vec<&'a Form>)),
    // from other grids into row
    GridsRow(&'a [Grid]),
}

#[derive(Debug)]
pub struct Grid {
    // Total grid size
    pub size: (u32, u32),
    // Cells map
    pub map: HashMap<(u32, u32), usize>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            size: (0, 0),
            map: HashMap::new(),
        }
    }

    pub fn from(layout: Layout<'_>) -> Result<Self, E> {
        Ok(match layout {
            Layout::WithFormsBySides((left, center, right)) => {
                with_forms_by_sides(left, center, right)?
            }
            Layout::GridsRow(grids) => from_grids_into_row(grids),
        })
    }

    pub fn relative(&self, target: usize) -> Relative {
        if let Some((x, y)) = self
            .map
            .iter()
            .filter_map(|((x, y), id)| if id == &target { Some((x, y)) } else { None })
            .min()
        {
            console_log!("{x} - {y}");
            Relative::new((x * CELL) as i32, (y * CELL) as i32)
        } else {
            Relative::new(0, 0)
        }
    }

    pub fn draw(
        &self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
    ) -> Result<(), E> {
        context.set_stroke_style(&JsValue::from_str("rgb(150, 150, 150)"));
        context.begin_path();
        let w = (self.size.0 * CELL) as i32;
        let h = (self.size.1 * CELL) as i32;
        for x in 0..=self.size.0 {
            context.move_to(relative.x((x * CELL) as i32) as f64, relative.y(0) as f64);
            context.line_to(relative.x((x * CELL) as i32) as f64, relative.y(h) as f64);
        }
        for y in 0..=self.size.1 {
            context.move_to(relative.x(0) as f64, relative.y((y * CELL) as i32) as f64);
            context.line_to(relative.x(w) as f64, relative.y((y * CELL) as i32) as f64);
        }
        context.stroke();
        Ok(())
    }
}

fn get_sizes(forms: Vec<&Form>) -> Result<Vec<(usize, (u32, u32))>, E> {
    let mut data = vec![];
    for form in forms {
        data.push((form.id(), form.cells()?));
    }
    Ok(data)
}

fn with_forms_by_sides(left: Vec<&Form>, center: Vec<&Form>, right: Vec<&Form>) -> Result<Grid, E> {
    let on_left = get_sizes(left)?;
    let on_center = get_sizes(center)?;
    let on_right = get_sizes(right)?;
    console_log!("on left: {on_left:?}");
    console_log!("on center: {on_center:?}");
    console_log!("on right: {on_right:?}");
    let mut map: HashMap<(u32, u32), usize> = HashMap::new();
    let mut size: (u32, u32) = (0, 0);
    // Put left side
    let mut cursor_by_y: u32 = 0;
    on_left.iter().for_each(|(id, (w, h))| {
        for x in 0..*w {
            for y in 0..*h {
                map.insert((x, y + cursor_by_y), *id);
            }
        }
        cursor_by_y += h + SPACE_IN_VERTICAL;
    });
    if cursor_by_y > 0 {
        size.1 = cursor_by_y - SPACE_IN_VERTICAL;
    }
    // Put center
    let mut cursor_by_x = *on_left.iter().map(|(_, (w, _))| w).max().unwrap_or(&0);
    size.0 = cursor_by_x;
    if cursor_by_x > 0 {
        cursor_by_x += SPACE_IN_HORIZONT;
    }
    cursor_by_y = 0;
    on_center.iter().for_each(|(id, (w, h))| {
        for x in 0..*w {
            for y in 0..*h {
                map.insert((x + cursor_by_x, y + cursor_by_y), *id);
            }
        }
        cursor_by_y += h + SPACE_IN_VERTICAL;
    });
    if cursor_by_y > 0 && cursor_by_y - SPACE_IN_VERTICAL > size.1 {
        size.1 = cursor_by_y - SPACE_IN_VERTICAL;
    }
    // Put right side
    let center_width = *on_center.iter().map(|(_, (w, _))| w).max().unwrap_or(&0);
    size.0 += center_width;
    if center_width > 0 {
        cursor_by_x += center_width + SPACE_IN_HORIZONT;
    }
    cursor_by_y = 0;
    on_right.iter().for_each(|(id, (w, h))| {
        for x in 0..*w {
            for y in 0..*h {
                map.insert((x + cursor_by_x, y + cursor_by_y), *id);
            }
        }
        cursor_by_y += h + SPACE_IN_VERTICAL;
    });
    if cursor_by_y > 0 && cursor_by_y - SPACE_IN_VERTICAL > size.1 {
        size.1 = cursor_by_y - SPACE_IN_VERTICAL;
    }
    size.0 += *on_right.iter().map(|(_, (w, _))| w).max().unwrap_or(&0);
    let rows = if on_right.is_empty() { 0 } else { 1 }
        + if on_center.is_empty() { 0 } else { 1 }
        + if on_left.is_empty() { 0 } else { 1 };
    size.0 += if rows == 0 {
        0
    } else {
        (rows - 1) * SPACE_IN_HORIZONT
    };
    Ok(Grid { size, map })
}

fn from_grids_into_row(grids: &[Grid]) -> Grid {
    let mut map: HashMap<(u32, u32), usize> = HashMap::new();
    let mut size: (u32, u32) = (0, 0);
    grids.iter().for_each(|grid| {
        grid.map.iter().for_each(|((x, y), id)| {
            map.insert((x + size.0, *y), *id);
        });
        size.0 += grid.size.0
            + if grid.size.0 > 0 {
                SPACE_IN_HORIZONT
            } else {
                0
            };
        if grid.size.1 > size.1 {
            size.1 = grid.size.1;
        }
    });
    if size.0 > 0 {
        size.0 -= SPACE_IN_HORIZONT;
    }
    Grid { size, map }
}
