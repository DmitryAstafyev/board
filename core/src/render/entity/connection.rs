use crate::{
    entity::{Connection, Signature, SignatureGetter},
    error::E,
    render::{
        form::Path, grid::ElementType, options::Options, Container, Form, Relative, Render, Style,
        View,
    },
    state::State,
};

impl<'a, 'b: 'a> SignatureGetter<'a, 'b> for Render<Connection> {
    fn sig(&'b self) -> &'a Signature {
        &self.origin().sig
    }
}

impl Render<Connection> {
    pub fn new(entity: Connection, options: &Options) -> Self {
        let id = entity.sig.id;
        Self {
            entity,
            view: View {
                container: Container {
                    form: Form::Path(
                        ElementType::Connection,
                        Path::new(id.to_string(), Vec::new(), &options.ratio()),
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
        _state: &State,
    ) -> Result<(), E> {
        if let Form::Path(_, path) = &mut self.view.container.form {
            path.sarrow = true;
            path.sdot = false;
            path.edot = true;
            path.earrow = false;
        }
        self.view.render(context, relative);
        Ok(())
    }
}
