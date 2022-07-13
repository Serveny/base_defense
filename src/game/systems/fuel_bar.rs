use crate::utils::{fuel_bar::FuelBarPercentage, shots::DamageInRadiusEnemyLockedShot};
use bevy::prelude::*;

type QueryShots<'w, 's, 'a> = Query<'w, 's, (Entity, &'a DamageInRadiusEnemyLockedShot)>;
type QueryFuelBars<'w, 's, 'a> =
    Query<'w, 's, (&'a Parent, &'a mut Transform), With<FuelBarPercentage>>;

pub(in crate::game) fn fuel_bar_system(mut q_fuel_bars: QueryFuelBars, q_shots: QueryShots) {
    for (parent, mut transform) in q_fuel_bars.iter_mut() {
        let (_, shot) = q_shots.get(**parent).unwrap();
        transform.scale = Vec3::new(1., shot.fuel_as_percent(), 1.);
    }
}
