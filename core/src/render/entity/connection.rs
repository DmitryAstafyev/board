use crate::{
    entity::{Connection, Signature, SignatureGetter},
    error::E,
    render::{form::Path, grid::ElementType, Container, Form, Relative, Render, Style, View},
};

impl<'a, 'b: 'a> SignatureGetter<'a, 'b> for Render<Connection> {
    fn sig(&'b self) -> &'a Signature {
        &self.origin().sig
    }
}

impl Render<Connection> {
    pub fn new(entity: Connection) -> Self {
        let id = entity.sig.id;
        Self {
            entity,
            view: View {
                container: Container {
                    form: Form::Path(
                        ElementType::Connection,
                        Path {
                            points: Vec::new(),
                            id: id.to_string(),
                        },
                    ),
                    style: Style {
                        stroke_style: String::from("rgb(30,30,30)"),
                        fill_style: String::from("rgb(30,30,30)"),
                    },
                },
                elements: Vec::new(),
            },
            hidden: false,
        }
    }
    pub fn draw(
        &mut self,
        context: &mut web_sys::CanvasRenderingContext2d,
        relative: &Relative,
    ) -> Result<(), E> {
        self.view.render(context, relative);
        Ok(())
    }
}
