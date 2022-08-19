use std::time::Duration;

use super::Amount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Buffer<T: Copy + Default> {
    pub fill: T,
    pub size: T,
    pub package: Option<Amount<T>>,
}

impl Buffer<f32> {
    pub fn new(buffer_size: f32, package: Amount<f32>) -> Self {
        Self {
            fill: buffer_size,
            size: buffer_size,
            package: Some(package),
        }
    }

    fn change(&mut self, amount: f32, multi: f32) -> Option<f32> {
        self.fill += amount * multi;

        (self.fill <= 0. || self.fill >= self.size).then(|| {
            self.fill += self.size * -multi;
            self.size * multi
        })
    }

    pub fn produce_during(&mut self, frame_dur: Duration) -> Option<f32> {
        match self.package {
            Some(Amount::PerSecond(amount)) => self.change(amount * frame_dur.as_secs_f32(), 1.),
            _ => None,
        }
    }

    pub fn consume_during(&mut self, frame_dur: Duration) -> Option<f32> {
        match self.package {
            Some(Amount::PerSecond(amount)) => self.change(amount * frame_dur.as_secs_f32(), -1.),
            _ => None,
        }
    }

    pub fn percent(&self) -> f32 {
        self.fill / self.size
    }
}
