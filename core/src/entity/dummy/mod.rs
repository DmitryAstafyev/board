mod comosition;
mod component;
mod port;

use crate::entity::Signature;

#[derive(Debug)]
pub struct SignatureProducer {
    current: usize,
}

impl SignatureProducer {
    pub fn new() -> Self {
        Self { current: 0 }
    }
    // This method is used for testing only with Dummy<T>
    pub fn next(&mut self) -> Signature {
        self.current += 1;
        Signature {
            id: self.current,
            class_name: format!("DummyClass_{}", self.current),
        }
    }
}

pub trait Dummy<T, O> {
    fn dummy(producer: &mut SignatureProducer, options: O) -> T;
}
