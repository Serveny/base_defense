use crate::{
    board::{step::BoardDirection, BoardCache},
    game::{actions::resources::ResourcesEvent, enemies::Enemy},
    utils::{IngameTime, Vec2Board},
};
use bevy::prelude::*;

pub(super) fn enemies_walk_system(
    mut cmds: Commands,
    mut res_actions: EventWriter<ResourcesEvent>,
    mut query: Query<(Entity, &mut Enemy, &mut Transform), With<Enemy>>,
    board_cache: Res<BoardCache>,
    time: Res<IngameTime>,
) {
    let dur = time.delta();
    query.for_each_mut(|(entity, mut enemy, mut transform)| {
        match enemy.walk_until_end(dur, &board_cache) {
            true => enemy_reached_base(&mut cmds, &mut res_actions, &enemy, entity),
            false => transform.translation = enemy.pos.to_scaled_vec3(1.),
        }
        set_enemy_spawn_line_flag(&mut enemy, &board_cache);
    });
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
        let size_radius = 2.;
        assert!(!is_in_spawn(start, pos, size_radius, direction))
    }
}
