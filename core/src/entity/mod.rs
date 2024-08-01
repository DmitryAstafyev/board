mod comosition;
mod component;
mod connection;
pub mod dummy;
mod port;

use std::fmt::Display;

pub use comosition::*;
pub use component::*;
pub use connection::*;
pub use port::*;
use serde::{Deserialize, Serialize};

const UNKNOWN: &str = "unknown";

pub trait SignatureGetter<'a, 'b: 'a> {
    fn sig(&'b self) -> &'a Signature;
}

pub trait SignatureEqual {
    fn get_if_equal<'a, 'b: 'a, T>(&self, entity: &'b T) -> Option<&'b T>
    where
        T: SignatureGetter<'a, 'b>;
}

impl SignatureEqual for &usize {
    fn get_if_equal<'a, 'b: 'a, T>(&self, entity: &'b T) -> Option<&'b T>
    where
        T: SignatureGetter<'a, 'b>,
    {
        if &&entity.sig().id == self {
            Some(entity)
        } else {
            None
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signature {
    pub id: usize,
    pub class_name: String,
    pub short_name: String,
}
impl Default for Signature {
    fn default() -> Self {
        Self {
            id: 0,
            class_name: "fake".to_string(),
            short_name: "fake".to_string(),
        }
    }
}
impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.short_name != UNKNOWN {
                self.short_name.to_owned()
            } else if self.class_name != UNKNOWN {
                self.class_name.to_owned()
            } else {
                self.id.to_string()
            }
        )
    }
}

impl Signature {
    pub fn as_label(&self, as_short_name: bool, len: usize) -> String {
        if as_short_name {
            let label = self.to_string();
            format!(
                "{:.len$}{}",
                label,
                if label.len() > len { "..." } else { "" }
            )
        } else {
            self.id.to_string()
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct EntityProps {
    class_name: Vec<String>,
    short_name: Vec<String>,
}

impl EntityProps {
    pub fn merge(&mut self, other: Self) {
        self.class_name.extend(other.class_name);
        self.short_name.extend(other.short_name);
    }
    pub fn dedup(mut self) -> Self {
        self.class_name.sort();
        self.class_name.dedup();
        self.short_name.sort();
        self.short_name.dedup();
        self
    }
}
