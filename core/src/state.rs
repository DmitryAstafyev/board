use crate::{entity::Port, render::Relative};

#[derive(Debug)]
pub struct State {
    components: Vec<usize>,
    ports: Vec<usize>,
    ports_highlighted: Vec<usize>,
    hovered: Option<usize>,
    // (ports, linked, owners)
    // ports - filtered ports
    // linked - ports linked to filtered ports
    // owners - components and compositions onwers of filtered and linked ports
    pub filtered: Option<(Vec<usize>, Vec<usize>, Vec<usize>)>,
    pub x: i32,
    pub y: i32,
    pub zoom: f64,
}

impl State {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            ports: Vec::new(),
            ports_highlighted: Vec::new(),
            hovered: None,
            filtered: None,
            x: 0,
            y: 0,
            zoom: 1.0,
        }
    }

    pub fn get_view_relative(&self) -> Relative {
        Relative::new(self.x, self.y, Some(self.zoom))
    }

    pub fn set_view_state(&mut self, x: i32, y: i32, zoom: f64) {
        self.x = x;
        self.y = y;
        self.zoom = zoom;
    }

    pub fn set_filtered(&mut self, filtered: Option<(Vec<usize>, Vec<usize>, Vec<usize>)>) {
        self.filtered = filtered;
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
        if let Some((i, _)) = self.ports.iter().enumerate().find(|(_, port)| *port == id) {
            let _ = self.ports.remove(i);
            false
        } else {
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

    pub fn is_component_selected(&self, id: &usize) -> bool {
        self.components.contains(id)
    }

    pub fn is_any_port_selected(&self, ids: Vec<&usize>) -> bool {
        ids.iter().any(|id| self.ports.contains(id))
    }
}
