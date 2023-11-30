use crate::{
    entity::Connection,
    error::E,
    render::{form::Path, Form, Relative, Render, Style},
};

impl Render<Connection> {
    pub fn new(entity: Connection) -> Self {
        let id = entity.sig.id;
        Self {
            entity,
            form: Form::Path(Path { points: vec![], id }),
            style: Style {
                stroke_style: String::from("rgb(0,0,0)"),
                fill_style: String::from("rgb(200,200,200)"),
            },
            over_style: None,
        }
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
