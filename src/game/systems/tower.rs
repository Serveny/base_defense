use crate::{
    game::{actions::tower::TowerActionsEvent, build_menus::BuildMenuScreen, enemies::Enemy},
    utils::{
        pos_to_quat,
        shots::{Shot, TowerStatus},
        towers::{Tower, TowerCannon, TowerValues},
        IngameTime, IngameTimestamp, Vec2Board,
    },
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::time::Duration;

type QueryTowersAndChildren<'w, 's, 'a> =
    Query<'w, 's, (&'a Tower, &'a Children), Without<BuildMenuScreen>>;
type QueryCannonTransMut<'w, 's, 'a> =
    Query<'w, 's, &'a mut Transform, (With<TowerCannon>, Without<BuildMenuScreen>)>;
type QueryCannonDrawMut<'w, 's, 'a> =
    Query<'w, 's, &'a mut Fill, (With<TowerCannon>, Without<BuildMenuScreen>)>;
type EnemiesQuery<'w, 's, 'a> = Query<'w, 's, (Entity, &'a Enemy, &'a Children)>;
type EntityEnemy<'a> = (Entity, &'a Enemy);

pub(super) fn tower_target_system(
    mut tower_acts: EventWriter<TowerActionsEvent>,
    mut q_towers: Query<&mut Tower, Without<BuildMenuScreen>>,
    q_enemies: EnemiesQuery,
    time: Res<IngameTime>,
) {
    for mut tower in q_towers.iter_mut() {
        let vals = tower.values_mut();
        let enemy = lock_tower_to_enemy(vals, &q_enemies);
        shoot_or_reload(&mut tower_acts, vals, enemy, time.now());
    }
}

pub(super) fn tower_rotation_system(
    mut q_cannons: QueryCannonTransMut,
    q_towers: QueryTowersAndChildren,
    q_enemies: EnemiesQuery,
) {
    for (tower, children) in q_towers.iter() {
        let vals = tower.values();
        if let Some(enemy) = target_enemy(&q_enemies, vals.target_lock) {
            if let Some(mut cannon_trans) = cannon_trans_mut(&mut q_cannons, children) {
                rotate_tower_cannon_to_pos(&mut cannon_trans, enemy.pos, vals.pos);
            }
        }
    }
}

pub(super) fn tower_overheat_system(
    mut q_cannons: QueryCannonDrawMut,
    q_towers: QueryTowersAndChildren,
    time: Res<IngameTime>,
) {
    for (tower, children) in q_towers.iter() {
        if let Some(draw_mode) = cannon_draw_mut(&mut q_cannons, children) {
            overheat_cannon(draw_mode, tower.values(), time.now());
        }
    }
}

fn target_enemy<'a>(
    q_enemies: &'a EnemiesQuery,
    locked_enemy: Option<Entity>,
) -> Option<&'a Enemy> {
    if let Some(locked_enemy) = locked_enemy {
        if let Ok((_, enemy, _)) = q_enemies.get(locked_enemy) {
            return Some(enemy);
        }
    }
    None
}

fn lock_tower_to_enemy<'a>(
    tower_vals: &mut TowerValues,
    enemies: &'a EnemiesQuery,
) -> Option<EntityEnemy<'a>> {
    match tower_vals.target_lock {
        Some(locked_entity) => {
            match find_locked_enemy_in_tower_range(locked_entity, enemies, tower_vals) {
                Some(locked_enemy) => return Some((locked_entity, locked_enemy)),
                None => tower_vals.target_lock = None,
            }
        }
        None => tower_vals.target_lock = find_first_enemy_entity_in_range(tower_vals, enemies),
    }
    None
}

fn find_first_enemy_entity_in_range(
    tower_vals: &TowerValues,
    enemies: &EnemiesQuery,
) -> Option<Entity> {
    for (entity, enemy, _) in enemies.iter() {
        if enemy.is_in_range(tower_vals.pos, tower_vals.range_radius) {
            return Some(entity);
        }
    }
    None
}

fn find_locked_enemy_in_tower_range<'a>(
    locked_enemy_entity: Entity,
    q_enemies: &'a EnemiesQuery,
    tower_vals: &TowerValues,
) -> Option<&'a Enemy> {
    if let Ok((_, enemy, _)) = q_enemies.get(locked_enemy_entity) {
        if enemy.is_in_range(tower_vals.pos, tower_vals.range_radius) {
            return Some(enemy);
        }
    }
    None
}

fn cannon_trans_mut<'a>(
    cannons: &'a mut QueryCannonTransMut,
    tower_children: &Children,
) -> Option<Mut<'a, Transform>> {
    for child in tower_children.iter() {
        if cannons.get(*child).is_ok() {
            return cannons.get_mut(*child).ok();
        }
    }
    None
}

fn cannon_draw_mut<'a>(
    cannons: &'a mut QueryCannonDrawMut,
    tower_children: &Children,
) -> Option<Mut<'a, Fill>> {
    for child in tower_children.iter() {
        if cannons.get(*child).is_ok() {
            return cannons.get_mut(*child).ok();
        }
    }
    None
}

fn rotate_tower_cannon_to_pos(
    transform: &mut Transform,
    enemy_pos: Vec2Board,
    tower_pos: Vec2Board,
) {
    transform.rotation = pos_to_quat(tower_pos, enemy_pos);
}

fn overheat_cannon(mut fill: Mut<Fill>, tower_vals: &TowerValues, now: IngameTimestamp) {
    *fill = Fill::color(overheat_color(tower_vals, now));
}

fn overheat_color(tower_vals: &TowerValues, now: IngameTimestamp) -> Color {
    let silver = 0.75;
    let heat = match tower_vals.tower_status {
        TowerStatus::Reloading(finish) => {
            time_to_percent_inverted(now, finish, tower_vals.reload_duration)
        }
        TowerStatus::Waiting => 0.,
        TowerStatus::Shooting(finish) => {
            tower_vals.reload_duration.as_secs_f32()
                - time_to_percent_inverted(now, finish, tower_vals.shoot_duration)
        }
    };
    Srgba {
        red: silver + (heat * 0.25),
        green: silver - (heat * 0.75),
        blue: silver - (heat * 0.75),
        alpha: 1.,
    }
    .into()
}

fn time_to_percent_inverted(
    now: IngameTimestamp,
    die_time: IngameTimestamp,
    lifetime: Duration,
) -> f32 {
    let elapsed = *die_time - *now;
    (elapsed / lifetime.as_secs_f32()).abs()
}

fn shoot_or_reload(
    actions: &mut EventWriter<TowerActionsEvent>,
    vals: &mut TowerValues,
    enemy: Option<EntityEnemy>,
    now: IngameTimestamp,
) {
    match vals.tower_status {
        TowerStatus::Reloading(time_finish) => {
            if now >= time_finish {
                vals.tower_status = TowerStatus::Waiting;
            }
        }
        TowerStatus::Waiting => {
            if shoot(actions, &vals.shot, enemy) {
                vals.tower_status = TowerStatus::Shooting(now + vals.shoot_duration);
            }
        }
        TowerStatus::Shooting(time_finish) => {
            if now >= time_finish || enemy.is_none() {
                let rl_dur = vals.reload_duration.as_secs_f32() - (*(time_finish - now)).abs();
                set_tower_status_reload(vals, now + rl_dur);
            }
        }
    };
}

fn set_tower_status_reload(tower_vals: &mut TowerValues, finish: IngameTimestamp) {
    tower_vals.tower_status = TowerStatus::Reloading(finish);
}

fn shoot(
    actions: &mut EventWriter<TowerActionsEvent>,
    shot: &Shot,
    enemy: Option<EntityEnemy>,
) -> bool {
    match shot {
        Shot::Laser(shot) => {
            if let Some((entity, _)) = enemy {
                actions.send(TowerActionsEvent::ShootLaser(shot.clone(), entity));
                return true;
            }
        }
        Shot::Rocket(shot) => {
            if let Some((entity, _)) = enemy {
                actions.send(TowerActionsEvent::ShootRocket(shot.clone(), entity));
                return true;
            }
        }
    };
    false
}
