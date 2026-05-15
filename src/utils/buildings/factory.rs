use std::time::Duration;

use super::{building_base_shape, Building, BuildingBase};
use crate::{
    balance::{
        FACTORY_ENERGY_BUFFER_SIZE, FACTORY_ENERGY_PER_SECOND, FACTORY_MATERIALS_BUFFER_SIZE,
        FACTORY_MATERIALS_PER_SECOND,
    },
    utils::{
        buffer::Buffer, materials::MATERIALS_COLOR, resource_bar::spawn_resource_bar, Amount,
        BoardPos, Energy, Materials, Vec2Board,
    },
};
use bevy::color::palettes::css::{DIM_GRAY, GRAY};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use euclid::Angle;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Factory {
    pub pos: Vec2Board,
    pub enery: Buffer<Energy>,
    pub materials: Buffer<Materials>,
}

impl Factory {
    pub fn new(pos: Vec2Board) -> Self {
        Self {
            pos,
            enery: Buffer::new(
                FACTORY_ENERGY_BUFFER_SIZE,
                Amount::PerSecond(FACTORY_ENERGY_PER_SECOND),
            ),
            materials: Buffer::new(
                FACTORY_MATERIALS_BUFFER_SIZE,
                Amount::PerSecond(FACTORY_MATERIALS_PER_SECOND),
            ),
        }
    }

    pub fn produce(&mut self, frame_dur: Duration) -> (Option<Energy>, Option<Materials>) {
        (
            self.enery.consume_during(frame_dur),
            self.materials.produce_during(frame_dur),
        )
    }
}

pub fn spawn_factory<TScreen: Component + Default>(
    cmds: &mut Commands,
    factory: Factory,
    tile_size: f32,
) {
    cmds.spawn((
        Transform::from_translation(factory.pos.to_scaled_vec3(1.)),
        Visibility::Visible,
    ))
    .with_children(|parent| factory_children(parent, tile_size))
    .insert(BuildingBase)
    .insert(Building::Factory)
    .insert(BoardPos(factory.pos.as_uvec2()))
    .insert(factory)
    .insert(TScreen::default());
}

fn factory_children(parent: &mut ChildSpawnerCommands, tile_size: f32) {
    let color = MATERIALS_COLOR.into();
    parent.spawn(building_base_shape(tile_size / 1.1, color));
    parent.spawn(factory_building_shape(tile_size, GRAY.into()));
    parent.spawn(factory_roof_shape(
        tile_size,
        GRAY.into(),
        Vec3::new(-tile_size / 6., tile_size / 5., 0.01),
    ));
    parent.spawn(factory_roof_shape(
        tile_size,
        GRAY.into(),
        Vec3::new(0., tile_size / 5., 0.01),
    ));
    parent.spawn(factory_roof_shape(
        tile_size,
        GRAY.into(),
        Vec3::new(tile_size / 6., tile_size / 5., 0.01),
    ));
    parent.spawn(factory_chimney_shape(
        tile_size,
        GRAY.into(),
        Vec3::new(tile_size / 6., -tile_size / 4., 0.011),
    ));
    spawn_resource_bar(
        parent,
        tile_size / 4.,
        Vec2Board::new(0.2, 0.),
        MATERIALS_COLOR.into(),
    );
}

fn factory_roof_shape(tile_size: f32, color: Color, translation: Vec3) -> impl Bundle {
    (
        ShapeBuilder::with(&shapes::RegularPolygon {
            sides: 3,
            feature: RegularPolygonFeature::Radius(tile_size / 10.),
            ..default()
        })
        .fill(color)
        .stroke(Stroke::new(DIM_GRAY, tile_size / 40.))
        .build(),
        Transform {
            translation,
            rotation: Quat::from_rotation_z(Angle::degrees(0.).radians),
            ..default()
        },
    )
}

fn factory_chimney_shape(tile_size: f32, color: Color, translation: Vec3) -> impl Bundle {
    (
        ShapeBuilder::with(&shapes::Rectangle {
            origin: RectangleOrigin::CustomCenter(Vec2::new(0., tile_size / 2.)),
            extents: Vec2::new(tile_size / 10., tile_size / 2.),
            radii: None,
        })
        .fill(color)
        .stroke(Stroke::new(DIM_GRAY, tile_size / 20.))
        .build(),
        Transform::from_translation(translation),
    )
}

fn factory_building_shape(tile_size: f32, color: Color) -> impl Bundle {
    (
        ShapeBuilder::with(&shapes::Rectangle {
            origin: RectangleOrigin::Center,
            extents: Vec2::new(tile_size / 1.75, tile_size / 3.),
            radii: None,
        })
        .fill(color)
        .stroke(Stroke::new(DIM_GRAY, tile_size / 20.))
        .build(),
        Transform::from_xyz(0., 0., 0.02),
    )
}
