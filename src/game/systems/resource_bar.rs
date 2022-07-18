use crate::utils::{resource_bar::ResourceBarPercentage, shots::DamageInRadiusEnemyLockedShot};
use bevy::prelude::*;

type QueryShots<'w, 's, 'a> = Query<'w, 's, (Entity, &'a DamageInRadiusEnemyLockedShot)>;
type QueryResourceBars<'w, 's, 'a> =
    Query<'w, 's, (&'a Parent, &'a mut Transform), With<ResourceBarPercentage>>;

pub(in crate::game) fn resource_bar_system(
    mut q_resource_bars: QueryResourceBars,
    q_shots: QueryShots,
) {
    for (parent, mut transform) in q_resource_bars.iter_mut() {
        let (_, shot) = q_shots.get(**parent).unwrap();
        transform.scale = Vec3::new(1., shot.fuel_as_percent(), 1.);
    }
}
