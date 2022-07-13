use crate::{
    game::GameScreen,
    utils::{
        explosions::{spawn_explosion, Explosion},
        Vec2Board,
    },
};
use bevy::prelude::*;

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

    pub fn to_explosion(&self) -> Explosion {
        Explosion::new(self.pos, self.radius, self.damage)
    }
}

pub fn on_explosions(mut events: EventReader<ExplosionEvent>, mut cmds: Commands) {
    for ev in events.iter() {
        spawn_explosion::<GameScreen>(&mut cmds, ev.to_explosion());
    }
}
