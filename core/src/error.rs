use std::convert::From;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("DOM error: {0}")]
    DOM(String),
    #[error("Serde parsing error: {0}")]
    Serde(String),
    #[error("Canvas context ins't setup")]
    NoCanvasContext,
    #[error("Entity {0} doesn't have parent")]
    NoParent(String),
    #[error("Render isn't inited")]
    RenderNotInited,
    #[error("Form isn't belong to grid")]
    NotGridForm,
    #[error("Static error message")]
    NotSupported,
    #[error("{0}")]
    Other(String),
}

impl From<E> for std::string::String {
    fn from(value: E) -> Self {
        value.to_string()
    }
}
