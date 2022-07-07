use std::time::Duration;

use crate::{
    board::visualisation::TILE_SIZE,
    game::{actions::tower::TowerActionsEvent, enemies::Enemy, tower_build_menu::TowerMenuScreen},
    utils::{
        pos_to_quat,
        shots::{Target, TowerStatus},
        towers::{Tower, TowerCannon, TowerValues},
        IngameTime, IngameTimestamp, Vec2Board,
    },
};
use bevy::{prelude::*, reflect::Uuid};
use bevy_prototype_lyon::prelude::*;

type Towers<'w, 's, 'a> =
    Query<'w, 's, (&'a mut Tower, &'a Children), (With<Tower>, Without<TowerMenuScreen>)>;
type Cannon<'a> = (&'a mut Transform, &'a mut DrawMode);
type CannonQuery<'w, 's, 'a> =
    Query<'w, 's, Cannon<'a>, (With<TowerCannon>, Without<TowerMenuScreen>)>;
type EnemiesQuery<'w, 's, 'a> = Query<'w, 's, (Entity, &'a Enemy, &'a Children), With<Enemy>>;

pub(in crate::game) fn tower_system(
    mut cannons: CannonQuery,
    mut towers: Towers,
    mut actions: EventWriter<TowerActionsEvent>,
    enemies: EnemiesQuery,
    time: Res<IngameTime>,
) {
    let now = IngameTimestamp::new(time.elapsed_secs());
    for (mut tower, tower_children) in towers.iter_mut() {
        let tower_vals = tower.values_mut();
        let locked_enemy = lock_tower_to_enemy(tower_vals, &enemies);
        if locked_enemy.is_none() {
            if let TowerStatus::Shooting(finish) = tower_vals.tower_status {
                let time_left = 1. - (*finish - *now);
                let earier_finish = *now + tower_vals.reload_duration.as_secs_f32() - time_left;
                set_tower_status_reload(tower_vals, earier_finish.into());
            }
        }
        let cannon = cannon_mut(&mut cannons, tower_children).expect("Every tower needs a cannon");

        rotate_cannon_to_enemy(cannon.0.into_inner(), tower_vals, locked_enemy);
        overheat_cannon(cannon.1.into_inner(), tower_vals, now);
        shoot_or_reload(&mut actions, tower_vals, locked_enemy, now);
    }
}

fn lock_tower_to_enemy<'a>(
    tower_vals: &mut TowerValues,
    enemies: &'a EnemiesQuery,
) -> Option<&'a Enemy> {
    if let Some(locked_entity) = tower_vals.target_lock {
        if let Some(locked_enemy) =
            find_locked_enemy_in_tower_range(locked_entity, enemies, tower_vals)
        {
            return Some(locked_enemy);
        } else {
            tower_vals.target_lock = None;
        }
    } else {
        tower_vals.target_lock = find_first_enemy_entity_in_range(tower_vals, enemies);
    }
    None
}

fn rotate_cannon_to_enemy(
    transform: &mut Transform,
    tower_vals: &TowerValues,
    locked_enemy: Option<&Enemy>,
) {
    if let Some(locked_enemy) = locked_enemy {
        rotate_tower_cannon_to_pos(transform, locked_enemy.pos, tower_vals.pos);
    }
}

fn find_first_enemy_entity_in_range(
    tower_vals: &TowerValues,
    enemies: &EnemiesQuery,
) -> Option<Uuid> {
    for (_, enemy, _) in enemies.iter() {
        if enemy.is_in_range(tower_vals.pos, tower_vals.range_radius) {
            return Some(enemy.id);
        }
    }
    None
}

fn find_locked_enemy_in_tower_range<'a>(
    locked_enemy_id: Uuid,
    enemies: &'a EnemiesQuery,
    tower_vals: &TowerValues,
) -> Option<&'a Enemy> {
    for (_, enemy, _) in enemies.iter() {
        if enemy.id == locked_enemy_id {
            return if enemy.is_in_range(tower_vals.pos, tower_vals.range_radius) {
                Some(enemy)
            } else {
                None
            };
        }
    }
    None
}

//fn irgendwas_mut<'a>(vector_1: &'a mut Vec<u8>, vector_2: &Vec<u8>) -> Option<&'a mut u8> {
//for item in vector_2.iter() {
//if let Some(res) = vector_1.get_mut(*item as usize) {
//Some(res);
//}
//}
//None
//}

//fn cannon_mut_wrong<'a>(
//cannons: &'a mut CannonQuery,
//tower_children: &Children,
//) -> Option<(Mut<'a, Transform>, Mut<'a, DrawMode>)> {
//for child in tower_children.iter() {
//let cannon = cannons.get_mut(*child);
//if let Ok(entity) = cannon {
//return Some(entity);
//}
//}
//panic!();
//}

fn cannon_mut<'a>(
    cannons: &'a mut CannonQuery,
    tower_children: &Children,
) -> Option<(Mut<'a, Transform>, Mut<'a, DrawMode>)> {
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

fn overheat_cannon(draw_mode: &mut DrawMode, tower_vals: &TowerValues, now: IngameTimestamp) {
    *draw_mode = DrawMode::Outlined {
        fill_mode: FillMode::color(overheat_color(tower_vals, now)),
        outline_mode: StrokeMode::new(Color::DARK_GRAY, TILE_SIZE / 16.),
    };
}

fn overheat_color(tower_vals: &TowerValues, now: IngameTimestamp) -> Color {
    let silver = 0.75;
    let heat = match tower_vals.tower_status {
        TowerStatus::Reloading(finish) => {
            time_to_percent_inverted(now, finish, tower_vals.reload_duration)
        }
        TowerStatus::Waiting => 0.,
        TowerStatus::Shooting(finish) => {
            1. - time_to_percent_inverted(now, finish, tower_vals.shoot_duration)
        }
    };
    Color::Rgba {
        red: silver + (heat * 0.25),
        green: silver - (heat * 0.75),
        blue: silver - (heat * 0.75),
        alpha: 1.,
    }
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
                actions.send(TowerActionsEvent::Shoot(
                    tower_vals.shoot(Target::Enemy(enemy.id)),
                ));
                tower_vals.tower_status = TowerStatus::Shooting(now + tower_vals.shoot_duration);
            }
        }
        TowerStatus::Shooting(time_finish) => {
            if now >= time_finish {
                set_tower_status_reload(tower_vals, now);
            }
        }
    };
}

fn set_tower_status_reload(tower_vals: &mut TowerValues, finish: IngameTimestamp) {
    tower_vals.tower_status = TowerStatus::Reloading(finish);
}
