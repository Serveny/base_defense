use std::time::Duration;

use super::BDVal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Consumption<T: Copy + Default> {
    pub buffer_fill: T,
    pub refill_amount: T,
    pub consume: BDVal<T>,
}

impl Consumption<f32> {
    pub fn new(buffer: f32, consume: BDVal<f32>) -> Self {
        Self {
            buffer_fill: buffer,
            refill_amount: buffer,
            consume,
        }
    }

    fn consume(&mut self, amount: f32) -> Option<f32> {
        self.buffer_fill -= amount;

        (self.buffer_fill <= 0.).then(|| {
            self.buffer_fill += self.refill_amount;
            -self.refill_amount
        })
    }

    pub fn consume_at_start(&mut self) -> Option<f32> {
        match self.consume {
            BDVal::PerShot(amount) => self.consume(amount),
            _ => None,
        }
    }

    pub fn consume_during(&mut self, frame_dur: Duration) -> Option<f32> {
        match self.consume {
            BDVal::PerSecond(amount) => self.consume(amount * frame_dur.as_secs_f32()),
            _ => None,
        }
    }
}
