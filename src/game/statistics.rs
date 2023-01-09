use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct EnemyKillCount(pub u64);

#[derive(Resource, Default)]
pub struct LaserShotsFired(pub u64);

#[derive(Resource, Default)]
pub struct RocketsFired(pub u64);
