use super::{actions::GameActionEvent, enemies::Enemy};
use crate::utils::{
    health_bar::HealthBarPercentage,
    pos_to_quat,
    shots::{Shot, TowerStatus},
    towers::{Tower, TowerCannon, TowerValues},
    IngameTime, IngameTimestamp, Vec2Board,
};
use bevy::prelude::*;

type EnemiesQuery<'w, 's, 'a> = Query<'w, 's, (Entity, &'a Enemy, &'a Children), With<Enemy>>;
type HeathBarsQuery<'w, 's, 'a> =
    Query<'w, 's, (&'a mut Transform, &'a mut HealthBarPercentage), With<HealthBarPercentage>>;
type TowerShotQuery<'w, 's, 'a> = Query<'w, 's, &'a mut Visibility, With<Shot>>;

pub(super) fn tower_system(
    mut cannon_transforms: Query<&mut Transform, With<TowerCannon>>,
    mut towers: Query<(&mut Tower, &Children), With<Tower>>,
    mut actions: EventWriter<GameActionEvent>,
    enemies: EnemiesQuery,
    time: Res<IngameTime>,
) {
    let now = IngameTimestamp::new(time.elapsed_secs());
    for (mut tower, children) in towers.iter_mut() {
        let mut tower_vals = tower.values_mut();
        let locked_enemy = lock_tower_to_enemy(&mut tower_vals, &enemies);
        rotate_cannon_to_enemy(
            &mut cannon_transforms,
            children,
            tower_vals.pos,
            locked_enemy,
        );
        shoot_or_reload(&mut actions, &mut tower_vals, locked_enemy, now);
    }
}

fn lock_tower_to_enemy<'a>(
    tower_vals: &mut TowerValues,
    enemies: &'a EnemiesQuery,
) -> Option<&'a Enemy> {
    if let Some(locked_entity) = tower_vals.target_lock {
        if let Some(locked_enemy) =
            find_locked_enemy_in_tower_range(locked_entity, &enemies, &tower_vals)
        {
            return Some(locked_enemy);
        } else {
            tower_vals.target_lock = None;
        }
    } else {
        tower_vals.target_lock = find_first_enemy_entity_in_range(&tower_vals, enemies);
    }
    None
}

fn rotate_cannon_to_enemy(
    cannon_transforms: &mut Query<&mut Transform, With<TowerCannon>>,
    tower_children: &Children,
    tower_pos: Vec2Board,
    locked_enemy: Option<&Enemy>,
) {
    if let Some(locked_enemy) = locked_enemy {
        rotate_tower_cannon_to_pos(
            cannon_transforms,
            tower_pos,
            locked_enemy.pos,
            tower_children,
        );
    }
}

fn find_first_enemy_entity_in_range<'a>(
    tower_vals: &TowerValues,
    enemies: &'a EnemiesQuery,
) -> Option<Entity> {
    for (entity, enemy, _) in enemies.iter() {
        if is_enemy_in_tower_range(enemy.pos, tower_vals) {
            return Some(entity);
        }
    }
    None
}

fn find_locked_enemy_in_tower_range<'a>(
    locked_entity: Entity,
    enemies: &'a EnemiesQuery,
    tower_vals: &TowerValues,
) -> Option<&'a Enemy> {
    for (entity, enemy, _) in enemies.iter() {
        if entity == locked_entity {
            return if is_enemy_in_tower_range(enemy.pos, tower_vals) {
                Some(enemy)
            } else {
                None
            };
        }
    }
    None
}

fn is_enemy_in_tower_range(enemy_pos: Vec2Board, tower_vals: &TowerValues) -> bool {
    enemy_pos.distance(tower_vals.pos.into()) <= tower_vals.range_radius
}

fn rotate_tower_cannon_to_pos(
    cannon_transforms: &mut Query<&mut Transform, With<TowerCannon>>,
    tower_pos: Vec2Board,
    enemy_pos: Vec2Board,
    tower_children: &Children,
) {
    for child in tower_children.iter() {
        if let Ok(mut transform) = cannon_transforms.get_mut(*child) {
            transform.rotation = pos_to_quat(tower_pos, enemy_pos);
        }
    }
}

fn shoot_or_reload(
    actions: &mut EventWriter<GameActionEvent>,
    tower_vals: &mut TowerValues,
    enemy: Option<&Enemy>,
    now: IngameTimestamp,
) {
    match tower_vals.tower_status {
        TowerStatus::Reloading(time_finish) => {
            if now >= time_finish {
                tower_vals.tower_status = TowerStatus::Waiting;
            }
        }
        TowerStatus::Waiting => {
            if let Some(enemy) = enemy {
                actions.send(GameActionEvent::Shoot(tower_vals.shoot(enemy.pos)));
                tower_vals.tower_status = TowerStatus::Reloading(now + tower_vals.reload_duration);
            }
        }
    };
}
