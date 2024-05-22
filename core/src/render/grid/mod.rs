use crate::{
    error::E,
    render::{elements, options::GridOptions, Form, Ratio, Relative},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::console_log;

pub const CELL: u32 = 25;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ElementType {
    Unknown,
    Component,
    Composition,
    Connection,
    Port,
    Element,
}
pub type ElementCoors = (String, ElementType, (i32, i32, i32, i32));

pub fn as_u32(n: i32) -> u32 {
    (if n < 0 { 0 } else { n }) as u32
}

fn as_cells(px: u32, cell: f64) -> u32 {
    (px as f64 / cell).ceil() as u32
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub options: GridOptions,
    // Total grid size (in cells)
    pub size: (u32, u32),
    // Cells map <EntityID, Occupied area <(x, y, x1, y1)>>
    pub map: HashMap<String, ElementCoor>,
    pub id: Option<usize>,
    pub cell: u32,
    pub ratio: Ratio,
}

impl Grid {
    pub fn new(options: &GridOptions, ratio: Ratio) -> Self {
        Grid {
            options: options.clone(),
            size: (options.hpadding * 2, options.vpadding * 2),
            map: HashMap::new(),
            id: None,
            cell: ratio.get(CELL),
            ratio,
        }
    }

    pub fn forms_as_pair(
        a: Vec<&Form>,
        b: Vec<&Form>,
        options: &GridOptions,
        ratio: Ratio,
    ) -> Result<Self, E> {
        let on_left = get_sizes(a)?;
        let on_right = get_sizes(b)?;
        let mut map: HashMap<String, ElementCoor> = HashMap::new();
        let mut cursor_by_y: u32 = 0;
        let mut size: (u32, u32) = (0, 0);
        if !on_left.is_empty() {
            on_left.iter().for_each(|(id, ty, (w, h))| {
                map.insert(
                    id.to_string(),
                    (ty.clone(), (0, cursor_by_y, w - 1, cursor_by_y + (h - 1))),
                );
                cursor_by_y += h + options.cells_space_vertical;
                if size.0 < *w {
                    size.0 = *w;
                    if *w > 3 {
                        console_log!("OOPPS: {w}");
                    }
                }
            });
            size.1 = cursor_by_y - options.cells_space_vertical;
            if !on_right.is_empty() {
                size.0 += options.cells_space_horizontal;
            }
        };
        if !on_right.is_empty() {
            cursor_by_y = 0;
            let mut max_w = 0;
            on_right.iter().for_each(|(id, ty, (w, h))| {
                map.insert(
                    id.to_string(),
                    (
                        ty.clone(),
                        (size.0, cursor_by_y, size.0 + (w - 1), cursor_by_y + (h - 1)),
                    ),
                );
                cursor_by_y += h + options.cells_space_vertical;
                if max_w < *w {
                    max_w = *w;
                }
            });
            size.1 = if cursor_by_y - options.cells_space_vertical > size.1 {
                cursor_by_y - options.cells_space_vertical
            } else {
                size.1
            };
            size.0 += max_w;
        }
        let mut options = options.clone();
        options.vpadding = 0;
        options.hpadding = 0;
        Ok(Grid {
            options,
            size,
            map,
            id: None,
            cell: ratio.get(CELL),
            ratio,
        })
    }

    pub fn set_min_height(&mut self, height_px: u32) -> u32 {
        if self.size.1 * self.cell < height_px {
            self.size.1 = (height_px as f64 / self.cell as f64).ceil() as u32;
        }
        self.size.1 * self.cell
    }

    pub fn insert_self(&mut self, id: usize, ty: ElementType) {
        self.id = Some(id);
        self.map.insert(
            id.to_string(),
            (
                ty,
                (
                    0,
                    0,
                    as_u32(self.size.0 as i32 - 1),
                    as_u32(self.size.1 as i32 - 1),
                ),
            ),
        );
    }

    pub fn relative(&self, target: usize) -> Relative {
        if let Some((x, y)) = self.map.iter().find_map(|(id, (_, (x, y, _, _)))| {
            if id == &target.to_string() {
                Some((x, y))
            } else {
                None
            }
        }) {
            Relative::new((x * self.cell) as i32, (y * self.cell) as i32, None)
        } else {
            Relative::new(0, 0, None)
        }
    }

    pub fn get_coors_by_ids(
        &self,
        ids: &[usize],
        relative: &Relative,
        ratio: &Ratio,
    ) -> Vec<ElementCoors> {
        let mut found: Vec<ElementCoors> = Vec::new();
        self.map.iter().for_each(|(id, (ty, area))| {
            if let Ok(id) = id.parse::<usize>() {
                if !ids.contains(&id) {
                    return;
                }
                found.push((
                    id.to_string(),
                    ty.clone(),
                    (
                        ratio.invert(relative.x((area.0 * self.cell) as i32)),
                        ratio.invert(relative.y((area.1 * self.cell) as i32)),
                        ratio.invert(relative.x(((area.2 + 1) * self.cell) as i32)),
                        ratio.invert(relative.y(((area.3 + 1) * self.cell) as i32)),
                    ),
                ));
            }
        });
        found
    }

    pub fn point(
        &self,
        position: (i32, i32),
        around: i32,
        relative: &Relative,
    ) -> Vec<ElementCoors> {
        let (x, y) = (position.0, position.1);
        self.in_area(
            (
                as_u32(x - around),
                as_u32(y - around),
                as_u32(x + around * 2),
                as_u32(y + around * 2),
            ),
            relative.get_zoom(),
            0,
        )
    }

    pub fn viewport(&self, position: (i32, i32), size: (u32, u32), zoom: f64) -> Vec<ElementCoors> {
        let (x, y) = (
            (position.0 as f64 * zoom).ceil() as i32,
            (position.1 as f64 * zoom).ceil() as i32,
        );
        let (w, h) = size;
        let vx = if x > 0 { 0 } else { -x };
        let vy = if y > 0 { 0 } else { -y };
        let vx1 = w as i32 - x;
        let vy1 = h as i32 - y;
        self.in_area((as_u32(vx), as_u32(vy), as_u32(vx1), as_u32(vy1)), zoom, 1)
    }

    pub fn in_area(
        &self,
        area_px: (u32, u32, u32, u32),
        zoom: f64,
        prolongation: u32,
    ) -> Vec<ElementCoors> {
        let cell = self.cell as f64 * zoom;
        let (mut ax, mut ay, mut ax1, mut ay1) = (
            as_cells(area_px.0, cell),
            as_cells(area_px.1, cell),
            as_cells(area_px.2, cell) + prolongation,
            as_cells(area_px.3, cell) + prolongation,
        );
        ax = ax.saturating_sub(1);
        ay = ay.saturating_sub(1);
        ax1 = ax1.saturating_sub(1);
        ay1 = ay1.saturating_sub(1);
        let targets = self
            .map
            .iter()
            .filter_map(|(id, (ty, block))| {
                if elements::is_area_cross(&(ax, ay, ax1, ay1), block) {
                    Some((
                        id.clone(),
                        ty.clone(),
                        (
                            (block.0 as f64 * cell) as i32,
                            (block.1 as f64 * cell) as i32,
                            ((block.2 + 1) as f64 * cell) as i32,
                            ((block.3 + 1) as f64 * cell) as i32,
                        ),
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<ElementCoors>>();
        targets
    }

    fn is_block_free(&self, target: (u32, u32, u32, u32)) -> bool {
        let (mut x, mut y, mut x1, mut y1) = target;
        // Check space
        if self.size.0 - self.options.hpadding < x1 || self.size.1 - self.options.vpadding < y1 {
            return false;
        }
        // Extend box to consider necessary spaces
        x = if x > self.options.cells_space_horizontal {
            x - self.options.cells_space_horizontal
        } else {
            0
        };
        y = if y > self.options.cells_space_vertical {
            y - self.options.cells_space_vertical
        } else {
            0
        };
        x1 += self.options.cells_space_horizontal;
        y1 += self.options.cells_space_vertical;
        let extd_target = (x, y, x1, y1);
        // Check crossing
        for (_, (_, (ax, ay, ax1, ay1))) in self.map.iter() {
            if elements::is_area_cross(&extd_target, &(*ax, *ay, *ax1, *ay1)) {
                return false;
            }
        }
        true
    }

    fn is_point_free(&self, point: &(u32, u32)) -> bool {
        if point.0 < self.options.hpadding
            || point.0 > self.size.0 + self.options.hpadding * 2
            || point.1 < self.options.vpadding
            || point.1 > self.size.1 + self.options.vpadding * 2
        {
            return false;
        }
        let self_id = self.id.unwrap_or(0).to_string();
        for (id, (_, (ax, ay, ax1, ay1))) in self.map.iter() {
            if &self_id != id
                && elements::is_point_in(
                    &(point.0 as i32, point.1 as i32),
                    &(*ax as i32, *ay as i32, *ax1 as i32, *ay1 as i32),
                )
            {
                return false;
            }
        }
        true
    }

    pub fn cut_unused_space(&mut self) {
        let max_x = self
            .map
            .values()
            .map(|(_, (_, _, x1, _))| x1)
            .max()
            .unwrap_or(&0)
            + self.options.hpadding;
        let max_y = self
            .map
            .values()
            .map(|(_, (_, _, _, y1))| y1)
            .max()
            .unwrap_or(&0)
            + self.options.vpadding;
        self.size = (
            [max_x + 1, self.size.0].iter().min().copied().unwrap_or(0),
            [max_y + 1, self.size.1].iter().min().copied().unwrap_or(0),
        );
    }

    pub fn insert(&mut self, grid: &Grid) {
        // TODO: conside if size == (0,0)
        // Looking for point to insert grid
        let mut point: Option<(u32, u32)> = None;
        self.size = (
            elements::max(&[self.size.0, grid.size.0], self.options.hpadding * 2),
            elements::max(&[self.size.1, grid.size.1], self.options.vpadding * 2),
        );
        while point.is_none() {
            for y in 0..self.size.1 {
                for x in 0..self.size.0 {
                    if !self.is_point_free(&(x, y)) {
                        continue;
                    }
                    if self.is_block_free((x, y, x + grid.size.0 - 1, y + grid.size.1 - 1)) {
                        point = Some((x, y));
                        break;
                    }
                }
                if point.is_some() {
                    break;
                }
            }
            if point.is_none() {
                if self.size.0.lt(&self.size.1) {
                    self.size.1 += 1;
                    self.size.0 += (self.size.1 as f64 / self.size.0 as f64).ceil() as u32;
                } else if self.size.1.lt(&self.size.0) {
                    self.size.0 += 1;
                    self.size.1 += (self.size.0 as f64 / self.size.1 as f64).ceil() as u32;
                } else {
                    self.size.0 += 1;
                    self.size.1 += 1;
                }
            }
        }
        // Merge grid
        if let Some((p_x, p_y)) = point {
            grid.map.iter().for_each(|(id, (ty, (x, y, x1, y1)))| {
                self.map.insert(
                    id.clone(),
                    (ty.clone(), (x + p_x, y + p_y, x1 + p_x, y1 + p_y)),
                );
            });
        }
        // Remove unused space
        self.cut_unused_space();
    }

    pub fn as_px(&self, cells: u32) -> i32 {
        (self.cell * cells) as i32
    }

    pub fn draw(
        &self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
    ) -> Result<(), E> {
        if !self.options.visible {
            return Ok(());
        }
        context.set_stroke_style(&JsValue::from_str("rgb(150, 150, 150)"));
        context.begin_path();
        let w = (self.size.0 * self.cell) as i32;
        let h = (self.size.1 * self.cell) as i32;
        for x in 0..=self.size.0 {
            context.move_to(
                relative.x((x * self.cell) as i32) as f64,
                relative.y(0) as f64,
            );
            context.line_to(
                relative.x((x * self.cell) as i32) as f64,
                relative.y(h) as f64,
            );
        }
        for y in 0..=self.size.1 {
            context.move_to(
                relative.x(0) as f64,
                relative.y((y * self.cell) as i32) as f64,
            );
            context.line_to(
                relative.x(w) as f64,
                relative.y((y * self.cell) as i32) as f64,
            );
        }
        context.stroke();
        Ok(())
    }

    pub fn get_size_px(&self) -> (u32, u32) {
        (self.size.0 * self.cell, self.size.1 * self.cell)
    }
    pub fn get_size_invert_px(&self) -> (u32, u32) {
        (
            self.ratio.invert(self.size.0 * self.cell),
            self.ratio.invert(self.size.1 * self.cell),
        )
    }
}

pub type FormSize = (String, ElementType, (u32, u32));
pub type ElementCoor = (ElementType, (u32, u32, u32, u32));
fn get_sizes(forms: Vec<&Form>) -> Result<Vec<FormSize>, E> {
    let mut data = Vec::new();
    for form in forms {
        data.push((form.id(), form.get_el_ty().clone(), form.cells()?));
    }
    Ok(data)
}
