use num_traits::Num;
use serde::{Deserialize, Serialize};
use std::convert::Into;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ratio {
    pub ratio: u8,
}

impl Ratio {
    pub fn get<T: Num + From<u8> + Copy>(&self, value: T) -> T {
        value * self.ratio.into()
    }
    pub fn invert<T: Num + From<u8> + Copy>(&self, value: T) -> T {
        value / self.ratio.into()
    }
}
