use crate::{
    entity::{Component, Ports},
    error::E,
    render::{
        elements, form::GridRectangle, grid::ElementType, options::Options, Container, Form,
        Relative, Render, Representation, Style, View,
    },
    state::State,
};

const MIN_HEIGHT: i32 = 64;
const MIN_WIDTH: i32 = 64;

impl Render<Component> {
    pub fn new(mut entity: Component, options: &Options, mut ty: Option<ElementType>) -> Self {
        entity.ports = if let Representation::Origin(ports) = entity.ports {
            Representation::Render(Render::<Ports>::new(ports, options))
        } else {
            entity.ports
        };
        let id = entity.sig.id;
        let composition = entity.composition;
        Self {
            entity,
            view: View {
                container: Container {
                    form: Form::GridRectangle(
                        if let Some(ty) = ty.take() {
                            ty
                        } else {
                            ElementType::Component
                        },
                        GridRectangle::new(id.to_string(), 0, 0, MIN_WIDTH, MIN_HEIGHT),
                    ),
                    style: Style {
                        stroke_style: String::from("rgb(0,0,0)"),
                        fill_style: if composition {
                            String::from("rgb(250,200,200)")
                        } else {
                            String::from("rgb(200,250,200)")
                        },
                    },
                    hover: None,
                },
                elements: vec![],
            },
            hidden: false,
        }
    }

    pub fn calc(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
        options: &Options,
    ) -> Result<(), E> {
        // Set self size
        self.view.container.set_box_size(
            None,
            Some(elements::max(
                &[MIN_HEIGHT, self.entity.ports.render()?.height()],
                MIN_HEIGHT,
            )),
        );
        // Calc ports
        let self_relative = self.relative(relative);
        self.entity.ports.render_mut()?.calc(
            context,
            self.view.container.get_box_size().0,
            &self_relative,
            options,
        )?;
        Ok(())
    }

    pub fn draw(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
        options: &Options,
        state: &State,
    ) -> Result<(), E> {
        if matches!(
            self.view.container.form.get_el_ty(),
            ElementType::Composition
        ) {
            self.view.container.style = Style {
                stroke_style: String::from("rgb(30,30,30)"),
                fill_style: String::from("rgb(250,200,200)"),
            };
        } else if state.is_component_selected(&self.entity.sig.id) {
            self.view.container.style = Style {
                stroke_style: String::from("rgb(0,0,0)"),
                fill_style: String::from("rgb(100,150,100)"),
            };
        } else {
            self.view.container.style = Style {
                stroke_style: String::from("rgb(30,30,30)"),
                fill_style: String::from("rgb(250,250,250)"),
            };
        }
        self.view.render(context, relative);
        let self_relative = self.relative(relative);
        self.entity
            .ports
            .render_mut()?
            .draw(context, &self_relative, options, state)?;
        context.set_text_baseline("bottom");
        context.set_font(&format!("{}px serif", relative.zoom(12)));
        let _ = context.stroke_text(
            &self.origin().sig.id.to_string(),
            relative.x(self.view.container.get_coors().0) as f64,
            relative.y(self.view.container.get_coors().1 - 3) as f64,
        );
        Ok(())
    }
}
