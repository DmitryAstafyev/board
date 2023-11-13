use crate::{
    entity::{Component, Composition, Connection, Port, PortType},
    error::E,
    render::{form::Rectangle, Form, Relative, Render, Representation, Style},
};

impl Render<Composition> {
    pub fn new(mut entity: Composition) -> Self {
        entity.components = entity
            .components
            .drain(..)
            .map(|r| {
                if let Representation::Origin(component) = r {
                    Representation::Render(Render::<Component>::new(component))
                } else {
                    r
                }
            })
            .collect::<Vec<Representation<Component>>>();
        entity.connections = entity
            .connections
            .drain(..)
            .map(|r| {
                if let Representation::Origin(connection) = r {
                    Representation::Render(Render::<Connection>::new(connection))
                } else {
                    r
                }
            })
            .collect::<Vec<Representation<Connection>>>();
        Self {
            entity,
            form: Form::Rectangle(Rectangle {
                x: 200,
                y: 20,
                w: 100,
                h: 100,
            }),
            style: Style {
                stroke_style: String::from("rgb(0,0,0)"),
                fill_style: String::from("rgb(230,230,230)"),
            },
        }
    }

    pub fn calc(&mut self) -> Result<(), E> {
        for component in self.entity.components.iter_mut() {
            component.render_mut()?.calc()?;
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
        for component in self.entity.components.iter() {
            component.render()?.draw(context, relative)?;
        }
        Ok(())
    }
}
