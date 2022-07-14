use self::power_plant::PowerPlant;
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use serde::{Deserialize, Serialize};

pub mod power_plant;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildingValues {}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Building {
    Factory(BuildingValues),
    PowerPlant(PowerPlant),
}

#[derive(Component)]
pub struct BuildingBase;

fn building_base_shape(tile_size: f32, color: Color) -> ShapeBundle {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(tile_size / 2.),
        ..shapes::RegularPolygon::default()
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(color),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, tile_size / 20.),
        },
        Transform {
            scale: Vec3::new(0., 0., 0.),
            ..Default::default()
        },
    )
}
