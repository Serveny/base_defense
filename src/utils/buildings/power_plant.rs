use std::time::Duration;

use super::{building_base_shape, Building, BuildingBase};
use crate::utils::{energy::ENERGY_COLOR, Energy, Vec2Board};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PowerPlant {
    energy_package_size: Energy,
    production_time: Duration,
}

impl Building {
    pub fn power_plant() -> Self {
        Self::PowerPlant(PowerPlant::new())
    }
}

impl PowerPlant {
    pub fn new() -> Self {
        Self {
            energy_package_size: 50.,
            production_time: Duration::from_secs(5),
        }
    }
}

pub fn spawn_power_plant<TScreen: Component + Default>(
    cmds: &mut Commands,
    power_plant: PowerPlant,
    tile_size: f32,
    pos: Vec2Board,
) {
    let color = ENERGY_COLOR;
    cmds.spawn_bundle(building_base_shape(
        pos.to_scaled_vec3(1.),
        tile_size / 1.1,
        color,
    ))
    .with_children(|parent| power_plant_children::<TScreen>(parent, tile_size))
    .insert(BuildingBase)
    .insert(Building::PowerPlant(power_plant))
    .insert(TScreen::default());
}

fn power_plant_children<TScreen: Component + Default>(parent: &mut ChildBuilder, tile_size: f32) {
    parent
        .spawn_bundle(power_plant_building_shape(tile_size, Color::GRAY))
        .insert(TScreen::default());
    parent
        .spawn_bundle(power_plant_chimney_shape(
            tile_size,
            Color::GRAY,
            Vec3::new(tile_size / 4.5, -tile_size / 4., 0.1),
        ))
        .insert(TScreen::default());
    parent
        .spawn_bundle(power_plant_chimney_shape(
            tile_size,
            Color::GRAY,
            Vec3::new(tile_size / 20., -tile_size / 4., 0.1),
        ))
        .insert(TScreen::default());
}

fn power_plant_chimney_shape(tile_size: f32, color: Color, translation: Vec3) -> ShapeBundle {
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

fn power_plant_building_shape(tile_size: f32, color: Color) -> ShapeBundle {
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
