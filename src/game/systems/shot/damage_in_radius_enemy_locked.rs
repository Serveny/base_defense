use crate::{
    game::enemies::Enemy,
    utils::{pos_to_quat, shots::DamageInRadiusEnemyLockedShot, Vec2Board},
};
use bevy::prelude::*;

type QueryEnemies<'w, 's, 'a> = Query<'w, 's, (Entity, &'a Enemy)>;
type QueryEnemiesMut<'w, 's, 'a> = Query<'w, 's, &'a mut Enemy>;

pub fn damage_and_despawn_system(
    mut cmds: Commands,
    mut q_enemies: QueryEnemiesMut,
    q_shots: Query<(Entity, &DamageInRadiusEnemyLockedShot)>,
) {
    for (entity, shot) in q_shots.iter() {
        if is_explode(&q_enemies, shot) {
            damage_enemies_in_range(&mut q_enemies, shot);
            cmds.entity(entity).despawn_recursive();
        }
    }
}

fn is_explode(q_enemies: &QueryEnemiesMut, shot: &DamageInRadiusEnemyLockedShot) -> bool {
    match q_enemies.get(shot.target_enemy_id) {
        Ok(enemy) if shot.pos.distance(enemy.pos.into()) >= shot.damage_radius => shot.fuel <= 0.,
        _ => true,
    }
}

fn damage_enemies_in_range<'a>(
    q_enemies: &'a mut QueryEnemiesMut,
    shot: &DamageInRadiusEnemyLockedShot,
) {
    for mut enemy in q_enemies.iter_mut() {
        if enemy.is_in_range(shot.pos, shot.damage_radius) {
            enemy.health -= shot.damage;
        }
    }
}

pub fn fly_system(
    mut q_shots: Query<&mut DamageInRadiusEnemyLockedShot>,
    q_enemies: QueryEnemies,
    time: Res<Time>,
) {
    let frame_dur = time.delta();
    for mut shot in q_shots.iter_mut() {
        if let Ok((_, enemy)) = q_enemies.get(shot.target_enemy_id) {
            shot.fly(enemy.pos, frame_dur);
        } else if let Some(enemy) = find_nearest_enemy(&q_enemies, shot.pos) {
            shot.target_enemy_id = enemy;
        }
    }
}

fn find_nearest_enemy(q_enemies: &QueryEnemies, pos: Vec2Board) -> Option<Entity> {
    q_enemies
        .iter()
        .reduce(|item_1, item_2| {
            if item_1.1.pos.distance(pos.into()) < item_2.1.pos.distance(pos.into()) {
                item_1
            } else {
                item_2
            }
        })
        .map(|item| item.0)
}

pub fn visual_system(
    mut q_shot: Query<(&mut Transform, &DamageInRadiusEnemyLockedShot)>,
    q_enemy: Query<&Enemy>,
) {
    for (mut transform, shot) in q_shot.iter_mut() {
        transform.translation = shot.pos.to_scaled_vec3(1.);

        if let Ok(enemy) = q_enemy.get(shot.target_enemy_id) {
            transform.rotation = pos_to_quat(shot.pos, enemy.pos);
        }
    }
}
