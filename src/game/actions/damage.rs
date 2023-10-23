use crate::game::enemies::Enemy;
use bevy::prelude::*;

#[derive(Event)]
pub struct DamageEvent {
    entity: Entity,
    damage: f32,
}

impl DamageEvent {
    pub fn new(entity: Entity, damage: f32) -> Self {
        Self { entity, damage }
    }
}

pub fn on_damage(mut events: EventReader<DamageEvent>, mut enemies: Query<&mut Enemy>) {
    for ev in events.iter() {
        if let Ok(mut enemy) = enemies.get_mut(ev.entity) {
            enemy.health -= ev.damage;
        }
    }
}
