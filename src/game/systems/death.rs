use crate::game::enemies::Enemy;
use bevy::prelude::*;

pub fn death_system(mut cmds: Commands, q_enemies: Query<(Entity, &Enemy)>) {
    for (entity, enemy) in q_enemies.iter() {
        if enemy.health <= 0. {
            cmds.entity(entity).despawn_recursive();
        }
    }
}
