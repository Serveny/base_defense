use crate::board::visualisation::BoardVisualisation;
use bevy::prelude::*;
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{
        DrawMode, FillMode, GeometryBuilder, RectangleOrigin, RegularPolygon,
        RegularPolygonFeature, StrokeMode,
    },
    shapes::{self, Circle},
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::{
    shots::{Shot, TowerStatus},
    IngameTimestamp, Vec2Board,
};

//pub struct Tower {
//tower_type: TowerType,
//shot_con: Consumption,
//}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Component)]
pub enum Tower {
    // Damages enemies, needs energy
    LaserShot(TowerValues),

    // Slows enemies down, needs energy
    Microwave(TowerValues),

    // Damages enemies, needs energy and material
    Rocket(TowerValues),

    // Damages enemies, needs energy and material
    Grenade(TowerValues),
}

impl Tower {
    pub fn laser_shot(pos: Vec2Board, now: IngameTimestamp) -> Self {
        Self::LaserShot(TowerValues::laser_shot(pos, now))
    }

    //    pub fn values(&self) -> &TowerValues {
    //match self {
    //Tower::LaserShot(values) => values,
    //Tower::Microwave(values) => values,
    //Tower::Rocket(values) => values,
    //Tower::Grenade(values) => values,
    //}
    //}

    pub fn values_mut(&mut self) -> &mut TowerValues {
        match self {
            Tower::LaserShot(values) => values,
            Tower::Microwave(values) => values,
            Tower::Rocket(values) => values,
            Tower::Grenade(values) => values,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TowerValues {
    pub pos: Vec2Board,
    pub range_radius: f32,
    pub shot: Shot,
    pub reload_duration: Duration,

    // temp values
    pub target_lock: Option<Entity>,
    pub tower_status: TowerStatus,
}

impl TowerValues {
    pub fn laser_shot(pos: Vec2Board, now: IngameTimestamp) -> Self {
        let reload_duration = Duration::from_secs(1);
        Self {
            pos,
            range_radius: 2.,
            shot: Shot::laser(),
            reload_duration,

            target_lock: None,
            tower_status: TowerStatus::Reloading(now + reload_duration),
        }
    }
    pub fn shoot(&self, target: Vec2Board) -> Shot {
        let mut shot = self.shot.clone();
        shot.set_target(target);
        shot
    }
}

#[derive(Component)]
pub struct TowerCannon;

#[derive(Component, Deref, DerefMut)]
pub struct TowerRangeCircle(UVec2);

pub trait BoardTower {
    fn draw(&self, cmds: &mut Commands);
}

pub fn draw_tower<TScreen: Component + Copy + Default>(
    cmds: &mut Commands,
    board_visu: &BoardVisualisation<TScreen>,
    pos: Vec2Board,
    tower: &Tower,
    time: IngameTimestamp,
) {
    match tower {
        Tower::LaserShot(values) => spawn_laser_shot_tower(cmds, board_visu, pos, values, time),
        Tower::Microwave(_) => todo!(),
        Tower::Rocket(_) => todo!(),
        Tower::Grenade(_) => todo!(),
    };
}

fn spawn_laser_shot_tower<TScreen: Component + Copy + Default>(
    cmds: &mut Commands,
    board_visu: &BoardVisualisation<TScreen>,
    pos: Vec2Board,
    tower_values: &TowerValues,
    time: IngameTimestamp,
) {
    let tile_size = board_visu.inner_tile_size;
    let color = Color::RED;
    cmds.spawn_bundle(tower_base_shape(tile_size, pos.to_vec3(1.), color))
        .with_children(|parent| {
            laser_tower_children(parent, tile_size, tower_values.range_radius, &pos, color);
        })
        .insert(Tower::LaserShot(TowerValues::laser_shot(pos, time)))
        .insert(TScreen::default());
}

fn laser_tower_children(
    parent: &mut ChildBuilder,
    tile_size: f32,
    range_radius_px: f32,
    pos: &Vec2Board,
    color: Color,
) {
    // Tower circle
    parent.spawn_bundle(tower_circle_shape(tile_size));

    // Tower cannon
    parent
        .spawn_bundle(tower_laser_cannon(tile_size))
        .insert(TowerCannon);

    // Range circle
    let mut range_circle = tower_range_circle_shape(range_radius_px, color);
    range_circle.visibility.is_visible = false;
    parent
        .spawn_bundle(range_circle)
        .insert(TowerRangeCircle(pos.as_uvec2()));

    // Laser
}

fn tower_base_shape(tile_size: f32, translation: Vec3, color: Color) -> ShapeBundle {
    let shape = RegularPolygon {
        sides: 8,
        feature: RegularPolygonFeature::Radius(tile_size / 2.4),
        ..RegularPolygon::default()
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(color),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, tile_size / 16.),
        },
        Transform {
            translation,
            ..Default::default()
        },
    )
}

fn tower_circle_shape(tile_size: f32) -> ShapeBundle {
    let shape = Circle {
        center: Vec2::default(),
        radius: tile_size / 5.,
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::SILVER),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, tile_size / 16.),
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
        radius: radius,
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::NONE),
            outline_mode: StrokeMode::new(color, 2.),
        },
        Transform {
            translation: Vec3::new(0., 0., 0.3),
            ..Default::default()
        },
    )
}

fn tower_laser_cannon(tile_size: f32) -> ShapeBundle {
    let shape = shapes::Rectangle {
        origin: RectangleOrigin::CustomCenter(Vec2::new(0., -tile_size / 4.)),
        extents: Vec2::new(tile_size / 6., tile_size / 2.),
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::SILVER),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, tile_size / 16.),
        },
        Transform {
            translation: Vec3::new(0., 0., 0.1),
            rotation: Quat::from_rotation_z(0.),
            ..Default::default()
        },
    )
}
