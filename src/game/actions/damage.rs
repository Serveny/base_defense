use crate::game::enemies::Enemy;
use bevy::prelude::*;

#[derive(Message)]
pub struct DamageMessage {
    entity: Entity,
    damage: f32,
}

impl DamageMessage {
    pub fn new(entity: Entity, damage: f32) -> Self {
        Self { entity, damage }
    }
}

pub fn on_damage(mut events: MessageReader<DamageMessage>, mut enemies: Query<&mut Enemy>) {
    for ev in events.read() {
        if let Ok(mut enemy) = enemies.get_mut(ev.entity) {
            enemy.health -= ev.damage;
        }
    }
}
