mod comosition;
mod component;
mod port;

use crate::entity::Signature;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureProducer {
    current: usize,
}

impl SignatureProducer {
    pub fn new(current: usize) -> Self {
        Self { current }
    }
    // This method is used for testing only with Dummy<T>
    pub fn next(&mut self) -> Signature {
        self.current += 1;
        Signature {
            id: self.current,
            class_name: format!("DummyClass_{}", self.current),
            short_name: format!("DummySN_{}", self.current),
        }
    }

    pub fn next_for(&mut self, class_name: &str) -> Signature {
        self.current += 1;
        Signature {
            id: self.current,
            class_name: class_name.to_string(),
            short_name: class_name.to_string(),
        }
    }
}

pub trait Dummy<T, O> {
    fn dummy(producer: &mut SignatureProducer, options: O) -> T;
}
