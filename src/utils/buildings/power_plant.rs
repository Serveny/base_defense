use super::{building_base_shape, Building, BuildingBase};
use crate::utils::{energy::ENERGY_COLOR, BoardPos, Energy, IngameTimestamp, Vec2Board};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PowerPlant {
    pub energy_package_size: Energy,
    pub production_time: Duration,
    pub next_drop: IngameTimestamp,
    pub pos: Vec2Board,
}

impl PowerPlant {
    pub fn new(now: IngameTimestamp, pos: Vec2Board) -> Self {
        let production_time = Duration::from_secs(5);
        Self {
            energy_package_size: 50.,
            production_time,
            next_drop: now + production_time,
            pos,
        }
    }

    pub fn set_next_drop(&mut self, now: IngameTimestamp) {
        self.next_drop = now + self.production_time;
    }
}

pub fn spawn_power_plant<TScreen: Component + Default>(
    cmds: &mut Commands,
    power_plant: PowerPlant,
    tile_size: f32,
) {
    let color = ENERGY_COLOR;
    cmds.spawn_bundle(building_base_shape(
        power_plant.pos.to_scaled_vec3(1.),
        tile_size / 1.1,
        color,
    ))
    .with_children(|parent| power_plant_children::<TScreen>(parent, tile_size))
    .insert(BuildingBase)
    .insert(Building::PowerPlant)
    .insert(BoardPos(power_plant.pos.as_uvec2()))
    .insert(power_plant)
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
