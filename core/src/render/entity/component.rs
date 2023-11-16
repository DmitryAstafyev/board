use wasm_bindgen_test::console_log;

use crate::{
    entity::{Component, Port, PortType},
    error::E,
    render::{
        entity::port::PORT_SIDE, form::GridRectangle, Form, Grid, Relative, Render, Representation,
        Style,
    },
};

const MIN_HEIGHT: i32 = 64;
const MIN_WIDTH: i32 = 64;
const PORTS_VERTICAL_OFFSET: i32 = 8;

impl Render<Component> {
    pub fn new(mut entity: Component) -> Self {
        entity.ports.ports = entity
            .ports
            .ports
            .drain(..)
            .map(|r| {
                if let Representation::Origin(port) = r {
                    Representation::Render(Render::<Port>::new(port))
                } else {
                    r
                }
            })
            .collect::<Vec<Representation<Port>>>();
        let id = entity.sig.id;
        Self {
            entity,
            form: Form::GridRectangle(GridRectangle::new(id, 0, 0, MIN_WIDTH, MIN_HEIGHT)),
            style: Style {
                stroke_style: String::from("rgb(0,0,0)"),
                fill_style: String::from("rgb(200,250,200)"),
            },
            grid: Some(Grid::new()),
        }
    }

    fn ports_height(&self) -> i32 {
        if self.entity.ports.is_empty() {
            return 0;
        }
        [
            self.entity.ports.filter(PortType::In).len(),
            self.entity.ports.filter(PortType::Out).len(),
        ]
        .iter()
        .max()
        .copied()
        .unwrap_or(0) as i32
            * (PORTS_VERTICAL_OFFSET + PORT_SIDE)
            + PORTS_VERTICAL_OFFSET
    }

    pub fn calc(&mut self) -> Result<(), E> {
        // Set self size
        self.form.set_box_size(
            None,
            Some(
                [MIN_HEIGHT, self.ports_height()]
                    .iter()
                    .max()
                    .copied()
                    .unwrap_or(MIN_HEIGHT),
            ),
        );
        // Calc ports
        for port in self.entity.ports.ports.iter_mut() {
            port.render_mut()?.calc()?;
        }
        // Order ports on a left side
        let mut cursor: i32 = PORTS_VERTICAL_OFFSET;
        for port in self.entity.ports.filter_mut(PortType::In) {
            let render = port.render_mut()?;
            let (w, h) = render.form.get_box_size();
            render.form.set_coors(Some(-(w / 2)), Some(cursor));
            cursor += h + PORTS_VERTICAL_OFFSET;
        }
        // Order ports on a right side
        cursor = PORTS_VERTICAL_OFFSET;
        let (self_width, _) = self.form.get_box_size();
        for port in self.entity.ports.filter_mut(PortType::Out) {
            let render = port.render_mut()?;
            let (w, h) = render.form.get_box_size();
            render
                .form
                .set_coors(Some(self_width - (w / 2)), Some(cursor));
            cursor += h + PORTS_VERTICAL_OFFSET;
        }
        Ok(())
    }

    pub fn draw(
        &self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
    ) -> Result<(), E> {
        self.style.apply(context);
        self.form.render(context, relative);
        let self_relative = self.relative(relative);
        for port in self.entity.ports.ports.iter() {
            port.render()?.draw(context, &self_relative)?;
        }
        let _ = context.stroke_text(
            &self.origin().sig.id.to_string(),
            relative.x(self.form.get_coors().0 + 4) as f64,
            relative.y(self.form.get_coors().1 + 4) as f64,
        );
        Ok(())
    }
}
