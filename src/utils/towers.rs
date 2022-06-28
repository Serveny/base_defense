use std::time::Duration;

use super::Vec2Board;
use crate::board::{visualisation::BoardVisualisation, Tile};
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

#[derive(Component)]
pub struct TowerRangeHoverCircle(UVec2);

pub trait BoardTower {
    fn draw(&self, cmds: &mut Commands);
}

pub fn draw_tower<TScreen: Component + Copy>(
    cmds: &mut Commands,
    board_visu: &BoardVisualisation<TScreen>,
    pos: Vec2Board,
    tower: &Tower,
) {
    match *tower {
        Tower::LaserShot(_) => spawn_laser_shot_tower(cmds, board_visu, pos),
        Tower::Microwave(_) => todo!(),
        Tower::Rocket(_) => todo!(),
        Tower::Grenade(_) => todo!(),
    };
}

fn spawn_laser_shot_tower<TScreen: Component + Copy>(
    cmds: &mut Commands,
    board_visu: &BoardVisualisation<TScreen>,
    pos: Vec2Board,
) {
    let tile_size = board_visu.inner_tile_size;
    let translation = board_visu.pos_to_px_with_tile_margin(pos, 1.);
    cmds.spawn_bundle(tower_base_shape(tile_size, translation.clone(), Color::RED))
        .with_children(|children| {
            children.spawn_bundle(tower_circle_shape(tile_size, Vec3::new(0., 0., 1.2)));
            children
                .spawn_bundle(tower_laser_cannon(tile_size, Vec3::new(0., 0., 1.1)))
                .insert(TowerCannon);
        })
        .insert(Tower::LaserShot(TowerValues::laser_shot(pos)))
        .insert(board_visu.screen);
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

pub fn set_range_cycle<TScreen: Component + Copy>(
    cmds: &mut Commands,
    board_visu: &BoardVisualisation<TScreen>,
    pos: &Vec2Board,
    tile: &Tile,
    mut query_range_circle: Query<
        (Entity, &mut Transform, &mut TowerRangeHoverCircle),
        With<TowerRangeHoverCircle>,
    >,
) {
    if let Ok((entity, mut trans, mut circle)) = query_range_circle.get_single_mut() {
        println!("{:?} == {:?}", pos.as_uvec2(), &circle.0);
        let pos_uvec = pos.as_uvec2();
        if pos_uvec != circle.0 {
            if let Tile::TowerGround(tower) = tile {
                if let Some(tower) = tower {
                    trans.translation = board_visu.pos_to_px(tower.values().pos, 1.4);
                    circle.0 = pos_uvec;
                } else {
                    cmds.entity(entity).despawn_recursive();
                }
            } else {
                cmds.entity(entity).despawn_recursive();
            }
        }
    } else if let Tile::TowerGround(tower) = tile {
        if let Some(tower) = tower {
            draw_range_circle(cmds, board_visu, tower.values());
        }
    }
}

fn draw_range_circle<TScreen: Component + Copy>(
    cmds: &mut Commands,
    board_visu: &BoardVisualisation<TScreen>,
    vals: &TowerValues,
) {
    println!("{:?}", vals.pos);
    cmds.spawn_bundle(tower_range_circle_shape(
        board_visu.distance_board_to_px(vals.range_radius),
        board_visu.pos_to_px(vals.pos, 1.4),
    ))
    .insert(TowerRangeHoverCircle(vals.pos.as_uvec2()))
    .insert(board_visu.screen);
}

pub fn delete_range_circle(
    cmds: &mut Commands,
    mut query_range_circle: Query<
        (Entity, &mut Transform, &mut TowerRangeHoverCircle),
        With<TowerRangeHoverCircle>,
    >,
) {
    if let Ok(range_cycle) = query_range_circle.get_single_mut() {
        cmds.entity(range_cycle.0).despawn_recursive();
    }
}

fn tower_range_circle_shape(radius: f32, translation: Vec3) -> ShapeBundle {
    let shape = Circle {
        center: Vec2::default(),
        radius: radius,
    };
    println!("{:?}", translation);
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::NONE),
            outline_mode: StrokeMode::new(Color::RED, 2.),
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
