use std::collections::HashMap;
use wasm_bindgen::JsValue;

use crate::{
    error::E,
    render::{elements, Form, Relative},
};

pub const CELL: u32 = 25;
pub const SPACE_IN_VERTICAL: u32 = 1;
pub const SPACE_IN_HORIZONT: u32 = 3;

#[derive(Debug)]
pub enum Layout<'a> {
    // Forms in center and forms on left and right sides
    WithFormsBySides((Vec<&'a Form>, Vec<&'a Form>, Vec<&'a Form>)),
    // From other grids into row
    _GridsRow(&'a [Grid]),
    // Order grids into one box: Grid[], offset_by_each_side
    GridsBox(&'a mut [Grid], u32),
}

#[derive(Debug, Clone)]
pub struct Grid {
    // Offset from each side
    pub offset: u32,
    // Total grid size
    pub size: (u32, u32),
    // Cells map <EntityID, Occupied area <(x, y, x1, y1)>>
    pub map: HashMap<usize, (u32, u32, u32, u32)>,
}

impl Grid {
    pub fn new(offset: u32) -> Self {
        Grid {
            offset,
            size: (offset * 2, offset * 2),
            map: HashMap::new(),
        }
    }

    pub fn from(layout: Layout<'_>) -> Result<Self, E> {
        Ok(match layout {
            Layout::WithFormsBySides((left, center, right)) => {
                with_forms_by_sides(left, center, right)?
            }
            Layout::_GridsRow(grids) => from_grids_into_row(grids),
            Layout::GridsBox(grids, offset) => from_grids_into_box(grids, offset),
        })
    }

    pub fn set_min_height(&mut self, height_px: u32) -> u32 {
        if self.size.1 * CELL < height_px {
            self.size.1 = (height_px as f64 / CELL as f64).ceil() as u32;
        }
        self.size.1 * CELL
    }

    pub fn insert_self(&mut self, id: usize) {
        self.map.insert(id, (0, 0, self.size.0, self.size.1));
    }

    pub fn relative(&self, target: usize) -> Relative {
        if let Some((x, y)) =
            self.map.iter().find_map(
                |(id, (x, y, _, _))| {
                    if id == &target {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        {
            Relative::new((x * CELL) as i32, (y * CELL) as i32, None)
        } else {
            Relative::new(0, 0, None)
        }
    }

    pub fn in_area(&self, area_px: (u32, u32, u32, u32), zoom: f64) -> Vec<usize> {
        let cell = (CELL as f64 * zoom) as u32;
        let (ax, ay, ax1, ay1) = area_px;
        let (mut ax, mut ay, ax1, ay1) = (ax / cell, ay / cell, (ax1 / cell) + 1, (ay1 / cell) + 1);
        ax = ax.saturating_sub(1);
        ay = ay.saturating_sub(1);
        self.map
            .iter()
            .filter_map(|(id, block)| {
                if elements::is_area_cross(&(ax, ay, ax1, ay1), block) {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect()
    }

    fn is_block_free(&self, target: (u32, u32, u32, u32)) -> bool {
        let (mut x, mut y, mut x1, mut y1) = target;
        // Check space
        if self.size.0 - self.offset < x1 || self.size.1 - self.offset < y1 {
            return false;
        }
        // Extend box to consider necessary spaces
        x = if x > SPACE_IN_HORIZONT - 1 {
            x - (SPACE_IN_HORIZONT - 1)
        } else {
            0
        };
        y = if y > SPACE_IN_VERTICAL - 1 {
            y - (SPACE_IN_VERTICAL - 1)
        } else {
            0
        };
        x1 += SPACE_IN_HORIZONT;
        y1 += SPACE_IN_VERTICAL;
        let extd_target = (x, y, x1, y1);
        // Check crossing
        for (_, (ax, ay, ax1, ay1)) in self.map.iter() {
            if elements::is_area_cross(&extd_target, &(*ax, *ay, *ax1, *ay1)) {
                return false;
            }
        }
        true
    }

    fn is_point_free(&self, point: &(u32, u32)) -> bool {
        if point.0 < self.offset
            || point.0 > self.size.0 + self.offset * 2
            || point.1 < self.offset
            || point.1 > self.size.1 + self.offset * 2
        {
            return false;
        }
        for (_, (ax, ay, ax1, ay1)) in self.map.iter() {
            if elements::is_point_in(point, &(*ax, *ay, *ax1, *ay1)) {
                return false;
            }
        }
        true
    }

    pub fn cut_unused_space(&mut self) {
        let max_x = self
            .map
            .values()
            .map(|(_, _, x1, _)| x1)
            .max()
            .unwrap_or(&0)
            + self.offset;
        let max_y = self
            .map
            .values()
            .map(|(_, _, _, y1)| y1)
            .max()
            .unwrap_or(&0)
            + self.offset;
        self.size = (
            [max_x, self.size.0].iter().min().copied().unwrap_or(0),
            [max_y, self.size.1].iter().min().copied().unwrap_or(0),
        );
    }

    pub fn insert(&mut self, grid: &Grid) {
        // TODO: conside if size == (0,0)
        // Looking for point to insert grid
        let mut point: Option<(u32, u32)> = None;
        while point.is_none() {
            for x in 0..self.size.0 {
                for y in 0..self.size.1 {
                    if !self.is_point_free(&(x, y)) {
                        continue;
                    }
                    if self.is_block_free((x, y, x + grid.size.0, y + grid.size.1)) {
                        point = Some((x, y));
                        break;
                    }
                }
                if point.is_some() {
                    break;
                }
            }
            if point.is_none() {
                // Point isn't found. Grid doesn't have enought space. Increase space
                self.size.0 += 1;
                self.size.1 += 1;
            }
        }
        // Merge grid
        if let Some((p_x, p_y)) = point {
            grid.map.iter().for_each(|(id, (x, y, x1, y1))| {
                self.map.insert(*id, (x + p_x, y + p_y, x1 + p_x, y1 + p_y));
            });
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

    pub fn get_size_px(&self) -> (u32, u32) {
        (self.size.0 * CELL, self.size.1 * CELL)
    }
}

pub type FormSize = (usize, (u32, u32));

fn get_sizes(forms: Vec<&Form>) -> Result<Vec<FormSize>, E> {
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
    let mut map: HashMap<usize, (u32, u32, u32, u32)> = HashMap::new();
    let mut size: (u32, u32) = (0, 0);
    // Put left side
    let mut cursor_by_y: u32 = 0;
    on_left.iter().for_each(|(id, (w, h))| {
        map.insert(*id, (0, cursor_by_y, *w, cursor_by_y + h));
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
        map.insert(
            *id,
            (cursor_by_x, cursor_by_y, cursor_by_x + w, cursor_by_y + h),
        );
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
        map.insert(
            *id,
            (cursor_by_x, cursor_by_y, cursor_by_x + w, cursor_by_y + h),
        );
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
    Ok(Grid {
        offset: 0,
        size,
        map,
    })
}

fn from_grids_into_row(grids: &[Grid]) -> Grid {
    let mut map: HashMap<usize, (u32, u32, u32, u32)> = HashMap::new();
    let mut size: (u32, u32) = (0, 0);
    grids.iter().for_each(|grid| {
        grid.map.iter().for_each(|(id, (x, y, x1, y1))| {
            map.insert(*id, (x + size.0, *y, x1 + size.0, *y1));
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
    Grid {
        offset: 0,
        size,
        map,
    }
}

fn from_grids_into_box(grids: &mut [Grid], offset: u32) -> Grid {
    // Sort from biggest to smallest
    grids.sort_by(|a, b| (b.size.0 * b.size.1).cmp(&(a.size.0 * a.size.1)));
    let mut packed = Grid::new(offset);
    // Merge grids
    grids.iter().for_each(|grid| {
        packed.insert(grid);
    });
    packed.cut_unused_space();
    packed
}
