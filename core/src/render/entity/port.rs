use crate::{
    entity::{Port, PortType, Ports},
    error::E,
    render::{form::Rectangle, Form, Relative, Render, Representation, Style},
};

pub const PORT_SIDE: i32 = 8;
const PORTS_VERTICAL_OFFSET: i32 = 8;

impl Render<Ports> {
    pub fn new(mut entity: Ports) -> Self {
        entity.ports = entity
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
        Self {
            entity,
            form: Form::Rectangle(Rectangle {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
                id: 0,
            }),
            style: Style {
                stroke_style: String::from("rgb(0,0,0)"),
                fill_style: String::from("rgb(0,0,0)"),
            },
            over_style: None,
            grid: None,
        }
    }

    pub fn height(&self) -> i32 {
        if self.entity.ports.is_empty() {
            return 0;
        }
        [
            self.entity.filter(PortType::In).len(),
            self.entity.filter(PortType::Out).len(),
        ]
        .iter()
        .max()
        .copied()
        .unwrap_or(0) as i32
            * (PORTS_VERTICAL_OFFSET + PORT_SIDE)
            + PORTS_VERTICAL_OFFSET
    }

    pub fn calc(&mut self, container_width: i32) -> Result<(), E> {
        // Calc ports
        for port in self.entity.ports.iter_mut() {
            port.render_mut()?.calc()?;
        }
        // Order ports on a left side
        let mut cursor: i32 = PORTS_VERTICAL_OFFSET;
        for port in self.entity.filter_mut(PortType::In) {
            let render = port.render_mut()?;
            let (w, h) = render.form.get_box_size();
            render.form.set_coors(Some(-(w / 2)), Some(cursor));
            cursor += h + PORTS_VERTICAL_OFFSET;
        }
        // Order ports on a right side
        cursor = PORTS_VERTICAL_OFFSET;
        for port in self.entity.filter_mut(PortType::Out) {
            let render = port.render_mut()?;
            let (w, h) = render.form.get_box_size();
            render
                .form
                .set_coors(Some(container_width - (w / 2)), Some(cursor));
            cursor += h + PORTS_VERTICAL_OFFSET;
        }
        Ok(())
    }

    pub fn draw(
        &self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
    ) -> Result<(), E> {
        if let Some(over) = self.over_style.as_ref() {
            over.apply(context);
        } else {
            self.style.apply(context);
        }
        let self_relative = self.relative(relative);
        for port in self.entity.ports.iter() {
            port.render()?.draw(context, &self_relative)?;
        }
        Ok(())
    }
}

impl Render<Port> {
    pub fn new(entity: Port) -> Self {
        let id = entity.sig.id;
        Self {
            entity,
            form: Form::Rectangle(Rectangle {
                x: 0,
                y: 0,
                w: PORT_SIDE,
                h: PORT_SIDE,
                id,
            }),
            style: Style {
                stroke_style: String::from("rgb(0,0,0)"),
                fill_style: String::from("rgb(50,50,50)"),
            },
            over_style: None,
            grid: None,
        }
    }

    pub fn calc(&mut self) -> Result<(), E> {
        Ok(())
    }

    pub fn draw(
        &self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
    ) -> Result<(), E> {
        if let Some(over) = self.over_style.as_ref() {
            over.apply(context);
        } else {
            self.style.apply(context);
        }
        self.form.render(context, relative);
        Ok(())
    }
}
