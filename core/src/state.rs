use crate::render::Relative;

#[derive(Debug)]
pub struct State {
    components: Vec<usize>,
    ports: Vec<usize>,
    pub x: i32,
    pub y: i32,
    pub zoom: f64,
}

impl State {
    pub fn new() -> Self {
        Self {
            components: vec![],
            ports: vec![],
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

    pub fn toggle_component(&mut self, id: usize) {
        if let Some((i, _)) = self
            .components
            .iter()
            .enumerate()
            .find(|(_, comp)| *comp == &id)
        {
            let _ = self.components.remove(i);
        } else {
            self.components.push(id);
        }
    }

    pub fn toggle_port(&mut self, id: usize) {
        if let Some((i, _)) = self.ports.iter().enumerate().find(|(_, port)| *port == &id) {
            let _ = self.ports.remove(i);
        } else {
            self.ports.push(id);
        }
    }

    pub fn insert_component(&mut self, id: usize) -> bool {
        if !self.components.contains(&id) {
            self.components.push(id);
            true
        } else {
            false
        }
    }

    pub fn remove_component(&mut self, id: usize) -> bool {
        if let Some((i, _)) = self
            .components
            .iter()
            .enumerate()
            .find(|(_, comp)| *comp == &id)
        {
            let _ = self.components.remove(i);
            true
        } else {
            false
        }
    }

    pub fn insert_port(&mut self, id: usize) -> bool {
        if !self.ports.contains(&id) {
            self.ports.push(id);
            true
        } else {
            false
        }
    }

    pub fn remove_port(&mut self, id: usize) -> bool {
        if let Some((i, _)) = self.ports.iter().enumerate().find(|(_, comp)| *comp == &id) {
            let _ = self.ports.remove(i);
            true
        } else {
            false
        }
    }

    pub fn is_port_selected(&self, id: &usize) -> bool {
        self.ports.contains(id)
    }

    pub fn is_component_selected(&self, id: &usize) -> bool {
        self.components.contains(id)
    }
}
