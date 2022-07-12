use super::{IngameTimestamp, Materials, TilesPerSecond, Vec2Board};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
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

pub type DamagePerSecond = f32;
pub type InstantDamage = f32;

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DamagePerTimeShotValues {
    pub damage: DamagePerSecond,
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

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DamageInRadiusEnemyLockedShotValues {
    pub pos: Vec2Board,
    pub damage: InstantDamage,
    pub damage_radius: f32,
    pub range_radius: f32,
    pub speed: TilesPerSecond,
    pub fuel: Materials,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DamageInRadiusEnemyLockedShot {
    pub target_enemy_id: Entity,
    pub vals: DamageInRadiusEnemyLockedShotValues,
}
