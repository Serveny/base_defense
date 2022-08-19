use crate::{
    board::{step::BoardDirection, BoardCache},
    game::{
        actions::{collision::EnemyCollisionAddEvent, resources::ResourcesEvent},
        enemies::Enemy,
    },
    utils::{
        collision::{Collision, Collisions},
        speed::Speed,
        IngameTime, Vec2Board,
    },
};
use bevy::prelude::*;

type QEnemies<'w, 's, 'a> =
    Query<'w, 's, (Entity, &'a mut Enemy, &'a mut Transform, &'a Speed), With<Enemy>>;

pub(super) fn enemy_walk_system(
    mut cmds: Commands,
    mut res_actions: EventWriter<ResourcesEvent>,
    mut q_enemies: QEnemies,
    board_cache: Res<BoardCache>,
    time: Res<IngameTime>,
    collisions: Res<Collisions>,
) {
    let dur = time.delta();
    q_enemies.for_each_mut(|(entity, mut enemy, mut transform, speed)| {
        if !collisions
            .iter()
            .any(|coll| coll.enemy_behind == entity && coll.is_critical)
        {
            match enemy.walk_until_end(dur, speed.current, &board_cache) {
                true => enemy_reached_base(&mut cmds, &mut res_actions, &enemy, entity),
                false => transform.translation = enemy.pos.to_scaled_vec3(1.),
            }
            set_enemy_spawn_line_flag(&mut enemy, &board_cache);
        }
    });
}

pub(super) fn enemy_collision_add_system(
    mut add_ev: EventWriter<EnemyCollisionAddEvent>,
    mut collisions: ResMut<Collisions>,
    q_enemies: Query<(Entity, &Enemy)>,
) {
    q_enemies.for_each(|(entity, enemy)| {
        q_enemies.for_each(|(other_entity, other_enemy)| {
            if entity != other_entity {
                let distance = enemy.pos.distance(*other_enemy.pos);
                if distance <= enemy.break_radius + other_enemy.break_radius
                    && !is_already_found(entity, other_entity, &collisions)
                {
                    let is_critical = distance <= enemy.size_radius + other_enemy.size_radius;
                    collisions.push(if enemy.is_behind_of(other_enemy) {
                        Collision::new(other_entity, entity, is_critical)
                    } else {
                        Collision::new(entity, other_entity, is_critical)
                    });
                    add_ev.send(EnemyCollisionAddEvent(entity, other_entity));
                }
            }
        });
    });
}

fn is_already_found(entity_0: Entity, entity_1: Entity, collisions: &[Collision]) -> bool {
    collisions.iter().any(|ev| {
        (entity_0 == ev.enemy_before && entity_1 == ev.enemy_behind)
            || (entity_0 == ev.enemy_behind && entity_1 == ev.enemy_before)
    })
}

fn enemy_reached_base(
    cmds: &mut Commands,
    res_actions: &mut EventWriter<ResourcesEvent>,
    enemy: &Enemy,
    entity: Entity,
) {
    let damage = (-enemy.health * 20.).round();
    res_actions.send(ResourcesEvent::Energy(damage, enemy.pos));
    res_actions.send(ResourcesEvent::Materials(damage, enemy.pos));
    cmds.entity(entity).despawn_recursive();
}

fn set_enemy_spawn_line_flag(enemy: &mut Enemy, board_cache: &BoardCache) {
    if enemy.is_in_spawn {
        let start = Vec2Board::from(board_cache.road_start_pos.unwrap());
        let step = board_cache.road_path.first().unwrap();
        enemy.is_in_spawn = is_in_spawn(start, enemy.pos, enemy.size_radius, step.direction);
    }
}

fn is_in_spawn(
    start_pos: Vec2Board,
    enemy_pos: Vec2Board,
    size_radius: f32,
    first_step_dir: BoardDirection,
) -> bool {
    use BoardDirection::*;
    let distance = match first_step_dir {
        East | West => enemy_pos.x - start_pos.x,
        North | South => enemy_pos.y - start_pos.y,
    }
    .abs();
    distance <= size_radius * 2.
}

#[cfg(test)]
mod tests {
    use crate::{board::step::BoardDirection, utils::Vec2Board};

    use super::is_in_spawn;

    #[test]
    fn test_is_in_spawn() {
        let start = Vec2Board::new(0., 4.);
        let direction = BoardDirection::East;
        let pos = Vec2Board::new(1.2, 3.);
        let size_radius = 2.;
        assert!(is_in_spawn(start, pos, size_radius, direction))
    }

    #[test]
    fn test_out_of_spawn() {
        let start = Vec2Board::new(0., 4.);
        let direction = BoardDirection::East;
        let pos = Vec2Board::new(2.2, 3.);
        let size_radius = 1.;
        assert!(!is_in_spawn(start, pos, size_radius, direction))
    }
}
