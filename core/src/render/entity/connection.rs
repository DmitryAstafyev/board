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
        state: &State,
    ) -> Result<(), E> {
        let in_port = *self.origin().in_port();
        let out_port = *self.origin().out_port();
        if let Form::Path(_, path) = &mut self.view.container.form {
            path.sdot = state.is_port_selected(&in_port);
            path.edot = state.is_port_selected(&out_port);
        }
        self.view.render(context, relative);
        Ok(())
    }
}
