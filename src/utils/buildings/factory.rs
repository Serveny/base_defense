use std::time::Duration;

use super::{building_base_shape, Building, BuildingBase};
use crate::utils::{materials::MATERIALS_COLOR, Energy, Materials, Vec2Board};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use euclid::Angle;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Factory {
    materials_package_size: Materials,
    energy_consumption: Energy,
    production_time: Duration,
}

impl Building {
    pub fn factory() -> Self {
        Self::Factory(Factory::new())
    }
}

impl Factory {
    pub fn new() -> Self {
        Self {
            materials_package_size: 5.,
            energy_consumption: 1.,
            production_time: Duration::from_secs(5),
        }
    }
}

pub fn spawn_factory<TScreen: Component + Default>(
    cmds: &mut Commands,
    factory: Factory,
    tile_size: f32,
    pos: Vec2Board,
) {
    let color = MATERIALS_COLOR;
    cmds.spawn_bundle(building_base_shape(
        pos.to_scaled_vec3(1.),
        tile_size / 1.1,
        color,
    ))
    .with_children(|parent| factory_children::<TScreen>(parent, tile_size))
    .insert(BuildingBase)
    .insert(Building::Factory(factory))
    .insert(TScreen::default());
}

fn factory_children<TScreen: Component + Default>(parent: &mut ChildBuilder, tile_size: f32) {
    parent
        .spawn_bundle(factory_building_shape(tile_size, Color::GRAY))
        .insert(TScreen::default());
    parent
        .spawn_bundle(factory_roof_shape(
            tile_size,
            Color::GRAY,
            Vec3::new(-tile_size / 6., tile_size / 5., 0.1),
        ))
        .insert(TScreen::default());
    parent
        .spawn_bundle(factory_roof_shape(
            tile_size,
            Color::GRAY,
            Vec3::new(0., tile_size / 5., 0.1),
        ))
        .insert(TScreen::default());
    parent
        .spawn_bundle(factory_roof_shape(
            tile_size,
            Color::GRAY,
            Vec3::new(tile_size / 6., tile_size / 5., 0.1),
        ))
        .insert(TScreen::default());
    parent
        .spawn_bundle(factory_chimney_shape(
            tile_size,
            Color::GRAY,
            Vec3::new(tile_size / 6., -tile_size / 4., 0.09),
        ))
        .insert(TScreen::default());
}

fn factory_roof_shape(tile_size: f32, color: Color, translation: Vec3) -> ShapeBundle {
    let shape = shapes::RegularPolygon {
        sides: 3,
        feature: RegularPolygonFeature::Radius(tile_size / 10.),
        ..default()
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(color),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, tile_size / 40.),
        },
        Transform {
            translation,
            rotation: Quat::from_rotation_z(Angle::degrees(0.).radians),
            ..default()
        },
    )
}

fn factory_chimney_shape(tile_size: f32, color: Color, translation: Vec3) -> ShapeBundle {
    let shape = shapes::Rectangle {
        origin: RectangleOrigin::CustomCenter(Vec2::new(0., tile_size / 2.)),
        extents: Vec2::new(tile_size / 10., tile_size / 2.),
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(color),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, tile_size / 20.),
        },
        Transform {
            translation,
            ..default()
        },
    )
}

fn factory_building_shape(tile_size: f32, color: Color) -> ShapeBundle {
    let shape = shapes::Rectangle {
        origin: RectangleOrigin::Center,
        extents: Vec2::new(tile_size / 1.75, tile_size / 3.),
    };

    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(color),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, tile_size / 20.),
        },
        Transform {
            translation: Vec3::new(0., 0., 0.2),
            ..default()
        },
    )
}
