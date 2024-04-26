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

struct Filter<'a> {
    bound: &'a mut Render<Ports>,
}

impl<'a> Filter<'a> {
    pub fn new(bound: &'a mut Render<Ports>) -> Self {
        Self { bound }
    }

    pub fn all(&mut self, state: &State) -> Vec<&mut Representation<Port>> {
        let hide = self.bound.origin().hide_invisible;
        self.bound
            .entity
            .ports
            .iter_mut()
            .filter(|p| {
                (p.origin().visibility || !hide) && state.is_port_filtered_or_linked(p.origin())
            })
            .collect::<Vec<&mut Representation<Port>>>()
    }

    pub fn left(&mut self, state: &State) -> Vec<&mut Representation<Port>> {
        let hide = self.bound.origin().hide_invisible;
        self.bound
            .entity
            .filter_mut(&[PortType::In, PortType::Unbound])
            .into_iter()
            .filter(|p| {
                (p.origin().visibility || !hide) && state.is_port_filtered_or_linked(p.origin())
            })
            .collect::<Vec<&mut Representation<Port>>>()
    }

    pub fn right(&mut self, state: &State) -> Vec<&mut Representation<Port>> {
        let hide = self.bound.origin().hide_invisible;
        self.bound
            .entity
            .filter_mut(&[PortType::Out])
            .into_iter()
            .filter(|p| {
                (p.origin().visibility || !hide) && state.is_port_filtered_or_linked(p.origin())
            })
            .collect::<Vec<&mut Representation<Port>>>()
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

    fn filter(&mut self) -> Filter<'_> {
        Filter::new(self)
    }

    pub fn height(&mut self, state: &State, options: &Options) -> i32 {
        if self.entity.ports.is_empty() {
            return 0;
        }
        let max_in = self.filter().left(state).len();
        let max_out = self.filter().right(state).len();
        let padding = options.ratio().get(PORTS_VERTICAL_OFFSET);
        elements::max(&[max_in, max_out], 0) as i32 * padding + padding
    }

    pub fn calc(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        container_width: i32,
        relative: &Relative,
        options: &Options,
        state: &State,
    ) -> Result<(), E> {
        // Calc ports
        for port in self.filter().all(state) {
            port.render_mut()?.calc(context, relative, options)?;
        }
        let ratio = options.ratio();
        let padding = ratio.get(PORTS_VERTICAL_OFFSET);
        let side = ratio.get(PORT_SIDE);
        let cell = ratio.get(CELL);
        match options.ports.representation {
            options::PortsRepresentation::Blocks => {
                // Order ports on a left side
                let mut cursor: i32 = padding / 2 - side / 2;
                for port in self.filter().left(state) {
                    let render = port.render_mut()?;
                    let (w, _h) = render.view.container.get_box_size();
                    render
                        .view
                        .container
                        .set_coors(Some(-(w / 2)), Some(cursor));
                    cursor += padding;
                }
                // Order ports on a right side
                cursor = padding / 2 - side / 2;
                for port in self.filter().right(state) {
                    let render = port.render_mut()?;
                    let (w, _h) = render.view.container.get_box_size();
                    render
                        .view
                        .container
                        .set_coors(Some(container_width - (w / 2)), Some(cursor));
                    cursor += padding;
                }
            }
            options::PortsRepresentation::Labels => {
                let label_height = (cell as f64 * 0.7).ceil() as i32;
                let step_between = cell as i32 - label_height;
                let start_from = (step_between as f64 / 2.0).ceil() as i32;
                let over = (container_width as f64 * 0.8 / 2.0) as i32;
                // Order ports on a left side
                let mut cursor: i32 = start_from;
                for port in self.filter().left(state) {
                    port.render_mut()?
                        .view
                        .container
                        .set_coors(Some(over), Some(cursor));
                    cursor += step_between + label_height;
                }
                // Order ports on a right side
                cursor = start_from;
                for port in self.filter().right(state) {
                    port.render_mut()?
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
        for port in self.filter().all(state) {
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
            entity.get_label(options)
        } else if let (1, Some(id)) = (entity.contains.len(), entity.contains.first()) {
            id.to_string()
        } else {
            format!("{} ports", entity.contains.len())
        };
        let align = match entity.port_type {
            PortType::Out => label::Align::Left,
            PortType::In | PortType::Unbound => label::Align::Right,
        };
        let ratio = options.ratio();
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
                                w: ratio.get(PORT_SIDE),
                                h: ratio.get(PORT_SIDE),
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
                            Label::new(
                                0,
                                0,
                                0,
                                0,
                                label,
                                4,
                                id.to_string(),
                                align,
                                &options.ratio(),
                            ),
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
