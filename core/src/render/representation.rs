use crate::{
    entity::{Signature, SignatureGetter},
    error::E,
    render::Render,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::large_enum_variant)]
pub enum Representation<T> {
    Origin(T),
    #[serde(skip_serializing, skip_deserializing)]
    Render(Render<T>),
}

impl<'a, 'b: 'a, T: SignatureGetter<'a, 'b>> Representation<T> {
    pub fn sig(&'b self) -> &'a Signature {
        match self {
            Self::Origin(t) => t.sig(),
            Self::Render(r) => r.origin().sig(),
        }
    }
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
