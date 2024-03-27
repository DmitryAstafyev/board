use crate::{
    entity::{Port, PortType, Ports, Signature, SignatureGetter},
    error::E,
    render::{
        elements,
        form::{label, Label, Rectangle},
        grid::{ElementCoors, ElementType, CELL},
        options::{self, Options},
        Container, Form, Relative, Render, Representation, Style, View,
    },
    state::State,
};

pub const PORT_SIDE: i32 = 8;
const PORTS_VERTICAL_OFFSET: i32 = CELL as i32;

impl<'a, 'b: 'a> SignatureGetter<'a, 'b> for Render<Ports> {
    fn sig(&'b self) -> &'a Signature {
        &self.origin().sig
    }
}

impl Render<Ports> {
    pub fn new(mut entity: Ports, options: &Options) -> Self {
        entity.ports = entity
            .ports
            .drain(..)
            .map(|r| {
                if let Representation::Origin(port) = r {
                    Representation::Render(Render::<Port>::new(port, options))
                } else {
                    r
                }
            })
            .collect::<Vec<Representation<Port>>>();
        Self {
            entity,
            view: View {
                container: Container {
                    form: Form::Rectangle(
                        ElementType::Port,
                        Rectangle {
                            x: 0,
                            y: 0,
                            w: 0,
                            h: 0,
                            id: String::new(),
                        },
                    ),
                    style: Style {
                        stroke_style: String::from("rgb(0,0,0)"),
                        fill_style: String::from("rgb(0,0,0)"),
                    },
                },
                elements: Vec::new(),
            },
            hidden: false,
        }
    }

    pub fn height(&self, state: &State) -> i32 {
        if self.entity.ports.is_empty() {
            return 0;
        }
        let ports_in = self.entity.filter(&[PortType::In, PortType::Unbound]);
        let ports_out = self.entity.filter(&[PortType::Out]);
        let max_in = if self.origin().hide_invisible {
            ports_in
                .iter()
                .filter(|r| r.origin().visibility && state.is_port_filtered_or_linked(r.origin()))
                .collect::<Vec<&&Representation<Port>>>()
                .len()
        } else {
            ports_in.len()
        };
        let max_out = if self.origin().hide_invisible {
            ports_out
                .iter()
                .filter(|r| r.origin().visibility && state.is_port_filtered_or_linked(r.origin()))
                .collect::<Vec<&&Representation<Port>>>()
                .len()
        } else {
            ports_in.len()
        };
        elements::max(&[max_in, max_out], 0) as i32 * PORTS_VERTICAL_OFFSET + PORTS_VERTICAL_OFFSET
    }

    pub fn calc(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        container_width: i32,
        relative: &Relative,
        options: &Options,
        state: &State,
    ) -> Result<(), E> {
        let hide = self.origin().hide_invisible;
        // Calc ports
        for port in self.entity.ports.iter_mut().filter(|p| {
            (p.origin().visibility || !hide) && state.is_port_filtered_or_linked(p.origin())
        }) {
            port.render_mut()?.calc(context, relative, options)?;
        }
        match options.ports.representation {
            options::PortsRepresentation::Blocks => {
                // Order ports on a left side
                let mut cursor: i32 = PORTS_VERTICAL_OFFSET / 2 - PORT_SIDE / 2;
                for port in self
                    .entity
                    .filter_mut(&[PortType::In, PortType::Unbound])
                    .iter_mut()
                    .filter(|p| {
                        (p.origin().visibility || !hide)
                            && state.is_port_filtered_or_linked(p.origin())
                    })
                {
                    let render = port.render_mut()?;
                    let (w, _h) = render.view.container.get_box_size();
                    render
                        .view
                        .container
                        .set_coors(Some(-(w / 2)), Some(cursor));
                    cursor += PORTS_VERTICAL_OFFSET;
                }
                // Order ports on a right side
                cursor = PORTS_VERTICAL_OFFSET / 2 - PORT_SIDE / 2;
                for port in self
                    .entity
                    .filter_mut(&[PortType::Out])
                    .iter_mut()
                    .filter(|p| {
                        (p.origin().visibility || !hide)
                            && state.is_port_filtered_or_linked(p.origin())
                    })
                {
                    let render = port.render_mut()?;
                    let (w, _h) = render.view.container.get_box_size();
                    render
                        .view
                        .container
                        .set_coors(Some(container_width - (w / 2)), Some(cursor));
                    cursor += PORTS_VERTICAL_OFFSET;
                }
            }
            options::PortsRepresentation::Labels => {
                // Order ports on a left side
                let label_height = (CELL as f64 * 0.7).ceil() as i32;
                let step_between = CELL as i32 - label_height;
                let start_from = (step_between as f64 / 2.0).ceil() as i32;
                let mut cursor: i32 = start_from;
                let over = (container_width as f64 * 0.8 / 2.0) as i32;
                for port in self
                    .entity
                    .filter_mut(&[PortType::In, PortType::Unbound])
                    .iter_mut()
                    .filter(|p| {
                        (p.origin().visibility || !hide)
                            && state.is_port_filtered_or_linked(p.origin())
                    })
                {
                    let render = port.render_mut()?;
                    render.view.container.set_coors(Some(over), Some(cursor));
                    cursor += step_between + label_height;
                }
                // Order ports on a right side
                cursor = start_from;
                for port in self
                    .entity
                    .filter_mut(&[PortType::Out])
                    .iter_mut()
                    .filter(|p| {
                        (p.origin().visibility || !hide)
                            && state.is_port_filtered_or_linked(p.origin())
                    })
                {
                    let render = port.render_mut()?;
                    render
                        .view
                        .container
                        .set_coors(Some(container_width - over), Some(cursor));
                    cursor += step_between + label_height;
                }
            }
        }
        Ok(())
    }

    pub fn draw(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
        options: &Options,
        state: &State,
    ) -> Result<(), E> {
        let self_relative = self.relative(relative);
        let hide = self.origin().hide_invisible;
        for port in self.entity.ports.iter_mut().filter(|p| {
            (p.origin().visibility || !hide) && state.is_port_filtered_or_linked(p.origin())
        }) {
            port.render_mut()?
                .draw(context, &self_relative, options, state)?;
        }
        Ok(())
    }
    pub fn find(
        &self,
        position: &(i32, i32),
        relative: &Relative,
        state: &State,
    ) -> Result<Vec<ElementCoors>, E> {
        if self.hidden {
            return Ok(Vec::new());
        }
        // Take into account: position already considers zoom-factor.
        // That's why we need consider zoom-factor only for areas.
        // With width and height a little more complicated situation:
        // these props are calculated during render and it means: height
        // width also already consider zoom factor
        let mut found: Vec<ElementCoors> = Vec::new();
        for port in self.entity.ports.iter() {
            let (x, y) = port.render()?.view.container.get_coors_with_zoom(relative);
            let (w, h) = port.render()?.view.container.get_box_size();
            let area = (x, y, x + w, y + h);
            if elements::is_point_in(position, &area)
                && state.is_port_filtered_or_linked(port.origin())
            {
                found.push((
                    port.sig().id.to_string(),
                    ElementType::Port,
                    (
                        relative.x(0) + x,
                        relative.y(0) + y,
                        relative.x(0) + x + w,
                        relative.y(0) + y + h,
                    ),
                ));
            }
        }
        Ok(found)
    }
}

impl<'a, 'b: 'a> SignatureGetter<'a, 'b> for Render<Port> {
    fn sig(&'b self) -> &'a Signature {
        &self.origin().sig
    }
}

impl Render<Port> {
    pub fn new(entity: Port, options: &Options) -> Self {
        let id = entity.sig.id;
        let label = if entity.contains.is_empty() {
            entity.get_label(options, 20)
        } else if let (1, Some(id)) = (entity.contains.len(), entity.contains.first()) {
            id.to_string()
        } else {
            format!("{} ports", entity.contains.len())
        };
        let align = match entity.port_type {
            PortType::Out => label::Align::Left,
            PortType::In | PortType::Unbound => label::Align::Right,
        };
        Self {
            entity,
            view: View {
                container: match options.ports.representation {
                    options::PortsRepresentation::Blocks => Container {
                        form: Form::Rectangle(
                            ElementType::Port,
                            Rectangle {
                                x: 0,
                                y: 0,
                                w: PORT_SIDE,
                                h: PORT_SIDE,
                                id: id.to_string(),
                            },
                        ),
                        style: Style {
                            stroke_style: String::from("rgb(0,0,0)"),
                            fill_style: String::from("rgb(50,50,50)"),
                        },
                    },
                    options::PortsRepresentation::Labels => Container {
                        form: Form::Label(
                            ElementType::Port,
                            Label {
                                x: 0,
                                y: 0,
                                w: 0,
                                h: 0,
                                id: id.to_string(),
                                padding: 4,
                                label,
                                align,
                            },
                        ),
                        style: Style {
                            stroke_style: String::from("rgb(0,0,0)"),
                            fill_style: String::from("rgb(220,220,220)"),
                        },
                    },
                },
                elements: Vec::new(),
            },
            hidden: false,
        }
    }

    pub fn calc(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
        _options: &Options,
    ) -> Result<(), E> {
        self.view.container.form.calc(context, relative);
        Ok(())
    }

    pub fn draw(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
        _options: &Options,
        state: &State,
    ) -> Result<(), E> {
        if state.is_hovered(&self.entity.sig.id) {
            self.view.container.style = Style {
                stroke_style: String::from("rgb(50,50,50)"),
                fill_style: String::from("rgb(200,200,200)"),
            };
        } else if matches!(self.entity.port_type, PortType::Unbound) {
            self.view.container.style = Style {
                stroke_style: String::from("rgb(50,50,50)"),
                fill_style: String::from("rgb(200,200,240)"),
            };
        } else if state.is_port_selected(&self.entity.sig.id) {
            self.view.container.style = Style {
                stroke_style: String::from("rgb(0,0,0)"),
                fill_style: String::from("rgb(150,250,150)"),
            };
        } else if state.is_port_highlighted(&self.entity.sig.id) {
            self.view.container.style = Style {
                stroke_style: String::from("rgb(0,0,0)"),
                fill_style: String::from("rgb(200,250,200)"),
            };
        } else if state.is_port_linked(&self.entity) {
            self.view.container.style = Style {
                stroke_style: String::from("rgb(150,150,150)"),
                fill_style: String::from("rgb(250,250,250)"),
            };
        } else {
            self.view.container.style = Style {
                stroke_style: String::from("rgb(50,50,50)"),
                fill_style: String::from("rgb(240,240,240)"),
            };
        }
        self.view.render(context, relative);
        Ok(())
    }
}
