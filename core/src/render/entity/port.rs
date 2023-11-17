use crate::{
    entity::Port,
    error::E,
    render::{form::Rectangle, Form, Relative, Render, Style},
};

pub const PORT_SIDE: i32 = 8;

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
