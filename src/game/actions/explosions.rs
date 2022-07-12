use bevy::prelude::*;

use crate::utils::IngameTime;

pub struct ExplosionEvent {
    radius: f32,
}

impl ExplosionEvent {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

pub fn on_explosion(
    mut events: EventReader<ExplosionEvent>,
    mut cmds: Commands,
    time: Res<IngameTime>,
) {
    for ev in events.iter() {}
}
