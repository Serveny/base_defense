use crate::{
    game::{enemies::Enemy, GameScreen},
    utils::{
        explosions::{spawn_explosion, Explosion},
        Vec2Board,
    },
};
use bevy::prelude::*;

#[derive(Event)]
pub struct ExplosionEvent {
    pos: Vec2Board,
    radius: f32,
    damage: f32,
}

impl ExplosionEvent {
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
            radius: enemy.size_radius * 1.2,
            damage: enemy.health_max / 20.,
        }
    }

    pub fn to_explosion(&self) -> Explosion {
        Explosion::new(self.pos, self.radius, self.damage)
    }
}

pub fn on_explosions(mut events: EventReader<ExplosionEvent>, mut cmds: Commands) {
    for ev in events.read() {
        spawn_explosion::<GameScreen>(&mut cmds, ev.to_explosion());
    }
}
