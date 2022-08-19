use crate::{
    game::{actions::collision::EnemyCollisionRemoveEvent, enemies::Enemy},
    utils::collision::{Collision, Collisions},
};
use bevy::prelude::*;

pub(super) fn enemy_collision_remove_system(
    mut rm_ev: EventWriter<EnemyCollisionRemoveEvent>,
    mut collisions: ResMut<Collisions>,
    q_enemies: Query<&Enemy>,
) {
    collisions.iter_mut().for_each(|coll| {
        if let Some((enemy_1, enemy_2)) = active_collision_enemies(coll, &q_enemies) {
            coll.is_critical =
                enemy_1.pos.distance(*enemy_2.pos) < enemy_1.size_radius + enemy_2.size_radius;
        } else {
            rm_ev.send(EnemyCollisionRemoveEvent(
                coll.enemy_before,
                coll.enemy_behind,
            ));
        }
    })
}

fn active_collision_enemies<'a>(
    coll: &Collision,
    q_enemies: &'a Query<&Enemy>,
) -> Option<(&'a Enemy, &'a Enemy)> {
    if let Ok([enemy_1, enemy_2]) = q_enemies.get_many([coll.enemy_before, coll.enemy_behind]) {
        if enemy_1.pos.distance(*enemy_2.pos) < enemy_1.break_radius + enemy_2.break_radius {
            return Some((enemy_1, enemy_2));
        }
    }
    None
}
