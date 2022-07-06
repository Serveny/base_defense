use super::{DamagePerTimeShot, Shot};
use crate::{board::visualisation::TILE_SIZE, utils::Vec2Board};
use bevy::{prelude::*, reflect::Uuid};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use std::time::Duration;

pub const INIT_RANGE_RADIUS: f32 = 2.;
pub const INIT_SHOT_DURATION_SECS: f32 = 1.;

impl Shot {
    pub fn laser(pos_start: Vec2Board) -> Self {
        Self::Laser(DamagePerTimeShot {
            target_enemy_id: Uuid::default(),
            damage: 100.,
            lifetime: Duration::from_secs_f32(INIT_SHOT_DURATION_SECS),
            die_time: None,
            range_radius: INIT_RANGE_RADIUS,
            pos_start,
        })
    }
}

pub fn spawn_shot_laser<TScreen: Component + Default>(cmds: &mut Commands, shot: &Shot) {
    cmds.spawn_bundle(laser_shape(TILE_SIZE))
        .insert(shot.clone())
        .insert(TScreen::default());
}

fn laser_shape(tile_size: f32) -> ShapeBundle {
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
