use crate::{
    entity::Connection,
    error::E,
    render::{form::Path, Form, Render, Style},
};

impl Render<Connection> {
    pub fn new(entity: Connection) -> Self {
        Self {
            entity,
            form: Form::Path(Path { points: vec![] }),
            style: Style {
                stroke_style: String::from("rgb(0,0,0)"),
                fill_style: String::from("rgb(200,200,200)"),
            },
        }
    }
}
