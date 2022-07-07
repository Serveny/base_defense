use super::{IngameTimestamp, TilesPerSecond, Vec2Board};
use bevy::{prelude::*, reflect::Uuid};
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
    Laser(DamagePerTimeShot),
    Rocket(DamageInRadiusEnemyLockedShot),
}

pub enum Target {
    Pos(Vec2Board),
    Enemy(Uuid),
}

pub type DamagePerSecond = f32;
pub type InstantDamage = f32;

impl Shot {
    pub fn set_target(&mut self, target: Target) {
        match self {
            Shot::Laser(shot) => {
                if let Target::Enemy(id) = target {
                    shot.target_enemy_id = id;
                } else {
                    panic!("Wrong target type. DamagePerTimeShot needs target id.");
                }
            }
            Shot::Rocket(shot) => {
                if let Target::Enemy(id) = target {
                    shot.target_enemy_id = id;
                } else {
                    panic!("Wrong target type. DamageInRadiusEnemyLocked needs target id.");
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DamagePerTimeShot {
    pub target_enemy_id: Uuid,
    pub damage: DamagePerSecond,
    pub lifetime: Duration,
    pub die_time: Option<IngameTimestamp>,
    pub range_radius: f32,
    pub pos_start: Vec2Board,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DamageInRadiusEnemyLockedShot {
    pub target_enemy_id: Uuid,
    pub pos: Vec2Board,
    pub damage: InstantDamage,
    pub damage_radius: f32,
    pub range_radius: f32,
    pub speed: TilesPerSecond,
}
