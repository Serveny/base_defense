use super::{DamageInRadiusEnemyLockedShot, DamageInRadiusEnemyLockedShotValues, Shot};
use crate::{
    board::visualisation::TILE_SIZE,
    utils::{resource_bar::resource_bar, Vec2Board},
};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

pub const INIT_RANGE_RADIUS: f32 = 3.5;

#[derive(Component)]
pub struct RocketShot;

impl Shot {
    pub fn rocket(pos: Vec2Board) -> Self {
        Self::Rocket(DamageInRadiusEnemyLockedShotValues {
            pos,
            damage: 100.,
            damage_radius: 0.5,
            range_radius: INIT_RANGE_RADIUS,
            speed: 3.,
            fuel: 5.,
            fuel_max: 5.,
        })
    }
}
pub fn spawn_shot_rocket<TScreen: Component + Default>(
    cmds: &mut Commands,
    shot: DamageInRadiusEnemyLockedShot,
) {
    cmds.spawn_bundle(rocket_body_shape(TILE_SIZE))
        .with_children(rocket_shot_children)
        .insert(shot)
        .insert(RocketShot)
        .insert(TScreen::default());
}

fn rocket_body_shape(tile_size: f32) -> ShapeBundle {
    let shape = shapes::Rectangle {
        origin: RectangleOrigin::Center,
        // origin: RectangleOrigin::CustomCenter(Vec2::new(0., tile_size / 2.)),
        extents: Vec2::new(tile_size / 10., tile_size / 5.),
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::PURPLE),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, tile_size / 40.),
        },
        Transform {
            ..Default::default()
        },
    )
}

fn rocket_shot_children(parent: &mut ChildBuilder) {
    // Head
    parent.spawn_bundle(rocket_head_shape(TILE_SIZE));

    // Body
    parent.spawn_bundle(rocket_bottom_shape(TILE_SIZE));

    // Fuel bar
    resource_bar(parent, TILE_SIZE / 5.);
}

fn rocket_head_shape(tile_size: f32) -> ShapeBundle {
    let shape = RegularPolygon {
        sides: 3,
        feature: RegularPolygonFeature::Radius(tile_size / 20.),
        ..RegularPolygon::default()
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::PURPLE),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, tile_size / 40.),
        },
        Transform {
            translation: Vec3::new(0., tile_size / 10. + tile_size / 20., 0.1),
            ..Default::default()
        },
    )
}

fn rocket_bottom_shape(tile_size: f32) -> ShapeBundle {
    let shape = RegularPolygon {
        sides: 3,
        feature: RegularPolygonFeature::Radius(tile_size / 10.),
        ..RegularPolygon::default()
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::PURPLE),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, tile_size / 20.),
        },
        Transform {
            translation: Vec3::new(0., -tile_size / 10., -0.1),
            ..Default::default()
        },
    )
}
