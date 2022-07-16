use crate::game::{actions::explosions::ExplosionEvent, enemies::Enemy};
use bevy::prelude::*;

pub fn death_system(
    mut cmds: Commands,
    mut expl_actions: EventWriter<ExplosionEvent>,
    q_enemies: Query<(Entity, &Enemy)>,
) {
    for (entity, enemy) in q_enemies.iter() {
        if enemy.health <= 0. {
            expl_actions.send(ExplosionEvent::death(enemy));
            cmds.entity(entity).despawn_recursive();
        }
    }
}
