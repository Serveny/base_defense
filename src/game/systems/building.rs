use crate::{
    game::{actions::resources::ResourcesEvent, build_menus::BuildMenuScreen},
    utils::{
        buildings::{factory::Factory, power_plant::PowerPlant},
        IngameTime,
    },
};

use bevy::prelude::*;
pub(super) fn power_plant_system(
    mut rs_actions: EventWriter<ResourcesEvent>,
    mut q_buildings: Query<&mut PowerPlant, Without<BuildMenuScreen>>,
    time: Res<IngameTime>,
) {
    q_buildings.iter_mut().for_each(|mut power_plant| {
        if let Some(energy) = power_plant.produce(time.delta()) {
            rs_actions.send(ResourcesEvent::Energy(energy, power_plant.pos));
        }
    });
}

pub(super) fn factory_system(
    mut rs_actions: EventWriter<ResourcesEvent>,
    mut q_buildings: Query<&mut Factory, Without<BuildMenuScreen>>,
    time: Res<IngameTime>,
) {
    q_buildings.iter_mut().for_each(|mut factory| {
        let (enery, materials) = factory.produce(time.delta());
        if let Some(energy) = enery {
            rs_actions.send(ResourcesEvent::Energy(energy, factory.pos));
        }
        if let Some(materials) = materials {
            rs_actions.send(ResourcesEvent::Materials(materials, factory.pos));
        }
    });
}
