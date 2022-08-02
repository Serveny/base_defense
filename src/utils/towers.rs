use self::{laser::spawn_laser_tower, rocket::spawn_rocket_tower};
use super::{
    buffer::Buffer,
    shots::{Shot, TowerStatus},
    Energy, Materials, Vec2Board,
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
    pub energy: Buffer<Energy>,
    pub materials: Buffer<Materials>,

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

fn tower_base_shape(color: Color) -> ShapeBundle {
    let shape = RegularPolygon {
        sides: 8,
        feature: RegularPolygonFeature::Radius(TILE_SIZE / 2.4),
        ..RegularPolygon::default()
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(color),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, TILE_SIZE / 16.),
        },
        Transform::default(),
    )
}

fn tower_circle_shape() -> ShapeBundle {
    let shape = Circle {
        center: Vec2::default(),
        radius: TILE_SIZE / 5.,
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::SILVER),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, TILE_SIZE / 16.),
        },
        Transform {
            translation: Vec3::new(0., 0., 0.2),
            ..Default::default()
        },
    )
}

fn tower_range_circle_shape(radius: f32, color: Color) -> ShapeBundle {
    let shape = Circle {
        center: Vec2::default(),
        radius: radius * TILE_SIZE,
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::NONE),
            outline_mode: StrokeMode::new(color, 0.025 * TILE_SIZE),
        },
        Transform::from_translation(Vec3::new(0., 0., 0.3)),
    )
}
