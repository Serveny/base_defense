use crate::{
    game::{actions::damage::DamageEvent, enemies::Enemy},
    utils::{pos_to_quat, shots::DamagePerTimeShot, IngameTime},
};
use bevy::prelude::*;

type EnemiesQuery<'w, 's, 'a> = Query<'w, 's, (Entity, &'a Enemy)>;

pub fn damage_system(
    mut dmg_ev: EventWriter<DamageEvent>,
    q_shots: Query<&DamagePerTimeShot>,
    q_enemies: EnemiesQuery,
    time: Res<Time>,
) {
    let frame_dur = time.delta_seconds();
    for shot in q_shots.iter() {
        if let Some((enemy_entity, _)) = find_enemy_in_range(&q_enemies, shot) {
            dmg_ev.send(DamageEvent::new(enemy_entity, frame_dur * shot.damage));
        }
    }
}

pub fn despawn_system(
    mut cmds: Commands,
    q_shots: Query<(Entity, &DamagePerTimeShot)>,
    q_enemies: EnemiesQuery,
    time: Res<IngameTime>,
) {
    let now = time.now();
    for (entity, shot) in q_shots.iter() {
        if now >= shot.die_time || find_enemy_in_range(&q_enemies, shot).is_none() {
            cmds.entity(entity).despawn_recursive();
        }
    }
}

pub fn visual_system(
    mut q_shots: Query<(&mut Transform, &mut DamagePerTimeShot)>,
    q_enemies: EnemiesQuery,
    time: Res<IngameTime>,
) {
    let now = time.now();
    for (mut transform, shot) in q_shots.iter_mut() {
        if let Ok((_, enemy)) = q_enemies.get(shot.target_enemy_id) {
            *transform = Transform {
                translation: shot.pos_start.to_scaled_vec3(0.9),
                rotation: pos_to_quat(shot.pos_start, enemy.pos),
                scale: Vec3::new(
                    0.4 + (*now % 0.1) * 6.,
                    shot.pos_start.distance(enemy.pos.into()),
                    1.,
                ),
            };
        }
    }
}

fn find_enemy_in_range<'a>(
    q_enemies: &'a EnemiesQuery,
    shot: &DamagePerTimeShot,
) -> Option<(Entity, &'a Enemy)> {
    if let Ok((entity, enemy)) = q_enemies.get(shot.target_enemy_id) {
        if enemy.is_in_range(shot.pos_start, shot.range_radius) {
            return Some((entity, enemy));
        }
    }
    None
}
