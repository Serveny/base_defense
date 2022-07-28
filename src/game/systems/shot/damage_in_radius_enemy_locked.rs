use crate::{
    board::BoardCache,
    game::{actions::explosions::ExplosionEvent, enemies::Enemy},
    utils::{pos_to_quat, shots::DamageInRadiusTargetPosShot, Vec2Board},
};
use bevy::prelude::*;

type QueryEnemies<'w, 's, 'a> = Query<'w, 's, (Entity, &'a Enemy)>;

pub fn damage_and_despawn_system(
    mut cmds: Commands,
    mut expl_ev: EventWriter<ExplosionEvent>,
    q_shots: Query<(Entity, &DamageInRadiusTargetPosShot)>,
) {
    for (entity, shot) in q_shots.iter() {
        if is_explode(shot) {
            expl_ev.send(ExplosionEvent::new(
                shot.pos,
                shot.damage_radius,
                shot.damage,
            ));
            cmds.entity(entity).despawn_recursive();
        }
    }
}

fn is_explode(shot: &DamageInRadiusTargetPosShot) -> bool {
    shot.pos.distance(*shot.target_pos) < 0.05
}

pub fn fly_system(
    mut q_shots: Query<&mut DamageInRadiusTargetPosShot>,
    q_enemies: QueryEnemies,
    time: Res<Time>,
    board_cache: Res<BoardCache>,
) {
    let frame_dur = time.delta();
    for mut shot in q_shots.iter_mut() {
        if let Some(target_id) = shot.target_id {
            if let Ok((_, enemy)) = q_enemies.get(target_id) {
                shot.fly_to(enemy.pos, frame_dur);
            } else if let Some(enemy) = find_nearest_enemy(&q_enemies, shot.pos) {
                shot.target_id = Some(enemy);
            } else {
                shot.set_target_point_to_likely(&board_cache.road_path);
            }
        } else {
            shot.fly(frame_dur);
        }
    }
}

fn find_nearest_enemy(q_enemies: &QueryEnemies, pos: Vec2Board) -> Option<Entity> {
    q_enemies
        .iter()
        .reduce(|item_1, item_2| {
            match item_1.1.pos.distance(pos.into()) < item_2.1.pos.distance(pos.into()) {
                true => item_1,
                false => item_2,
            }
        })
        .map(|item| item.0)
}

pub fn visual_system(mut q_shot: Query<(&mut Transform, &DamageInRadiusTargetPosShot)>) {
    for (mut transform, shot) in q_shot.iter_mut() {
        transform.translation = shot.pos.to_scaled_vec3(1.);
        transform.rotation = pos_to_quat(shot.pos, shot.target_pos);
    }
}
