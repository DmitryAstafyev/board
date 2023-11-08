use crate::{
    entity::{Component, Port, Signature},
    representation::{
        self,
        form::{self, path::Path, Form},
        style::{self, Style},
        Representation,
    },
};

#[derive(Debug)]
pub struct Joint<'a> {
    pub port: &'a Port,
    pub component: &'a Component<'a>,
}

#[derive(Debug)]
pub struct Connection<'a> {
    pub sig: Signature,
    pub joint_in: Joint<'a>,
    pub joint_out: Joint<'a>,
    pub repr: Representation,
}

impl form::Default for Connection<'_> {
    fn init() -> Form {
        Form::Path(Path { points: vec![] })
    }
}

impl style::Default for Connection<'_> {
    fn init() -> Style {
        Style {
            stroke_color: String::from("#222222"),
            fill_color: String::from("#FFFFFF"),
        }
    }
}

impl representation::Default for Connection<'_> {
    fn init() -> Representation {
        Representation {
            form: <Connection<'_> as form::Default>::init(),
            style: <Connection<'_> as style::Default>::init(),
        }
    }
}
