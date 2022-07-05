use std::time::Duration;

use super::{
    towers::{LASER_TOWER_INIT_RANGE_RADIUS, LASER_TOWER_INIT_SHOT_DURATION_SECS},
    IngameTimestamp, Vec2Board,
};
use bevy::{prelude::*, reflect::Uuid};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TowerStatus {
    Reloading(IngameTimestamp),
    Waiting,
    Shooting(IngameTimestamp),
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Shot {
    Laser(DamagePerTimeShot),
}

pub enum Target {
    Pos(Vec2Board),
    Enemy(Uuid),
}

pub type DamagePerSecond = f32;

impl Shot {
    pub fn laser(pos_start: Vec2Board) -> Self {
        Self::Laser(DamagePerTimeShot {
            target_enemy_id: Uuid::default(),
            damage: 100.,
            lifetime: Duration::from_secs_f32(LASER_TOWER_INIT_SHOT_DURATION_SECS),
            die_time: None,
            range_radius: LASER_TOWER_INIT_RANGE_RADIUS,
            pos_start,
        })
    }

    pub fn set_target(&mut self, target: Target) {
        match self {
            Shot::Laser(shot) => {
                if let Target::Enemy(id) = target {
                    shot.target_enemy_id = id;
                } else {
                    panic!("Wrong target type. Laser tower needs target id.");
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

pub fn laser_shape(tile_size: f32) -> ShapeBundle {
    let shape = shapes::Rectangle {
        origin: RectangleOrigin::CustomCenter(Vec2::new(0., tile_size / 2.)),
        extents: Vec2::new(tile_size / 10., tile_size),
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::Rgba {
                red: 1.,
                green: 1.,
                blue: 1.,
                alpha: 0.6,
            }),
            outline_mode: StrokeMode::new(
                Color::Rgba {
                    red: 1.,
                    green: 0.,
                    blue: 0.,
                    alpha: 0.6,
                },
                tile_size / 20.,
            ),
        },
        Transform {
            scale: Vec3::new(0., 0., 0.),
            ..Default::default()
        },
    )
}
