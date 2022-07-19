use crate::{
    game::{actions::explosions::ExplosionEvent, enemies::Enemy},
    utils::{pos_to_quat, shots::DamageInRadiusEnemyLockedShot, Vec2Board},
};
use bevy::prelude::*;

type QueryEnemies<'w, 's, 'a> = Query<'w, 's, (Entity, &'a Enemy)>;

pub fn damage_and_despawn_system(
    mut cmds: Commands,
    mut expl_ev: EventWriter<ExplosionEvent>,
    q_enemies: QueryEnemies,
    q_shots: Query<(Entity, &DamageInRadiusEnemyLockedShot)>,
) {
    for (entity, shot) in q_shots.iter() {
        if is_explode(&q_enemies, shot) {
            expl_ev.send(ExplosionEvent::new(
                shot.pos,
                shot.damage_radius,
                shot.damage,
            ));
            cmds.entity(entity).despawn_recursive();
        }
    }
}

fn is_explode(q_enemies: &QueryEnemies, shot: &DamageInRadiusEnemyLockedShot) -> bool {
    match q_enemies.get(shot.target_enemy_id) {
        Ok((_, enemy)) if shot.pos.distance(enemy.pos.into()) >= 0.05 => shot.is_fuel_empty(),
        _ => true,
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
            match item_1.1.pos.distance(pos.into()) < item_2.1.pos.distance(pos.into()) {
                true => item_1,
                false => item_2,
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
