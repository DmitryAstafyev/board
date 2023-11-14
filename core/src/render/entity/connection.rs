use crate::{
    entity::Connection,
    error::E,
    render::{form::Path, Form, Render, Style},
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
            grid: None,
        }
    }
}
