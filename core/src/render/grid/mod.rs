use crate::{
    error::E,
    render::{elements, Form, Relative},
};
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::console_log;

pub const CELL: u32 = 25;
pub const SPACE_IN_VERTICAL: u32 = 3;
pub const SPACE_IN_HORIZONT: u32 = 3;

pub type ElementCoors = (String, (u32, u32, u32, u32));

#[derive(Debug)]
pub enum Layout<'a> {
    // Put form near
    Pair(Vec<&'a Form>, Vec<&'a Form>),
    // Forms in center and forms on left and right sides
    _WithFormsBySides((Vec<&'a Form>, Vec<&'a Form>, Vec<&'a Form>)),
    // From other grids into row
    _GridsRow(&'a [Grid]),
    // Order grids into one box: Grid[], offset_by_each_side
    _GridsBox(&'a mut [Grid], u32),
}

fn as_u32(n: i32) -> u32 {
    (if n < 0 { 0 } else { n }) as u32
}

fn as_cells(px: u32, cell: f64) -> u32 {
    (px as f64 / cell).ceil() as u32
}

fn as_cells_round(px: u32, cell: f64) -> u32 {
    (px as f64 / cell).round() as u32
}

#[derive(Debug)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub left: bool,
}

impl Cell {
    pub fn new(x: u32, y: u32, grid: &Grid) -> Result<Self, E> {
        let left = (
            as_cells_round(x - CELL / 2, CELL as f64),
            as_cells_round(y, CELL as f64),
        );
        if grid.is_map_point_free(&left) {
            return Ok(Self {
                x: left.0,
                y: left.1,
                left: true,
            });
        }
        let right = (
            as_cells_round(x + CELL / 2, CELL as f64),
            as_cells_round(y, CELL as f64),
        );
        if grid.is_map_point_free(&right) {
            return Ok(Self {
                x: right.0,
                y: right.1,
                left: false,
            });
        }
        Err(E::Other("Fail to detect start point".to_string()))
    }
    pub fn normalize(
        a: &(u32, u32),
        b: &(u32, u32),
        grid: &Grid,
        points: &mut Vec<(u32, u32)>,
    ) -> Result<(), E> {
        fn get_busy_in_horizont(
            a: &(u32, u32),
            b: &(u32, u32),
            grid: &Grid,
        ) -> Option<((u32, u32), (u32, u32))> {
            //TODO: check a.0 < b.0
            let mut from: Option<(u32, u32)> = None;
            for x in a.0..=b.0 {
                let free = grid.is_map_point_free(&(x, a.1));
                if !free && from.is_none() {
                    from = Some((x, a.1));
                    continue;
                }
                if let (Some(from), true) = (from, free) {
                    return Some((from, (x - 1, a.1)));
                }
            }
            from.map(|from| (from, *b))
        }
        fn get_busy_in_vertical(
            a: &(u32, u32),
            b: &(u32, u32),
            grid: &Grid,
        ) -> Option<((u32, u32), (u32, u32))> {
            //TODO: check a.1 < b.1
            let mut from: Option<(u32, u32)> = None;
            for y in a.1..=b.1 {
                let free = grid.is_map_point_free(&(a.0, y));
                if !free && from.is_none() {
                    from = Some((a.0, y));
                    continue;
                }
                if let (Some(from), true) = (from, free) {
                    return Some((from, (a.0, y - 1)));
                }
            }
            from.map(|from| (from, *b))
        }
        fn get_closed_free_in_horizont(
            a: &(u32, u32),
            b: &(u32, u32),
            grid: &Grid,
        ) -> Option<((u32, u32), (u32, u32))> {
            let mut above: Option<u32> = None;
            let mut bellow: Option<u32> = None;
            for y in (0..a.1).rev() {
                if grid.is_map_block_free((a.0, y, b.0, y)) {
                    above = Some(y);
                    break;
                }
            }

            for y in a.1..grid.size.1 {
                if grid.is_map_block_free((a.0, y, b.0, y)) {
                    bellow = Some(y);
                    break;
                }
            }
            if let (Some(above), Some(bellow)) = (above, bellow) {
                Some(if a.1 - above < bellow - a.1 {
                    ((a.0, above), (b.0, above))
                } else {
                    ((a.0, bellow), (b.0, bellow))
                })
            } else if let Some(above) = above {
                Some(((a.0, above), (b.0, above)))
            } else {
                bellow.map(|bellow| ((a.0, bellow), (b.0, bellow)))
            }
        }
        fn get_closed_free_in_vertical(
            a: &(u32, u32),
            b: &(u32, u32),
            grid: &Grid,
        ) -> Option<((u32, u32), (u32, u32))> {
            let mut left: Option<u32> = None;
            let mut right: Option<u32> = None;
            for x in (a.0..=0).rev() {
                if grid.is_map_block_free((x, a.1, x, b.1)) {
                    left = Some(x);
                    break;
                }
            }
            for x in a.0..grid.size.0 {
                if grid.is_map_block_free((x, a.1, x, b.1)) {
                    right = Some(x);
                    break;
                }
            }
            if let (Some(left), Some(right)) = (left, right) {
                Some(if a.1 - left < right - a.1 {
                    ((left, a.1), (left, b.1))
                } else {
                    ((right, a.1), (right, b.1))
                })
            } else if let Some(left) = left {
                Some(((left, a.1), (left, b.1)))
            } else {
                right.map(|right| ((right, a.1), (right, b.1)))
            }
        }
        fn push_unique(points: &mut Vec<(u32, u32)>, point: (u32, u32)) {
            let add = if let Some(last) = points.last() {
                last != &point
            } else {
                true
            };
            if add {
                points.push(point);
            }
        }
        let mut current = *a;
        let mut checking = (*a, *b);
        push_unique(points, current);
        let mut iteration = 0;
        if a.1 == b.1 {
            // Horizontal
            loop {
                iteration += 1;
                if iteration > 10 {
                    break;
                }
                if let Some(busy) = get_busy_in_horizont(&checking.0, &checking.1, grid) {
                    let free = get_closed_free_in_horizont(
                        &(busy.0 .0 - 1, busy.0 .1),
                        &(busy.1 .0 + 1, busy.1 .1),
                        grid,
                    )
                    .ok_or(E::Other("Fail find free in horizont".to_string()))?;
                    push_unique(points, (free.0 .0, current.1));
                    push_unique(points, (free.0 .0, free.0 .1));
                    push_unique(points, (free.1 .0, free.1 .1));
                    push_unique(points, (free.1 .0, current.1));
                    current = (free.1 .0, current.1);
                    checking = (current, *b);
                } else {
                    push_unique(points, *b);
                    break;
                }
            }
            // normalize_horizont(points, grid);
        } else {
            // Vertical
            loop {
                iteration += 1;
                if iteration > 10 {
                    break;
                }
                if let Some(busy) = get_busy_in_vertical(&checking.0, &checking.1, grid) {
                    let free = get_closed_free_in_vertical(
                        &(busy.0 .0, busy.0 .1 - 1),
                        &(busy.1 .0, busy.1 .1 + 1),
                        grid,
                    )
                    .ok_or(E::Other("Fail find free in vertical".to_string()))?;
                    push_unique(points, (current.0, free.0 .1));
                    push_unique(points, (free.0 .0, free.0 .1));
                    push_unique(points, (free.1 .0, free.1 .1));
                    push_unique(points, (current.0, free.1 .1));
                    current = (current.0, free.1 .1);
                    checking = (current, *b);
                } else {
                    push_unique(points, *b);
                    break;
                }
            }
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct Grid {
    // Offset from each side
    pub offset: u32,
    // Total grid size
    pub size: (u32, u32),
    // Cells map <EntityID, Occupied area <(x, y, x1, y1)>>
    pub map: HashMap<String, (u32, u32, u32, u32)>,
    pub id: Option<usize>,
}

impl Grid {
    pub fn new(offset: u32) -> Self {
        Grid {
            offset,
            size: (offset * 2, offset * 2),
            map: HashMap::new(),
            id: None,
        }
    }

    pub fn from(layout: Layout<'_>) -> Result<Self, E> {
        Ok(match layout {
            Layout::_WithFormsBySides((left, center, right)) => {
                with_forms_by_sides(left, center, right)?
            }
            Layout::Pair(a, b) => forms_as_pair(a, b)?,
            Layout::_GridsRow(grids) => from_grids_into_row(grids),
            Layout::_GridsBox(grids, offset) => from_grids_into_box(grids, offset),
        })
    }

    pub fn set_min_height(&mut self, height_px: u32) -> u32 {
        if self.size.1 * CELL < height_px {
            self.size.1 = (height_px as f64 / CELL as f64).ceil() as u32;
        }
        self.size.1 * CELL
    }

    pub fn insert_self(&mut self, id: usize) {
        self.id = Some(id);
        self.map.insert(
            id.to_string(),
            (
                0,
                0,
                as_u32(self.size.0 as i32 - 1),
                as_u32(self.size.1 as i32 - 1),
            ),
        );
    }

    pub fn relative(&self, target: usize) -> Relative {
        if let Some((x, y)) = self.map.iter().find_map(|(id, (x, y, _, _))| {
            if id == &target.to_string() {
                Some((x, y))
            } else {
                None
            }
        }) {
            Relative::new((x * CELL) as i32, (y * CELL) as i32, None)
        } else {
            Relative::new(0, 0, None)
        }
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
        let cell = CELL as f64 * zoom;
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
            .filter_map(|(id, block)| {
                if elements::is_area_cross(&(ax, ay, ax1, ay1), block) {
                    Some((
                        id.clone(),
                        (
                            (block.0 as f64 * cell) as u32,
                            (block.1 as f64 * cell) as u32,
                            ((block.2 + 1) as f64 * cell) as u32,
                            ((block.3 + 1) as f64 * cell) as u32,
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
        if self.size.0 - self.offset < x1 || self.size.1 - self.offset < y1 {
            return false;
        }
        // Extend box to consider necessary spaces
        x = if x > SPACE_IN_HORIZONT {
            x - SPACE_IN_HORIZONT
        } else {
            0
        };
        y = if y > SPACE_IN_VERTICAL {
            y - SPACE_IN_VERTICAL
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

    fn is_map_block_free(&self, target: (u32, u32, u32, u32)) -> bool {
        // Check space
        if self.size.0 - 1 < target.2 || self.size.1 - 1 < target.3 {
            return false;
        }
        let self_id = self.id.unwrap_or(0).to_string();
        // Check crossing
        for (id, area) in self.map.iter() {
            if &self_id != id && elements::is_area_cross(&target, area) {
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
        let self_id = self.id.unwrap_or(0).to_string();
        for (id, (ax, ay, ax1, ay1)) in self.map.iter() {
            if &self_id != id
                && elements::is_point_in(
                    point,
                    &(as_u32(*ax as i32), as_u32(*ay as i32), *ax1, *ay1),
                )
            {
                return false;
            }
        }
        true
    }

    fn is_map_point_free(&self, point: &(u32, u32)) -> bool {
        let self_id = self.id.unwrap_or(0).to_string();
        for (id, (ax, ay, ax1, ay1)) in self.map.iter() {
            if &self_id != id
                && elements::is_point_in(
                    point,
                    &(as_u32(*ax as i32), as_u32(*ay as i32), *ax1, *ay1),
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
            [max_x + 1, self.size.0].iter().min().copied().unwrap_or(0),
            [max_y + 1, self.size.1].iter().min().copied().unwrap_or(0),
        );
    }

    pub fn insert(&mut self, grid: &Grid) {
        // TODO: conside if size == (0,0)
        // Looking for point to insert grid
        let mut point: Option<(u32, u32)> = None;
        self.size = (
            elements::max(&[self.size.0, grid.size.0], self.offset * 2),
            elements::max(&[self.size.1, grid.size.1], self.offset * 2),
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
            // for x in 0..self.size.0 {
            //     for y in 0..self.size.1 {
            //         if !self.is_point_free(&(x, y)) {
            //             continue;
            //         }
            //         if self.is_block_free((x, y, x + grid.size.0 - 1, y + grid.size.1 - 1)) {
            //             point = Some((x, y));
            //             break;
            //         }
            //     }
            //     if point.is_some() {
            //         break;
            //     }
            // }
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
                // // Point isn't found. Grid doesn't have enought space. Increase space
                // let f_w = self.size.0 + grid.size.0;
                // let f_h = self.size.1 + grid.size.1;
                // if f_w as i32 - self.size.1 as i32 >= f_h as i32 - self.size.0 as i32 {
                //     self.size.0 += grid.size.0 / 2;
                //     self.size.1 += 1;
                // } else {
                //     self.size.1 += grid.size.1 / 2;
                //     self.size.0 += 1;
                // }
            }
        }
        // Merge grid
        if let Some((p_x, p_y)) = point {
            grid.map.iter().for_each(|(id, (x, y, x1, y1))| {
                self.map
                    .insert(id.clone(), (x + p_x, y + p_y, x1 + p_x, y1 + p_y));
            });
        }
        // Remove unused space
        self.cut_unused_space();
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

pub type FormSize = (String, (u32, u32));

fn get_sizes(forms: Vec<&Form>) -> Result<Vec<FormSize>, E> {
    let mut data = vec![];
    for form in forms {
        data.push((form.id(), form.cells()?));
    }
    Ok(data)
}

fn forms_as_pair(a: Vec<&Form>, b: Vec<&Form>) -> Result<Grid, E> {
    let on_left = get_sizes(a)?;
    let on_right = get_sizes(b)?;
    let mut map: HashMap<String, (u32, u32, u32, u32)> = HashMap::new();
    let mut cursor_by_y: u32 = 0;
    let mut size: (u32, u32) = (0, 0);
    if !on_left.is_empty() {
        on_left.iter().for_each(|(id, (w, h))| {
            map.insert(
                id.to_string(),
                (0, cursor_by_y, w - 1, cursor_by_y + (h - 1)),
            );
            cursor_by_y += h + SPACE_IN_VERTICAL;
            if size.0 < *w {
                size.0 = *w;
                if *w > 3 {
                    console_log!("OOPPS: {w}");
                }
            }
        });
        size.1 = cursor_by_y - SPACE_IN_VERTICAL;
        if !on_right.is_empty() {
            size.0 += SPACE_IN_HORIZONT;
        }
    };
    if !on_right.is_empty() {
        cursor_by_y = 0;
        let mut max_w = 0;
        on_right.iter().for_each(|(id, (w, h))| {
            map.insert(
                id.to_string(),
                (size.0, cursor_by_y, size.0 + (w - 1), cursor_by_y + (h - 1)),
            );
            cursor_by_y += h + SPACE_IN_VERTICAL;
            if max_w < *w {
                max_w = *w;
            }
        });
        size.1 = if cursor_by_y - SPACE_IN_VERTICAL > size.1 {
            cursor_by_y - SPACE_IN_VERTICAL
        } else {
            size.1
        };
        size.0 += max_w;
    }
    Ok(Grid {
        offset: 0,
        size,
        map,
        id: None,
    })
}
fn with_forms_by_sides(left: Vec<&Form>, center: Vec<&Form>, right: Vec<&Form>) -> Result<Grid, E> {
    let on_left = get_sizes(left)?;
    let on_center = get_sizes(center)?;
    let on_right = get_sizes(right)?;
    let mut map: HashMap<String, (u32, u32, u32, u32)> = HashMap::new();
    let mut size: (u32, u32) = (0, 0);
    // Put left side
    let mut cursor_by_y: u32 = 0;
    on_left.iter().for_each(|(id, (w, h))| {
        map.insert(id.to_string(), (0, cursor_by_y, *w, cursor_by_y + h));
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
            id.to_string(),
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
            id.to_string(),
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
        id: None,
    })
}

fn from_grids_into_row(grids: &[Grid]) -> Grid {
    let mut map: HashMap<String, (u32, u32, u32, u32)> = HashMap::new();
    let mut size: (u32, u32) = (0, 0);
    grids.iter().for_each(|grid| {
        grid.map.iter().for_each(|(id, (x, y, x1, y1))| {
            map.insert(id.clone(), (x + size.0, *y, x1 + size.0, *y1));
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
        id: None,
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
    packed
}
