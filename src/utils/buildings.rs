use crate::board::visualisation::TILE_SIZE;

use self::{
    factory::{spawn_factory, Factory},
    power_plant::{spawn_power_plant, PowerPlant},
};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use serde::{Deserialize, Serialize};

use super::Vec2Board;

pub mod factory;
pub mod power_plant;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildingValues {}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Building {
    Factory(Factory),
    PowerPlant(PowerPlant),
}

impl Building {
    pub fn draw_preview<TScreen: Component + Default>(&self, cmds: &mut Commands) {
        match self {
            Building::Factory(factory) => {
                spawn_factory::<TScreen>(cmds, factory.clone(), TILE_SIZE, Vec2Board::default())
            }
            Building::PowerPlant(plant) => {
                spawn_power_plant::<TScreen>(cmds, plant.clone(), TILE_SIZE, Vec2Board::default())
            }
        }
    }
}

#[derive(Component)]
pub struct BuildingBase;

pub fn draw_building<TScreen: Component + Default>(
    cmds: &mut Commands,
    pos: Vec2Board,
    building: &Building,
) {
    match building {
        Building::Factory(factory) => {
            spawn_factory::<TScreen>(cmds, factory.clone(), TILE_SIZE, pos)
        }
        Building::PowerPlant(plant) => {
            spawn_power_plant::<TScreen>(cmds, plant.clone(), TILE_SIZE, pos)
        }
    };
}
fn building_base_shape(translation: Vec3, tile_size: f32, color: Color) -> ShapeBundle {
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
            translation,
            ..Default::default()
        },
    )
}
