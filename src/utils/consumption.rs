use super::BDVal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Consumption<T: Copy> {
    pub buffer_fill: T,
    pub refill_amount: T,
    pub consume: BDVal<T>,
}

impl<T: Copy> Consumption<T> {
    pub fn new(buffer: T, consume: BDVal<T>) -> Self {
        Self {
            buffer_fill: buffer,
            refill_amount: buffer,
            consume,
        }
    }
}
