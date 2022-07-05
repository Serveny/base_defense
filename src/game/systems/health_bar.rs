use bevy::prelude::*;

use crate::{game::enemies::Enemy, utils::health_bar::HealthBarPercentage};

type EnemiesQuery<'w, 's, 'a> = Query<'w, 's, (Entity, &'a Enemy), With<Enemy>>;
type HealthBarsQuery<'w, 's, 'a> =
    Query<'w, 's, (&'a Parent, &'a mut Transform), With<HealthBarPercentage>>;

pub(in crate::game) fn health_bar_system(
    mut health_bar_query: HealthBarsQuery,
    enemy_query: EnemiesQuery,
) {
    for (parent, mut transform) in health_bar_query.iter_mut() {
        let enemy = find_enemy(&enemy_query, **parent).unwrap();
        transform.scale = Vec3::new(enemy.health_as_percent(), 1., 1.);
    }
}

fn find_enemy<'a>(query: &'a EnemiesQuery, enemy_entity: Entity) -> Option<&'a Enemy> {
    for (entity, enemy) in query.iter() {
        if entity == enemy_entity {
            return Some(enemy);
        }
    }
    None
}
