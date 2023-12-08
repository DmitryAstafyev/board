use crate::{
    entity::{Port, PortType, Ports},
    error::E,
    render::{
        elements, form::Rectangle, Container, Form, Relative, Render, Representation, Style, View,
    },
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
            view: View {
                container: Container {
                    form: Form::Rectangle(Rectangle {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                        id: String::new(),
                    }),
                    style: Style {
                        stroke_style: String::from("rgb(0,0,0)"),
                        fill_style: String::from("rgb(0,0,0)"),
                    },
                    hover: None,
                },
                elements: vec![],
            },
            hidden: false,
        }
    }

    pub fn height(&self) -> i32 {
        if self.entity.ports.is_empty() {
            return 0;
        }
        let ports_in = self.entity.filter(PortType::In);
        let ports_out = self.entity.filter(PortType::Out);
        elements::max(
            &[
                if self.origin().hide_invisible {
                    ports_in
                        .iter()
                        .filter(|r| r.origin().visibility)
                        .collect::<Vec<&&Representation<Port>>>()
                        .len()
                } else {
                    ports_in.len()
                },
                if self.origin().hide_invisible {
                    ports_out
                        .iter()
                        .filter(|r| r.origin().visibility)
                        .collect::<Vec<&&Representation<Port>>>()
                        .len()
                } else {
                    ports_in.len()
                },
            ],
            0,
        ) as i32
            * (PORTS_VERTICAL_OFFSET + PORT_SIDE)
            + PORTS_VERTICAL_OFFSET
    }

    pub fn calc(&mut self, container_width: i32) -> Result<(), E> {
        let hide = self.origin().hide_invisible;
        // Calc ports
        for port in self
            .entity
            .ports
            .iter_mut()
            .filter(|p| p.origin().visibility || !hide)
        {
            port.render_mut()?.calc()?;
        }
        // Order ports on a left side
        let mut cursor: i32 = PORTS_VERTICAL_OFFSET;
        for port in self
            .entity
            .filter_mut(PortType::In)
            .iter_mut()
            .filter(|p| p.origin().visibility || !hide)
        {
            let render = port.render_mut()?;
            let (w, h) = render.view.container.get_box_size();
            render
                .view
                .container
                .set_coors(Some(-(w / 2)), Some(cursor));
            cursor += h + PORTS_VERTICAL_OFFSET;
        }
        // Order ports on a right side
        cursor = PORTS_VERTICAL_OFFSET;
        for port in self
            .entity
            .filter_mut(PortType::Out)
            .iter_mut()
            .filter(|p| p.origin().visibility || !hide)
        {
            let render = port.render_mut()?;
            let (w, h) = render.view.container.get_box_size();
            render
                .view
                .container
                .set_coors(Some(container_width - (w / 2)), Some(cursor));
            cursor += h + PORTS_VERTICAL_OFFSET;
        }
        Ok(())
    }

    pub fn draw(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
    ) -> Result<(), E> {
        // if let Some(over) = self.over_style.as_ref() {
        //     over.apply(context);
        // } else {
        //     self.style.apply(context);
        // }
        let self_relative = self.relative(relative);
        let hide = self.origin().hide_invisible;
        for port in self
            .entity
            .ports
            .iter_mut()
            .filter(|p| p.origin().visibility || !hide)
        {
            port.render_mut()?.draw(context, &self_relative)?;
        }
        Ok(())
    }
}

impl Render<Port> {
    pub fn new(entity: Port) -> Self {
        let id = entity.sig.id;
        Self {
            entity,
            view: View {
                container: Container {
                    form: Form::Rectangle(Rectangle {
                        x: 0,
                        y: 0,
                        w: PORT_SIDE,
                        h: PORT_SIDE,
                        id: id.to_string(),
                    }),
                    style: Style {
                        stroke_style: String::from("rgb(0,0,0)"),
                        fill_style: String::from("rgb(50,50,50)"),
                    },
                    hover: None,
                },
                elements: vec![],
            },
            hidden: false,
        }
    }

    pub fn calc(&mut self) -> Result<(), E> {
        Ok(())
    }

    pub fn draw(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
    ) -> Result<(), E> {
        // if let Some(over) = self.over_style.as_ref() {
        //     over.apply(context);
        // } else {
        //     self.style.apply(context);
        // }
        self.view.render(context, relative);
        Ok(())
    }
}
