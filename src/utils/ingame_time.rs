use bevy::{prelude::*, time::Stopwatch};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration,
};

#[derive(Clone, Debug, Default)]
pub struct IngameTime {
    watch: Stopwatch,
    delta: Duration,
}

impl IngameTime {
    pub fn now(&self) -> IngameTimestamp {
        self.watch.elapsed_secs().into()
    }

    pub fn tick(&mut self, delta: Duration) {
        self.watch.tick(delta);
        self.delta = delta
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn delta_secs(&self) -> f32 {
        self.delta.as_secs_f32()
    }
}

use std::ops::Deref as StdDeref;
impl StdDeref for IngameTime {
    type Target = Stopwatch;

    fn deref(&self) -> &Self::Target {
        &self.watch
    }
}

#[derive(Default, Deref, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct IngameTimestamp(pub f32);

impl Display for IngameTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
