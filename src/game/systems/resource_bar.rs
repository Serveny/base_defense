use crate::utils::{
    buildings::{factory::Factory, power_plant::PowerPlant},
    resource_bar::ResourceBarPercentage,
    shots::DamageInRadiusEnemyLockedShot,
};
use bevy::prelude::*;

type QueryShots<'w, 's, 'a> = Query<'w, 's, (Entity, &'a DamageInRadiusEnemyLockedShot)>;
type QueryResourceBars<'w, 's, 'a> =
    Query<'w, 's, (&'a Parent, &'a mut Transform), With<ResourceBarPercentage>>;

pub(in crate::game) fn resource_bar_system(
    mut q_resource_bars: QueryResourceBars,
    q_shots: QueryShots,
    q_power_plants: Query<(Entity, &PowerPlant)>,
    q_factories: Query<(Entity, &Factory)>,
) {
    for (parent, mut transform) in q_resource_bars.iter_mut() {
        if let Ok((_, shot)) = q_shots.get(**parent) {
            transform.scale = Vec3::new(1., shot.fuel.percent(), 1.);
        }
        if let Ok((_, pp)) = q_power_plants.get(**parent) {
            transform.scale = Vec3::new(1., pp.energy.percent(), 1.);
        }
        if let Ok((_, factory)) = q_factories.get(**parent) {
            transform.scale = Vec3::new(1., factory.materials.percent(), 1.);
        }
    }
}
