use super::{IngameTimestamp, Materials, TilesPerSecond, Vec2Board};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::time::Duration;

pub mod laser;
pub mod rocket;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TowerStatus {
    Reloading(IngameTimestamp),
    Waiting,
    Shooting(IngameTimestamp),
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Shot {
    Laser(DamagePerTimeShotValues),
    Rocket(DamageInRadiusEnemyLockedShotValues),
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DamagePerTimeShotValues {
    pub damage: f32,
    pub lifetime: Duration,
    pub pos_start: Vec2Board,
    pub range_radius: f32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DamagePerTimeShot {
    pub target_enemy_id: Entity,
    pub die_time: IngameTimestamp,
    pub vals: DamagePerTimeShotValues,
}

impl DamagePerTimeShotValues {
    pub fn new_shot(
        &self,
        target_enemy_id: Entity,
        die_time: IngameTimestamp,
    ) -> DamagePerTimeShot {
        DamagePerTimeShot {
            target_enemy_id,
            die_time,
            vals: self.clone(),
        }
    }
}

impl DamageInRadiusEnemyLockedShotValues {
    pub fn new_shot(&self, target_enemy_id: Entity) -> DamageInRadiusEnemyLockedShot {
        DamageInRadiusEnemyLockedShot {
            target_enemy_id,
            vals: self.clone(),
        }
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DamageInRadiusEnemyLockedShotValues {
    pub pos: Vec2Board,
    pub damage: f32,
    pub damage_radius: f32,
    pub range_radius: f32,
    pub speed: TilesPerSecond,
    pub fuel: Materials,
    pub fuel_max: Materials,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DamageInRadiusEnemyLockedShot {
    pub target_enemy_id: Entity,
    pub vals: DamageInRadiusEnemyLockedShotValues,
}

impl DamageInRadiusEnemyLockedShot {
    pub fn fly(&mut self, pos: Vec2Board, frame_dur: Duration) {
        let way = pos - self.pos;
        let distance = pos.distance(self.pos.into());
        let distance_walked = frame_dur.as_secs_f32() * self.speed;
        self.pos += (way.normalize() * distance_walked.min(distance)).into();
        self.fuel -= distance_walked;
    }

    pub fn fuel_as_percent(&self) -> f32 {
        self.fuel / self.fuel_max
    }
}

impl Deref for DamagePerTimeShot {
    type Target = DamagePerTimeShotValues;

    fn deref(&self) -> &Self::Target {
        &self.vals
    }
}

impl DerefMut for DamagePerTimeShot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vals
    }
}

impl Deref for DamageInRadiusEnemyLockedShot {
    type Target = DamageInRadiusEnemyLockedShotValues;

    fn deref(&self) -> &Self::Target {
        &self.vals
    }
}

impl DerefMut for DamageInRadiusEnemyLockedShot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vals
    }
}
