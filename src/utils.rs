#![allow(unused)]
use bevy::prelude::*;

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Menu,
    Game,
    MapEditor,
}

#[derive(Debug, Clone)]
pub struct Tower {
    tower_type: TowerType,
}

impl Tower {
    fn new(tower_type: TowerType) -> Self {
        Self { tower_type }
    }
}

#[derive(Debug, Clone)]
pub enum TowerType {
    // Damages enemies, needs energy
    LaserShot,

    // Slows enemies down, needs energy
    Microwave,

    // Damages enemies, needs energy and material
    Rocket,

    // Damages enemies, needs energy and material
    Grenade,
}

#[derive(Debug, Clone)]
pub struct Building {
    building_type: BuildingType,
}

impl Building {
    fn new(building_type: BuildingType) -> Self {
        Self { building_type }
    }
}

#[derive(Debug, Clone)]
enum BuildingType {
    Factory,
    PowerPlant,
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
