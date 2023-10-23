use self::{laser::spawn_laser_tower, rocket::spawn_rocket_tower};
use super::{
    shots::{Shot, TowerStatus},
    Vec2Board,
};
use crate::board::visualisation::TILE_SIZE;
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use strum::EnumDiscriminants;

mod laser;
mod rocket;

//pub struct Tower {
//tower_type: TowerType,
//shot_con: Consumption,
//}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Component, EnumDiscriminants)]
#[strum_discriminants(derive(Component))]
#[strum_discriminants(name(Towerless))]
pub enum Tower {
    // Damages enemies, needs energy
    Laser(TowerValues),

    // Slows enemies down, needs energy
    Microwave(TowerValues),

    // Damages enemies, needs energy and material
    Rocket(TowerValues),

    // Damages enemies, needs energy and material
    Grenade(TowerValues),
}

impl Tower {
    pub fn values(&self) -> &TowerValues {
        match self {
            Tower::Laser(values) => values,
            Tower::Microwave(values) => values,
            Tower::Rocket(values) => values,
            Tower::Grenade(values) => values,
        }
    }

    pub fn values_mut(&mut self) -> &mut TowerValues {
        match self {
            Tower::Laser(values) => values,
            Tower::Microwave(values) => values,
            Tower::Rocket(values) => values,
            Tower::Grenade(values) => values,
        }
    }

    pub fn draw_preview<TScreen: Component + Default>(&self, cmds: &mut Commands) {
        match self {
            Tower::Laser(values) => spawn_laser_tower::<TScreen>(cmds, values.clone(), true),
            Tower::Microwave(_) => todo!(),
            Tower::Rocket(values) => spawn_rocket_tower::<TScreen>(cmds, values.clone(), true),
            Tower::Grenade(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TowerValues {
    pub pos: Vec2Board,
    pub range_radius: f32,
    pub shot: Shot,
    pub reload_duration: Duration,
    pub shoot_duration: Duration,

    // temp values
    pub target_lock: Option<Entity>,
    pub tower_status: TowerStatus,
}

impl TowerValues {
    pub fn clone_with_pos(&self, pos: Vec2Board) -> Self {
        let mut new_vals = self.clone();
        new_vals.pos = pos;
        match &mut new_vals.shot {
            Shot::Laser(shot) => shot.pos_start = pos,
            Shot::Rocket(shot) => shot.pos = pos,
        };
        new_vals
    }
}

#[derive(Component)]
pub struct TowerCannon;

#[derive(Component)]
pub struct TowerParent;

#[derive(Component, Deref, DerefMut)]
pub struct TowerRangeCircle(UVec2);

pub trait BoardTower {
    fn draw(&self, cmds: &mut Commands);
}

pub fn draw_tower<TScreen: Component + Default>(
    cmds: &mut Commands,
    pos: Vec2Board,
    tower: &Tower,
) {
    match tower {
        Tower::Laser(vals) => spawn_laser_tower::<TScreen>(cmds, vals.clone_with_pos(pos), false),
        Tower::Microwave(_) => todo!(),
        Tower::Rocket(vals) => spawn_rocket_tower::<TScreen>(cmds, vals.clone_with_pos(pos), false),
        Tower::Grenade(_) => todo!(),
    };
}

fn tower_base_shape(color: Color) -> impl Bundle {
    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&RegularPolygon {
                sides: 8,
                feature: RegularPolygonFeature::Radius(TILE_SIZE / 2.4),
                ..RegularPolygon::default()
            }),

            ..default()
        },
        Fill::color(color),
        Stroke::new(Color::DARK_GRAY, TILE_SIZE / 16.),
    )
}

fn tower_circle_shape() -> impl Bundle {
    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&Circle {
                center: Vec2::default(),
                radius: TILE_SIZE / 5.,
            }),

            transform: Transform::from_xyz(0., 0., 0.4),
            ..default()
        },
        Fill::color(Color::SILVER),
        Stroke::new(Color::DARK_GRAY, TILE_SIZE / 16.),
    )
}

fn tower_range_circle_shape(radius: f32, color: Color, visibility: Visibility) -> impl Bundle {
    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&Circle {
                center: Vec2::default(),
                radius,
            }),
            visibility,
            transform: Transform::from_xyz(0., 0., 0.3),
            ..default()
        },
        Fill::color(Color::NONE),
        Stroke::new(color, 0.025 * TILE_SIZE),
    )
}
