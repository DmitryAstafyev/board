use crate::{error::E, render::Render};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Representation<T> {
    Origin(T),
    #[serde(skip_serializing, skip_deserializing)]
    Render(Render<T>),
}

impl<T> Representation<T> {
    pub fn origin(&self) -> &T {
        match self {
            Self::Origin(t) => t,
            Self::Render(r) => r.origin(),
        }
    }

    pub fn origin_mut(&mut self) -> &mut T {
        match self {
            Self::Origin(t) => t,
            Self::Render(r) => r.origin_mut(),
        }
    }

    pub fn render(&self) -> Result<&Render<T>, E> {
        if let Self::Render(r) = self {
            Ok(r)
        } else {
            Err(E::RenderNotInited)
        }
    }

    pub fn render_mut(&mut self) -> Result<&mut Render<T>, E> {
        if let Self::Render(r) = self {
            Ok(r)
        } else {
            Err(E::RenderNotInited)
        }
    }
}
