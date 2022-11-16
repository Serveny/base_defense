use super::{DamageInRadiusTargetPosShot, DamageInRadiusTargetPosShotValues, Shot};
use crate::{
    board::visualisation::TILE_SIZE,
    utils::{
        buffer::Buffer, materials::MATERIALS_COLOR, resource_bar::spawn_resource_bar, Amount,
        Materials, Vec2Board,
    },
};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

pub const INIT_RANGE_RADIUS: f32 = 3.5;

#[derive(Component)]
pub struct RocketShot;

impl Shot {
    pub fn rocket(pos: Vec2Board) -> Self {
        Self::Rocket(DamageInRadiusTargetPosShotValues {
            pos_start: pos,
            pos,
            damage: 100.,
            damage_radius: 0.5,
            range_radius: INIT_RANGE_RADIUS,
            speed: 3.,
            fuel: Buffer::<Materials>::new(5., Amount::PerSecond(1.)),
        })
    }
}

pub fn spawn_shot_rocket<TScreen: Component + Default>(
    cmds: &mut Commands,
    shot: DamageInRadiusTargetPosShot,
) {
    cmds.spawn(SpatialBundle::from_transform(Transform::from_translation(
        Vec3::new(0., 0., 1.),
    )))
    .with_children(|parent| rocket_shot_children::<TScreen>(parent))
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
        Transform::default(),
    )
}

fn rocket_shot_children<TScreen: Component + Default>(parent: &mut ChildBuilder) {
    parent.spawn(rocket_head_shape(TILE_SIZE));
    parent.spawn(rocket_body_shape(TILE_SIZE));
    parent.spawn(rocket_bottom_shape(TILE_SIZE));

    // Fuel bar
    spawn_resource_bar(
        parent,
        TILE_SIZE / 5.,
        Vec2Board::default(),
        MATERIALS_COLOR,
    );
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
