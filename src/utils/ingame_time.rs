use bevy::{prelude::*, time::Stopwatch};
use serde::{Deserialize, Serialize};
use std::{
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration,
};

#[derive(Deref, DerefMut, Clone, Debug, Default)]
pub struct IngameTime(Stopwatch);

impl IngameTime {
    pub fn now(&self) -> IngameTimestamp {
        self.0.elapsed_secs().into()
    }
}

#[derive(Default, Deref, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct IngameTimestamp(f32);

impl IngameTimestamp {
    pub fn new(now: f32) -> Self {
        Self(now)
    }
}

impl Add<f32> for IngameTimestamp {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        IngameTimestamp(self.0 + rhs)
    }
}

impl AddAssign<f32> for IngameTimestamp {
    fn add_assign(&mut self, rhs: f32) {
        *self = IngameTimestamp(self.0 + rhs);
    }
}

impl Add<Duration> for IngameTimestamp {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        IngameTimestamp(self.0 + rhs.as_secs_f32())
    }
}

impl AddAssign<Duration> for IngameTimestamp {
    fn add_assign(&mut self, rhs: Duration) {
        *self = IngameTimestamp(self.0 + rhs.as_secs_f32());
    }
}

impl Sub<IngameTimestamp> for IngameTimestamp {
    type Output = Self;

    fn sub(self, rhs: IngameTimestamp) -> Self::Output {
        IngameTimestamp(self.0 - rhs.0)
    }
}

impl Sub<f32> for IngameTimestamp {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        IngameTimestamp(self.0 - rhs)
    }
}

impl Sub<Duration> for IngameTimestamp {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        IngameTimestamp(self.0 - rhs.as_secs_f32())
    }
}

impl SubAssign<IngameTimestamp> for IngameTimestamp {
    fn sub_assign(&mut self, rhs: IngameTimestamp) {
        *self = IngameTimestamp(self.0 - rhs.0);
    }
}

impl From<f32> for IngameTimestamp {
    fn from(val: f32) -> Self {
        Self(val)
    }
}
