use crate::{
    game::enemies::Enemy,
    utils::{explosions::Explosion, IngameTime},
};
use bevy::prelude::*;

pub fn explosion_system(
    mut cmds: Commands,
    mut q_explosions: Query<(Entity, &mut Transform, &mut Explosion)>,
    mut q_enemies: Query<&mut Enemy>,
    time: Res<IngameTime>,
) {
    for (entity, mut transform, mut expl) in q_explosions.iter_mut() {
        if expl.is_end() {
            damage_enemies_in_range(&mut q_enemies, &expl);
            cmds.entity(entity).despawn_recursive();
        } else {
            expl.grow(time.delta());
            let scale_val = expl.current_radius / expl.target_radius;
            transform.scale = Vec3::new(scale_val, scale_val, 1.);
        }
    }
}

fn damage_enemies_in_range<'a>(q_enemies: &'a mut Query<&mut Enemy>, expl: &Explosion) {
    for mut enemy in q_enemies.iter_mut() {
        if enemy.is_in_range(expl.pos, expl.target_radius) {
            enemy.health -= expl.damage;
        }
    }
}
