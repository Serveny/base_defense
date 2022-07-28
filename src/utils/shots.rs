use crate::board::step::BoardStep;

use super::buffer::Buffer;
use super::range_circle::RangeCircle;
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
    Rocket(DamageInRadiusTargetPosShotValues),
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

impl DamageInRadiusTargetPosShotValues {
    pub fn new_shot(&self, target_id: Entity) -> DamageInRadiusTargetPosShot {
        DamageInRadiusTargetPosShot {
            target_pos: Vec2Board::default(),
            target_id: Some(target_id),
            vals: self.clone(),
        }
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Target {
    Entity(Entity),
    Pos(Vec2Board),
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DamageInRadiusTargetPosShotValues {
    pub pos: Vec2Board,
    pub damage: f32,
    pub damage_radius: f32,
    pub range_radius: f32,
    pub speed: TilesPerSecond,
    pub fuel: Buffer<Materials>,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DamageInRadiusTargetPosShot {
    pub target_pos: Vec2Board,
    pub target_id: Option<Entity>,
    pub vals: DamageInRadiusTargetPosShotValues,
}

impl DamageInRadiusTargetPosShot {
    pub fn fly_to(&mut self, pos: Vec2Board, frame_dur: Duration) {
        self.target_pos = pos;
        self.fly(frame_dur)
    }

    pub fn fly(&mut self, frame_dur: Duration) {
        let way = self.target_pos - self.pos;
        let distance = self.target_pos.distance(self.pos.into());
        let distance_walked = frame_dur.as_secs_f32() * self.speed;
        self.pos += (way.normalize() * distance_walked.min(distance)).into();
        self.fuel.fill -= distance_walked;
    }

    pub fn set_target_point_to_likely(&mut self, road_path: &[BoardStep]) {
        self.target_id = None;
        let range = RangeCircle::new(self.pos, self.fuel.fill);
        let posis = road_path.iter().map(|step| step.start_pos);
        if let Some(pos) = posis
            .clone()
            .zip(posis.skip(1))
            .find_map(|(vec_start, vec_end)| range.target_point(*vec_start, *vec_end))
        {
            self.target_pos = pos.into();
        }
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

impl Deref for DamageInRadiusTargetPosShot {
    type Target = DamageInRadiusTargetPosShotValues;

    fn deref(&self) -> &Self::Target {
        &self.vals
    }
}

impl DerefMut for DamageInRadiusTargetPosShot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vals
    }
}
