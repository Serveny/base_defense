use bevy::prelude::*;

use crate::{
    board::visualisation::TILE_SIZE, game::actions::resources::ResourceAnimation, utils::IngameTime,
};

pub fn resouce_animation_system(
    mut cmds: Commands,
    mut q_anims: Query<(Entity, &mut Transform, &ResourceAnimation)>,
    ingame_time: Res<IngameTime>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    let now = ingame_time.now();
    for (entity, mut transform, anim) in q_anims.iter_mut() {
        if now >= anim.die_time {
            cmds.entity(entity).despawn_recursive();
        } else {
            transform.translation.y += delta * TILE_SIZE / 2.;
        }
    }
}
