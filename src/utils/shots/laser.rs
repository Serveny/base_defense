use super::{DamagePerTimeShot, DamagePerTimeShotValues, Shot};
use crate::{board::visualisation::TILE_SIZE, utils::Vec2Board};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use std::time::Duration;

pub const INIT_RANGE_RADIUS: f32 = 1.5;
pub const INIT_SHOT_DURATION_SECS: f32 = 1.;

#[derive(Component)]
pub struct LaserShot;

impl Shot {
    pub fn laser_vals(pos_start: Vec2Board) -> Self {
        Self::Laser(DamagePerTimeShotValues {
            damage: 100.,
            lifetime: Duration::from_secs_f32(INIT_SHOT_DURATION_SECS),
            range_radius: INIT_RANGE_RADIUS,
            pos_start,
        })
    }
}

pub fn spawn_shot_laser<TScreen: Component + Default>(
    cmds: &mut Commands,
    shot: DamagePerTimeShot,
) {
    cmds.spawn(laser_shape(TILE_SIZE))
        .insert(shot)
        .insert(LaserShot)
        .insert(TScreen::default());
}

fn laser_shape(tile_size: f32) -> impl Bundle {
    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                origin: RectangleOrigin::CustomCenter(Vec2::new(0., tile_size / 2.)),
                extents: Vec2::new(tile_size / 10., tile_size),
            }),
            transform: Transform {
                scale: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            ..default()
        },
        Fill::color(Color::Rgba {
            red: 1.,
            green: 1.,
            blue: 1.,
            alpha: 0.6,
        }),
        Stroke::new(
            Color::Rgba {
                red: 1.,
                green: 0.,
                blue: 0.,
                alpha: 0.6,
            },
            tile_size / 20.,
        ),
    )
}
