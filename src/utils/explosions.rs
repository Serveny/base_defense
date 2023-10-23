use std::time::Duration;

use super::{TilesPerSecond, Vec2Board};
use crate::board::visualisation::TILE_SIZE;
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

pub const EXPLOSION_SPEED: TilesPerSecond = 8.;

#[derive(Component)]
pub struct Explosion {
    pub pos: Vec2Board,
    pub target_radius: f32,
    pub current_radius: f32,
    pub damage: f32,
}

impl Explosion {
    pub fn new(pos: Vec2Board, radius: f32, damage: f32) -> Self {
        Self {
            pos,
            target_radius: radius,
            current_radius: 0.,
            damage,
        }
    }

    pub fn is_end(&self) -> bool {
        self.current_radius >= self.target_radius
    }

    pub fn grow(&mut self, frame_dur: Duration) {
        self.current_radius += EXPLOSION_SPEED * frame_dur.as_secs_f32();
    }
}

pub fn spawn_explosion<TScreen: Component + Default>(cmds: &mut Commands, expl: Explosion) {
    cmds.spawn(explosion_shape(expl.target_radius * TILE_SIZE, expl.pos))
        .insert(expl)
        .insert(TScreen::default());
}

fn explosion_shape(radius: f32, pos: Vec2Board) -> impl Bundle {
    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle {
                center: Vec2::default(),
                radius: radius / 2.,
            }),
            transform: Transform {
                translation: pos.to_scaled_vec3(3.),
                scale: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            ..default()
        },
        Fill::color(Color::Rgba {
            red: 1.,
            green: 0.,
            blue: 0.,
            alpha: 0.9,
        }),
        Stroke::new(
            Color::Rgba {
                red: 1.,
                green: 1.,
                blue: 0.,
                alpha: 0.7,
            },
            radius / 2.,
        ),
    )
}
