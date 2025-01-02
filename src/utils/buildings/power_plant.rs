use super::{building_base_shape, Building, BuildingBase};
use crate::utils::{
    buffer::Buffer, energy::ENERGY_COLOR, resource_bar::spawn_resource_bar, Amount, BoardPos,
    Energy, Vec2Board,
};
use bevy::color::palettes::css::{DARK_GRAY, GRAY};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PowerPlant {
    pub pos: Vec2Board,
    pub energy: Buffer<Energy>,
}

impl PowerPlant {
    pub fn new(pos: Vec2Board) -> Self {
        Self {
            pos,
            energy: Buffer::new(50., Amount::PerSecond(10.)),
        }
    }

    pub fn produce(&mut self, frame_dur: Duration) -> Option<Energy> {
        self.energy.produce_during(frame_dur)
    }
}

pub fn spawn_power_plant<TScreen: Component + Default>(
    cmds: &mut Commands,
    power_plant: PowerPlant,
    tile_size: f32,
) {
    cmds.spawn((
        Transform::from_translation(power_plant.pos.to_scaled_vec3(1.)),
        Visibility::Visible,
    ))
    .with_children(|parent| power_plant_children(parent, tile_size))
    .insert(BuildingBase)
    .insert(Building::PowerPlant)
    .insert(BoardPos(power_plant.pos.as_uvec2()))
    .insert(power_plant)
    .insert(TScreen::default());
}

fn power_plant_children(parent: &mut ChildBuilder, tile_size: f32) {
    let color = ENERGY_COLOR.into();
    parent.spawn(building_base_shape(tile_size / 1.1, color));
    parent.spawn(power_plant_building_shape(tile_size, GRAY.into()));
    parent.spawn(power_plant_chimney_shape(
        tile_size,
        GRAY.into(),
        Vec3::new(tile_size / 4.5, -tile_size / 4., 0.01),
    ));
    parent.spawn(power_plant_chimney_shape(
        tile_size,
        GRAY.into(),
        Vec3::new(tile_size / 20., -tile_size / 4., 0.01),
    ));
    spawn_resource_bar(
        parent,
        tile_size / 4.,
        Vec2Board::new(0.2, 0.),
        ENERGY_COLOR.into(),
    );
}

fn power_plant_chimney_shape(tile_size: f32, color: Color, translation: Vec3) -> impl Bundle {
    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                origin: RectangleOrigin::CustomCenter(Vec2::new(0., tile_size / 2.)),
                extents: Vec2::new(tile_size / 10., tile_size / 2.),
                radii: None,
            }),
            transform: Transform::from_translation(translation),
            ..default()
        },
        Fill::color(color),
        Stroke::new(DARK_GRAY, tile_size / 20.),
    )
}

fn power_plant_building_shape(tile_size: f32, color: Color) -> impl Bundle {
    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                origin: RectangleOrigin::Center,
                extents: Vec2::new(tile_size / 1.75, tile_size / 3.),
                radii: None,
            }),
            transform: Transform::from_xyz(0., 0., 0.02),
            ..default()
        },
        Fill::color(color),
        Stroke::new(DARK_GRAY, tile_size / 20.),
    )
}
