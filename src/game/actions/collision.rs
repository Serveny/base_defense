use crate::utils::{
    collision::{Collision, Collisions},
    speed::Speed,
};
use bevy::prelude::*;

pub struct EnemyCollisionAddEvent(pub Entity, pub Entity);
pub struct EnemyCollisionRemoveEvent(pub Entity, pub Entity);

impl From<Collision> for EnemyCollisionRemoveEvent {
    fn from(coll: Collision) -> Self {
        EnemyCollisionRemoveEvent(coll.enemy_before, coll.enemy_behind)
    }
}

impl PartialEq<Collision> for EnemyCollisionRemoveEvent {
    fn eq(&self, other: &Collision) -> bool {
        self.0 == other.enemy_before && self.1 == other.enemy_behind
            || self.0 == other.enemy_behind && self.1 == other.enemy_before
    }
}

pub(super) fn on_enemy_collision_add(
    mut events: EventReader<EnemyCollisionAddEvent>,
    mut q_speeds: Query<&mut Speed>,
) {
    for ev in events.iter() {
        if let Ok([mut speed_1, mut speed_2]) = q_speeds.get_many_mut([ev.0, ev.1]) {
            std::mem::swap(&mut speed_1.target, &mut speed_2.target);
        }
    }
}

pub(super) fn on_enemy_collision_remove(
    mut events: EventReader<EnemyCollisionRemoveEvent>,
    mut collisions: ResMut<Collisions>,
    mut q_speeds: Query<&mut Speed>,
) {
    for ev in events.iter() {
        set_speed_to_normal(&mut q_speeds, ev.0);
        set_speed_to_normal(&mut q_speeds, ev.1);
        remove_collision(&mut collisions, ev);
    }
}

fn remove_collision(collisions: &mut Collisions, ev: &EnemyCollisionRemoveEvent) {
    let index = collisions.iter().position(|coll| *ev == *coll).unwrap();
    collisions.remove(index);
}

fn set_speed_to_normal(q_speeds: &mut Query<&mut Speed>, entity: Entity) {
    if let Ok(mut speed) = q_speeds.get_mut(entity) {
        speed.target = speed.normal;
    }
}
