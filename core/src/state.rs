use crate::{entity::Port, render::Relative};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::console_log;

#[derive(Debug)]
pub struct Selection {
    components: Vec<usize>,
    ports: Vec<usize>,
    // Callback to notify about current selection
    pub selcb: Option<js_sys::Function>,
}

impl Selection {
    pub fn new(selcb: js_sys::Function) -> Self {
        Self {
            components: Vec::new(),
            ports: Vec::new(),
            selcb: Some(selcb),
        }
    }
    pub fn insert_component(&mut self, id: &usize) -> &mut Self {
        if !self.components.contains(id) {
            self.components.push(*id);
        }
        self
    }

    pub fn remove_component(&mut self, id: &usize) -> &mut Self {
        if let Some(pos) = self.components.iter().position(|v| v == id) {
            self.components.remove(pos);
        }
        self
    }

    pub fn insert_port(&mut self, id: &usize) -> &mut Self {
        if !self.ports.contains(id) {
            self.ports.push(*id);
        }
        self
    }

    pub fn remove_port(&mut self, id: &usize) -> &mut Self {
        if let Some(pos) = self.ports.iter().position(|v| v == id) {
            self.ports.remove(pos);
        }
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.ports.clear();
        self.components.clear();
        self
    }

    pub fn notify(&self) {
        if let Some(selcb) = self.selcb.as_ref() {
            let selections = (&self.components, &self.ports);
            let Ok(value) = serde_wasm_bindgen::to_value(&selections) else {
                console_log!("Fail to send current selection data");
                return;
            };
            let _ = selcb.call1(&JsValue::NULL, &value);
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub selection: Selection,
    pub components: Vec<usize>,
    ports: Vec<usize>,
    ports_highlighted: Vec<usize>,
    hovered: Option<usize>,
    hmargin: i32,
    vmargin: i32,
    // (ports, linked, owners)
    // ports - filtered ports/components/compositions
    // linked - ports linked to filtered ports
    // owners - components and compositions onwers of filtered and linked ports
    pub filtered: Option<(Vec<usize>, Vec<usize>, Vec<usize>)>,
    pub matches_extended: Option<Vec<(usize, Option<usize>)>>,
    pub matches: Option<Vec<usize>>,
    pub highlighted: Option<Vec<usize>>,
    pub x: i32,
    pub y: i32,
    pub zoom: f64,
}

impl State {
    pub fn new(hmargin: i32, vmargin: i32, selcb: js_sys::Function) -> Self {
        Self {
            selection: Selection::new(selcb),
            components: Vec::new(),
            ports: Vec::new(),
            ports_highlighted: Vec::new(),
            hovered: None,
            filtered: None,
            matches: None,
            matches_extended: None,
            highlighted: None,
            hmargin,
            vmargin,
            x: 0,
            y: 0,
            zoom: 1.0,
        }
    }

    pub fn get_view_relative(&self) -> Relative {
        Relative::new(self.x, self.y, Some(self.zoom))
    }

    pub fn get_grid_relative(&self) -> Relative {
        Relative::new(
            self.x + self.hmargin,
            self.y + self.vmargin,
            Some(self.zoom),
        )
    }

    pub fn x_margin(&self) -> i32 {
        self.x + self.hmargin
    }

    pub fn y_margin(&self) -> i32 {
        self.y + self.vmargin
    }

    pub fn with_hmargin(&self, v: i32) -> i32 {
        v - (self.hmargin as f64 * self.zoom) as i32
    }

    pub fn with_vmargin(&self, v: i32) -> i32 {
        v - (self.vmargin as f64 * self.zoom) as i32
    }

    pub fn set_view_state(&mut self, x: i32, y: i32, zoom: f64) {
        self.x = x;
        self.y = y;
        self.zoom = zoom;
    }

    pub fn set_filtered(&mut self, filtered: Option<(Vec<usize>, Vec<usize>, Vec<usize>)>) {
        self.filtered = filtered;
    }

    pub fn get_filtered(&self) -> Option<&Vec<usize>> {
        self.filtered.as_ref().map(|(ids, _, _)| ids)
    }

    pub fn set_matches(
        &mut self,
        matches: Option<Vec<usize>>,
        extended: Option<Vec<(usize, Option<usize>)>>,
    ) {
        self.matches = matches;
        self.matches_extended = extended;
    }

    pub fn get_matches(&self) -> Option<&Vec<usize>> {
        self.matches.as_ref()
    }

    pub fn get_extended_matches(&self) -> Option<&Vec<(usize, Option<usize>)>> {
        self.matches_extended.as_ref()
    }

    pub fn is_match(&self, port_id: &usize) -> bool {
        self.matches
            .as_ref()
            .map(|ids| ids.contains(port_id))
            .unwrap_or(false)
    }

    pub fn set_highlighted(&mut self, highlighted: Option<Vec<usize>>) {
        self.highlighted = highlighted;
    }

    pub fn get_highlighted(&self) -> Option<&Vec<usize>> {
        self.highlighted.as_ref()
    }

    pub fn is_highlighted(&self, id: &usize) -> bool {
        self.highlighted
            .as_ref()
            .map(|ids| ids.contains(id))
            .unwrap_or(false)
    }

    pub fn is_port_linked(&self, port: &Port) -> bool {
        if let Some((_filtered, linked, _owners)) = self.filtered.as_ref() {
            linked.contains(&port.sig.id)
        } else {
            false
        }
    }

    pub fn is_port_owner_filtered(&self, id: &usize) -> bool {
        if let Some((_filtered, _linked, owners)) = self.filtered.as_ref() {
            owners.contains(id)
        } else {
            true
        }
    }

    pub fn is_port_filtered_or_linked(&self, port: &Port) -> bool {
        if let Some((filtered, linked, _owners)) = self.filtered.as_ref() {
            filtered.contains(&port.sig.id) || linked.contains(&port.sig.id)
        } else {
            true
        }
    }

    pub fn toggle_port(&mut self, id: &usize) -> bool {
        if let Some(pos) = self.ports.iter().position(|port| port == id) {
            let _ = self.ports.remove(pos);
            self.selection.remove_port(id).notify();
            false
        } else {
            self.selection.insert_port(id).notify();
            self.ports.push(*id);
            true
        }
    }

    pub fn insert_component(&mut self, id: &usize) -> bool {
        if !self.components.contains(id) {
            self.components.push(*id);
            true
        } else {
            false
        }
    }

    pub fn remove_component(&mut self, id: &usize) -> bool {
        if let Some(i) = self.components.iter().position(|v| v == id) {
            let _ = self.components.remove(i);
            true
        } else {
            false
        }
    }

    pub fn insert_port(&mut self, id: &usize) -> bool {
        if !self.ports.contains(id) {
            self.ports.push(*id);
            true
        } else {
            false
        }
    }

    pub fn remove_port(&mut self, id: &usize) -> bool {
        if let Some(i) = self.ports.iter().position(|v| v == id) {
            let _ = self.ports.remove(i);
            true
        } else {
            false
        }
    }

    pub fn unselect_all(&mut self, silence: bool) {
        self.ports.clear();
        self.ports_highlighted.clear();
        self.components.clear();
        self.selection.clear();
        if !silence {
            self.selection.notify();
        }
    }

    pub fn hover(&mut self, id: &usize) -> bool {
        if self.hovered.is_some_and(|v| &v == id) {
            false
        } else {
            self.hovered = Some(*id);
            true
        }
    }

    pub fn unhover(&mut self) -> bool {
        if self.hovered.is_some() {
            self.hovered = None;
            true
        } else {
            false
        }
    }

    pub fn is_hovered(&self, id: &usize) -> bool {
        self.hovered.is_some_and(|v| &v == id)
    }

    pub fn highlight_port(&mut self, id: &usize) -> bool {
        if !self.ports_highlighted.contains(id) {
            self.ports_highlighted.push(*id);
            true
        } else {
            false
        }
    }

    pub fn unhighlight_port(&mut self, id: &usize) -> bool {
        if let Some(i) = self.ports_highlighted.iter().position(|v| v == id) {
            let _ = self.ports_highlighted.remove(i);
            true
        } else {
            false
        }
    }

    pub fn is_port_highlighted(&self, id: &usize) -> bool {
        self.ports_highlighted.contains(id)
    }

    pub fn is_port_selected(&self, id: &usize) -> bool {
        let visible = if let Some((filtered, linked, _owners)) = self.filtered.as_ref() {
            filtered.contains(id) || linked.contains(id)
        } else {
            true
        };
        self.ports.contains(id) && visible
    }

    pub fn is_port_selected_or_highlighted(&self, id: &usize) -> bool {
        let visible = if let Some((filtered, linked, _owners)) = self.filtered.as_ref() {
            filtered.contains(id) || linked.contains(id)
        } else {
            true
        };
        if !visible {
            return false;
        }
        self.ports.contains(id) || self.is_port_highlighted(id)
    }

    pub fn is_component_selected(&self, id: &usize) -> bool {
        self.components.contains(id)
    }

    // pub fn is_any_port_selected(&self, ids: Vec<&usize>) -> bool {
    //     ids.iter().any(|id| self.ports.contains(id))
    // }
}
