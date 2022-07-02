use super::{IngameTimestamp, Vec2Board};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TowerStatus {
    Reloading(IngameTimestamp),
    Waiting,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Shot {
    Laser(LaserShot),
}

pub type DamageRadiusBoard = f32;
pub type DamagePerSecond = f32;

impl Shot {
    pub fn laser() -> Self {
        Self::Laser(LaserShot {
            target: Vec2Board::default(),
            damage_radius: 0.1,
            damage: 10.,
        })
    }
    pub fn set_target(&mut self, target: Vec2Board) {
        match self {
            Shot::Laser(shot) => shot.target = target,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaserShot {
    pub target: Vec2Board,
    pub damage_radius: DamageRadiusBoard,
    pub damage: DamagePerSecond,
}

pub fn laser_shape(tile_size: f32) -> ShapeBundle {
    let shape = shapes::Rectangle {
        origin: RectangleOrigin::CustomCenter(Vec2::new(0., -tile_size / 2.)),
        extents: Vec2::new(tile_size / 6., tile_size),
    };
    let mut shape_bundle = GeometryBuilder::build_as(
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
                tile_size / 16.,
            ),
        },
        Transform {
            translation: Vec3::new(0., 0., -0.1),
            ..Default::default()
        },
    );
    shape_bundle.visibility.is_visible = false;
    shape_bundle
}
