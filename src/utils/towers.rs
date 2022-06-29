use std::time::Duration;

use super::Vec2Board;
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
use euclid::Angle;
use serde::{Deserialize, Serialize};

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
    pub fn laser_shot(pos: Vec2Board) -> Self {
        Self::LaserShot(TowerValues::laser_shot(pos))
    }
    pub fn values(&self) -> &TowerValues {
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
    pub damage: u32,
    pub reload_duration: Duration,
    pub rotation_speed: f32,

    // temp values
    pub current_roation: Angle<f32>,
}

impl TowerValues {
    pub fn laser_shot(pos: Vec2Board) -> Self {
        Self {
            pos,
            range_radius: 2.,
            damage: 1,
            reload_duration: Duration::from_secs(1),
            rotation_speed: 180.,

            current_roation: Angle::default(),
        }
    }
}

#[derive(Component)]
pub struct TowerCannon;

#[derive(Component, Deref, DerefMut)]
pub struct TowerRangeCircle(UVec2);

pub trait BoardTower {
    fn draw(&self, cmds: &mut Commands);
}

pub fn draw_tower<TScreen: Component + Copy>(
    cmds: &mut Commands,
    board_visu: &BoardVisualisation<TScreen>,
    pos: Vec2Board,
    tower: &Tower,
) {
    match tower {
        Tower::LaserShot(values) => spawn_laser_shot_tower(cmds, board_visu, pos, values),
        Tower::Microwave(_) => todo!(),
        Tower::Rocket(_) => todo!(),
        Tower::Grenade(_) => todo!(),
    };
}

fn spawn_laser_shot_tower<TScreen: Component + Copy>(
    cmds: &mut Commands,
    board_visu: &BoardVisualisation<TScreen>,
    pos: Vec2Board,
    tower_values: &TowerValues,
) {
    let tile_size = board_visu.inner_tile_size;
    let translation = board_visu.pos_to_px_with_tile_margin(pos, 1.);
    let color = Color::RED;
    let range_radius_px = board_visu.distance_board_to_px(tower_values.range_radius);
    cmds.spawn_bundle(tower_base_shape(tile_size, translation.clone(), color))
        .with_children(|parent| tower_children(parent, tile_size, range_radius_px, &pos, color))
        .insert(Tower::LaserShot(TowerValues::laser_shot(pos)))
        .insert(board_visu.screen);
}

fn tower_children(
    parent: &mut ChildBuilder,
    tile_size: f32,
    range_radius_px: f32,
    pos: &Vec2Board,
    color: Color,
) {
    // Tower circle
    parent.spawn_bundle(tower_circle_shape(tile_size, Vec3::new(0., 0., 0.2)));

    // Tower cannon
    parent
        .spawn_bundle(tower_laser_cannon(tile_size, Vec3::new(0., 0., 0.1)))
        .insert(TowerCannon);

    // Range circle
    let mut range_circle = tower_range_circle_shape(range_radius_px, Vec3::new(0., 0., 0.2), color);
    range_circle.visibility.is_visible = false;
    parent
        .spawn_bundle(range_circle)
        .insert(TowerRangeCircle(pos.as_uvec2()));
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

fn tower_circle_shape(tile_size: f32, translation: Vec3) -> ShapeBundle {
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
            translation,
            ..Default::default()
        },
    )
}

fn tower_range_circle_shape(radius: f32, translation: Vec3, color: Color) -> ShapeBundle {
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
            translation,
            ..Default::default()
        },
    )
}

fn tower_laser_cannon(tile_size: f32, translation: Vec3) -> ShapeBundle {
    let shape = shapes::Rectangle {
        origin: RectangleOrigin::CustomCenter(Vec2::new(0., tile_size / 4.)),
        extents: Vec2::new(tile_size / 8., tile_size / 2.),
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::SILVER),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, 2.),
        },
        Transform {
            translation,
            ..Default::default()
        },
    )
}
