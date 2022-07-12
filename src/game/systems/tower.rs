use crate::{
    board::visualisation::TILE_SIZE,
    game::{actions::tower::TowerActionsEvent, enemies::Enemy, tower_build_menu::TowerMenuScreen},
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
    Query<'w, 's, (&'a Tower, &'a Children), Without<TowerMenuScreen>>;
type QueryCannonTransMut<'w, 's, 'a> =
    Query<'w, 's, &'a mut Transform, (With<TowerCannon>, Without<TowerMenuScreen>)>;
type QueryCannonDrawMut<'w, 's, 'a> =
    Query<'w, 's, &'a mut DrawMode, (With<TowerCannon>, Without<TowerMenuScreen>)>;
type EnemiesQuery<'w, 's, 'a> = Query<'w, 's, (Entity, &'a Enemy, &'a Children)>;
type EntityEnemy<'a> = (Entity, &'a Enemy);

pub(super) fn tower_target_system(
    mut actions: EventWriter<TowerActionsEvent>,
    mut q_towers: Query<&mut Tower, Without<TowerMenuScreen>>,
    q_enemies: EnemiesQuery,
    time: Res<IngameTime>,
) {
    for mut tower in q_towers.iter_mut() {
        let vals = tower.values_mut();
        let enemy = lock_tower_to_enemy(vals, &q_enemies);
        shoot_or_reload(&mut actions, vals, enemy, time.now());
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

//pub(super) fn tower_system(
//mut cannons: CannonQuery,
//mut towers: QueryTowers,
//mut actions: EventWriter<TowerActionsEvent>,
//enemies: EnemiesQuery,
//time: Res<IngameTime>,
//) {
//let now = time.now();
//for (mut tower, tower_children) in towers.iter_mut() {
//let tower_vals = tower.values_mut();
//let enemy = lock_tower_to_enemy(tower_vals, &enemies);
//if let TowerStatus::Shooting(finish) = tower_vals.tower_status {
//let time_left = tower_vals.shoot_duration.as_secs_f32() - (*finish - *now);
//let earier_finish = *now + tower_vals.reload_duration.as_secs_f32() - time_left;
//set_tower_status_reload(tower_vals, earier_finish.into());
//}

//let cannon = cannon_mut(&mut cannons, tower_children).expect("Every tower needs a cannon");

//rotate_cannon_to_enemy(cannon.0.into_inner(), tower_vals, enemy);
//overheat_cannon(cannon.1.into_inner(), tower_vals, now);
//shoot_or_reload(&mut actions, tower_vals, enemy, now);
//}
//}

fn lock_tower_to_enemy<'a>(
    tower_vals: &mut TowerValues,
    enemies: &'a EnemiesQuery,
) -> Option<EntityEnemy<'a>> {
    if let Some(locked_entity) = tower_vals.target_lock {
        if let Some(locked_enemy) =
            find_locked_enemy_in_tower_range(locked_entity, enemies, tower_vals)
        {
            return Some((locked_entity, locked_enemy));
        } else {
            tower_vals.target_lock = None;
        }
    } else {
        tower_vals.target_lock = find_first_enemy_entity_in_range(tower_vals, enemies);
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
) -> Option<Mut<'a, DrawMode>> {
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

fn overheat_cannon(mut draw_mode: Mut<DrawMode>, tower_vals: &TowerValues, now: IngameTimestamp) {
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
    enemy: Option<EntityEnemy>,
    now: IngameTimestamp,
) {
    match tower_vals.tower_status {
        TowerStatus::Reloading(time_finish) => {
            if now >= time_finish {
                tower_vals.tower_status = TowerStatus::Waiting;
            }
        }
        TowerStatus::Waiting => {
            if shoot(actions, &tower_vals.shot, enemy) {
                tower_vals.tower_status = TowerStatus::Shooting(now + tower_vals.shoot_duration);
            }
        }
        TowerStatus::Shooting(time_finish) => {
            if now >= time_finish {
                set_tower_status_reload(tower_vals, now + tower_vals.reload_duration);
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
