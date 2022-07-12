use super::{IngameTimestamp, TilesPerSecond};
use crate::board::visualisation::TILE_SIZE;
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

pub const EXPLOSION_SPEED: TilesPerSecond = 2.;

#[derive(Component)]
pub struct Explosion {
    die_time: IngameTimestamp,
}

impl Explosion {
    pub fn new(die_time: IngameTimestamp) -> Self {
        Self { die_time }
    }
}

pub fn spawn_explosion<TScreen: Component + Default>(
    cmds: &mut Commands,
    radius: f32,
    die_time: IngameTimestamp,
) {
    cmds.spawn_bundle(explosion_shape(TILE_SIZE))
        .insert(Explosion::new(die_time))
        .insert(TScreen::default());
}

fn explosion_shape(radius: f32) -> ShapeBundle {
    let shape = shapes::Circle {
        center: Vec2::default(),
        radius: radius / 2.,
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::Rgba {
                red: 1.,
                green: 0.,
                blue: 0.,
                alpha: 0.9,
            }),
            outline_mode: StrokeMode::new(
                Color::Rgba {
                    red: 1.,
                    green: 1.,
                    blue: 0.,
                    alpha: 0.7,
                },
                radius / 2.,
            ),
        },
        Transform {
            scale: Vec3::new(0., 0., 0.),
            ..Default::default()
        },
    )
}
