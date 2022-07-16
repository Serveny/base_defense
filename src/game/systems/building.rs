use crate::{
    game::{actions::resources::ResourcesEvent, build_menus::BuildMenuScreen},
    utils::{
        buildings::{factory::Factory, power_plant::PowerPlant},
        IngameTime,
    },
};
use bevy::prelude::*;

pub(super) fn factory_system(
    mut rs_actions: EventWriter<ResourcesEvent>,
    mut q_buildings: Query<&mut Factory, Without<BuildMenuScreen>>,
    time: Res<IngameTime>,
) {
    let now = time.now();
    q_buildings.iter_mut().for_each(|mut factory| {
        if now >= factory.next_drop {
            factory.set_next_drop(now);
            rs_actions.send(ResourcesEvent::Energy(
                factory.energy_consumption,
                factory.pos,
            ));
            rs_actions.send(ResourcesEvent::Materials(
                factory.materials_package_size,
                factory.pos,
            ));
        }
    });
}

pub(super) fn power_plant_system(
    mut rs_actions: EventWriter<ResourcesEvent>,
    mut q_buildings: Query<&mut PowerPlant, Without<BuildMenuScreen>>,
    time: Res<IngameTime>,
) {
    let now = time.now();
    q_buildings.iter_mut().for_each(|mut power_plant| {
        if now >= power_plant.next_drop {
            power_plant.set_next_drop(now);
            rs_actions.send(ResourcesEvent::Energy(
                power_plant.energy_package_size,
                power_plant.pos,
            ));
        }
    });
}
