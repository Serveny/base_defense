use super::enemies::Enemy;
use crate::utils::{
    pos_to_quat,
    towers::{Tower, TowerCannon, TowerValues},
    Vec2Board,
};
use bevy::prelude::*;

type EnemiesQuery<'w, 's, 'a> = Query<'w, 's, (Entity, &'a Enemy), With<Enemy>>;

pub(super) fn tower_system(
    mut cannon_transforms: Query<&mut Transform, With<TowerCannon>>,
    mut towers: Query<(&mut Tower, &Children), With<Tower>>,
    enemies: EnemiesQuery,
) {
    for (mut tower, children) in towers.iter_mut() {
        rotate_tower_cannon_to_enemies(&mut cannon_transforms, children, &mut tower, &enemies);
    }
}

fn rotate_tower_cannon_to_enemies(
    cannon_transforms: &mut Query<&mut Transform, With<TowerCannon>>,
    tower_children: &Children,
    tower: &mut Tower,
    enemies: &EnemiesQuery,
) {
    let tower_vals = tower.values_mut();
    if let Some(locked_entity) = tower_vals.target_lock {
        if let Some(locked_enemy) =
            find_locked_enemy_in_tower_range(locked_entity, &enemies, &tower_vals)
        {
            rotate_tower_cannon_to_pos(
                cannon_transforms,
                tower_vals.pos,
                locked_enemy.pos,
                tower_children,
            );
        } else {
            tower_vals.target_lock = None;
        }
    } else {
        tower_vals.target_lock = find_first_enemy_entity_in_range(&tower_vals, enemies);
    }
}

fn find_first_enemy_entity_in_range<'a>(
    tower_vals: &TowerValues,
    enemies: &'a EnemiesQuery,
) -> Option<Entity> {
    for (entity, enemy) in enemies.iter() {
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
    for (entity, enemy) in enemies.iter() {
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
