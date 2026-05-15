use crate::{
    balance::{ENEMY_DEATH_EXPLOSION_DAMAGE_DIVISOR, ENEMY_DEATH_EXPLOSION_RADIUS_FACTOR},
    game::{enemies::Enemy, GameScreen},
    utils::{
        explosions::{spawn_explosion, Explosion},
        Vec2Board,
    },
};
use bevy::prelude::*;

#[derive(Message)]
pub struct ExplosionMessage {
    pos: Vec2Board,
    radius: f32,
    damage: f32,
}

impl ExplosionMessage {
    pub fn new(pos: Vec2Board, radius: f32, damage: f32) -> Self {
        Self {
            pos,
            radius,
            damage,
        }
    }

    pub fn death(enemy: &Enemy) -> Self {
        Self {
            pos: enemy.pos,
            radius: enemy.size_radius * ENEMY_DEATH_EXPLOSION_RADIUS_FACTOR,
            damage: enemy.health_max / ENEMY_DEATH_EXPLOSION_DAMAGE_DIVISOR,
        }
    }

    pub fn to_explosion(&self) -> Explosion {
        Explosion::new(self.pos, self.radius, self.damage)
    }
}

pub fn on_explosions(mut events: MessageReader<ExplosionMessage>, mut cmds: Commands) {
    for ev in events.read() {
        spawn_explosion::<GameScreen>(&mut cmds, ev.to_explosion());
    }
}
