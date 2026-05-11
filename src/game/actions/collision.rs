use crate::utils::{
    collision::{Collision, Collisions},
    speed::Speed,
};
use bevy::prelude::*;

#[derive(Message)]
pub struct EnemyCollisionAddMessage(pub Entity, pub Entity);

#[derive(Message)]
pub struct EnemyCollisionRemoveMessage(pub Entity, pub Entity);

impl From<Collision> for EnemyCollisionRemoveMessage {
    fn from(coll: Collision) -> Self {
        EnemyCollisionRemoveMessage(coll.enemy_before, coll.enemy_behind)
    }
}

impl PartialEq<Collision> for EnemyCollisionRemoveMessage {
    fn eq(&self, other: &Collision) -> bool {
        self.0 == other.enemy_before && self.1 == other.enemy_behind
            || self.0 == other.enemy_behind && self.1 == other.enemy_before
    }
}

pub(super) fn on_enemy_collision_add(
    mut events: MessageReader<EnemyCollisionAddMessage>,
    mut q_speeds: Query<&mut Speed>,
) {
    for ev in events.read() {
        if let Ok([mut speed_before, mut speed_behind]) = q_speeds.get_many_mut([ev.0, ev.1]) {
            if speed_before.current < speed_behind.current {
                std::mem::swap(&mut speed_before.current, &mut speed_behind.current);
            }
        }
    }
}

pub(super) fn on_enemy_collision_remove(
    mut events: MessageReader<EnemyCollisionRemoveMessage>,
    mut collisions: ResMut<Collisions>,
    mut q_speeds: Query<&mut Speed>,
) {
    for ev in events.read() {
        set_speed_to_normal(&mut q_speeds, ev.0);
        set_speed_to_normal(&mut q_speeds, ev.1);
        remove_collision(&mut collisions, ev);
    }
}

fn remove_collision(collisions: &mut Collisions, ev: &EnemyCollisionRemoveMessage) {
    if let Some(index) = collisions.iter().position(|coll| *ev == *coll) {
        collisions.remove(index);
    }
}

fn set_speed_to_normal(q_speeds: &mut Query<&mut Speed>, entity: Entity) {
    if let Ok(mut speed) = q_speeds.get_mut(entity) {
        speed.target = speed.normal;
    }
}
