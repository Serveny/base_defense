use std::time::Duration;

use bevy::prelude::{Query, With};

use crate::game::enemies::Enemy;
use crate::utils::shots::{DamagePerTimeShot, Shot};
use crate::utils::{pos_to_quat, IngameTime, IngameTimestamp};
use bevy::prelude::*;

type ShotsQuery<'w, 's, 'a> = Query<'w, 's, (Entity, &'a mut Transform, &'a mut Shot), With<Shot>>;
type EnemiesQuery<'w, 's, 'a> = Query<'w, 's, (Entity, &'a mut Enemy), With<Enemy>>;

// type ShotSystemQueries<'w, 's, 'a> = ParamSet<'w, 's, (ShotQuery<'w, 's, 'a>)>;

pub(in crate::game) fn shot_system(
    mut cmds: Commands,
    mut shots: ShotsQuery,
    mut enemies: EnemiesQuery,
    ingame_time: Res<IngameTime>,
    time: Res<Time>,
) {
    let now = ingame_time.now();
    for (shot_entity, mut transform, mut shot) in shots.iter_mut() {
        match shot.as_mut() {
            Shot::Laser(shot) => handle_damage_per_time_shot(
                &mut cmds,
                &mut enemies,
                &mut transform,
                shot,
                shot_entity,
                now,
                time.delta(),
            ),
            Shot::Rocket(shot) => handle_damage_in_radius_enemy_locked_shot(),
        }
    }
}

fn handle_damage_per_time_shot(
    cmds: &mut Commands,
    enemies: &mut EnemiesQuery,
    transform: &mut Transform,
    shot: &mut DamagePerTimeShot,
    shot_entity: Entity,
    now: IngameTimestamp,
    frame_dur: Duration,
) {
    if let Some(die_time) = shot.die_time {
        if now >= die_time {
            cmds.entity(shot_entity).despawn_recursive();
        } else if let Some((enemy_entity, enemy)) = find_enemy_in_range_mut(enemies, shot) {
            if enemy.health <= 0. {
                cmds.entity(enemy_entity).despawn_recursive();
            } else {
                enemy.health -= frame_dur.as_secs_f32() * shot.damage;
                transform.rotation = pos_to_quat(shot.pos_start, enemy.pos);
                transform.scale = Vec3::new(
                    // (time_to_die_percent(now, die_time, shot.lifetime) / 4.) + 0.75,
                    0.4 + (*now % 0.1) * 6.,
                    shot.pos_start.distance(enemy.pos.into()),
                    1.,
                );
            }
        } else {
            cmds.entity(shot_entity).despawn_recursive();
        }
    } else {
        shot.die_time = Some(now + shot.lifetime);
        transform.translation = shot.pos_start.to_scaled_vec3(0.9);
    }
}

fn handle_damage_in_radius_enemy_locked_shot() {}

fn find_enemy_in_range_mut<'a>(
    query: &'a mut EnemiesQuery,
    shot: &DamagePerTimeShot,
) -> Option<(Entity, &'a mut Enemy)> {
    for (entity, enemy) in query.iter_mut() {
        if enemy.id == shot.target_enemy_id && enemy.is_in_range(shot.pos_start, shot.range_radius)
        {
            return Some((entity, enemy.into_inner()));
        }
    }
    None
}
