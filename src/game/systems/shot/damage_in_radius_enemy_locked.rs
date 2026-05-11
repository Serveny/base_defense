use crate::{
    board::BoardCache,
    game::{actions::explosions::ExplosionMessage, enemies::Enemy},
    utils::{pos_to_quat, shots::DamageInRadiusTargetPosShot, IngameTime, Vec2Board},
};
use bevy::prelude::*;

type QueryEnemies<'w, 's, 'a> = Query<'w, 's, (Entity, &'a Enemy)>;

pub fn damage_and_despawn_system(
    mut cmds: Commands,
    mut expl_ev: MessageWriter<ExplosionMessage>,
    q_shots: Query<(Entity, &DamageInRadiusTargetPosShot)>,
) {
    for (entity, shot) in q_shots.iter() {
        if is_explode(shot) {
            expl_ev.write(ExplosionMessage::new(
                shot.pos,
                shot.damage_radius,
                shot.damage,
            ));
            cmds.entity(entity).despawn();
        }
    }
}

fn is_explode(shot: &DamageInRadiusTargetPosShot) -> bool {
    shot.pos.distance(*shot.target_pos) < 0.05
}

pub fn fly_system(
    mut q_shots: Query<&mut DamageInRadiusTargetPosShot>,
    q_enemies: QueryEnemies,
    time: Res<IngameTime>,
    board_cache: Res<BoardCache>,
) {
    let frame_dur = time.delta();
    for mut shot in &mut q_shots {
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
        transform.translation = shot.scaled_pos(transform.translation.z);
        transform.rotation = pos_to_quat(shot.pos, shot.target_pos);
    }
}
