use crate::{
    entity::{Component, Ports},
    error::E,
    render::{elements, form::GridRectangle, Form, Relative, Render, Representation, Style},
};

const MIN_HEIGHT: i32 = 64;
const MIN_WIDTH: i32 = 64;

impl Render<Component> {
    pub fn new(mut entity: Component) -> Self {
        entity.ports = if let Representation::Origin(ports) = entity.ports {
            Representation::Render(Render::<Ports>::new(ports))
        } else {
            entity.ports
        };
        let id = entity.sig.id;
        let composition = entity.composition;
        Self {
            entity,
            form: Form::GridRectangle(GridRectangle::new(id, 0, 0, MIN_WIDTH, MIN_HEIGHT)),
            style: Style {
                stroke_style: String::from("rgb(0,0,0)"),
                fill_style: if composition {
                    String::from("rgb(250,200,200)")
                } else {
                    String::from("rgb(200,250,200)")
                },
            },
            over_style: None,
            hidden: false,
        }
    }

    pub fn calc(&mut self) -> Result<(), E> {
        // Set self size
        self.form.set_box_size(
            None,
            Some(elements::max(
                &[MIN_HEIGHT, self.entity.ports.render()?.height()],
                MIN_HEIGHT,
            )),
        );
        // Calc ports
        self.entity
            .ports
            .render_mut()?
            .calc(self.form.get_box_size().0)?;
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
        let self_relative = self.relative(relative);
        self.entity.ports.render()?.draw(context, &self_relative)?;
        let _ = context.stroke_text(
            &self.origin().sig.id.to_string(),
            relative.x(self.form.get_coors().0) as f64,
            relative.y(self.form.get_coors().1 - 4) as f64,
        );
        Ok(())
    }
}
