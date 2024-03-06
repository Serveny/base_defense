use self::{
    base::base_system,
    building::{factory_system, power_plant_system},
    collision::enemy_collision_remove_system,
    death::death_system,
    enemy::{enemy_collision_add_system, enemy_walk_system},
    explosions::explosion_system,
    game_over::{end_game, game_over_screen, game_over_system, game_over_timer_system},
    health_bar::health_bar_system,
    resource::{resource_animation_system, resource_symbol_fade_system, resource_text_fade_system},
    resource_bar::resource_bar_system,
    speed::acceleration_system,
    tower::{tower_overheat_system, tower_rotation_system, tower_target_system},
    wave::{wave_spawn_system, wave_system, WaveState},
};
use bevy::prelude::*;

use super::{actions::Labels, controls::hovered_tile, IngameState};

pub mod base;
pub mod building;
pub mod collision;
pub mod death;
pub mod enemy;
pub mod explosions;
pub mod game_over;
pub mod health_bar;
pub mod resource;
pub mod resource_bar;
pub mod shot;
pub mod speed;
pub mod tower;
pub mod wave;

pub struct GameSystems;

impl Plugin for GameSystems {
    fn build(&self, app: &mut App) {
        app.init_state::<WaveState>()
            .add_systems(
                Update,
                (
                    wave_spawn_system,
                    acceleration_system,
                    enemy_walk_system,
                    power_plant_system,
                    factory_system,
                    resource_animation_system,
                    resource_text_fade_system,
                    resource_symbol_fade_system,
                    tower_target_system,
                    tower_rotation_system,
                    tower_overheat_system,
                    health_bar_system,
                    resource_bar_system,
                    base_system,
                    explosion_system,
                    death_system,
                    game_over_timer_system,
                    hovered_tile,
                    game_over_system,
                )
                    .run_if(in_state(IngameState::Running)),
            )
            .add_systems(
                Update,
                enemy_collision_add_system
                    .before(Labels::CollisionAdd)
                    .run_if(in_state(IngameState::Running)),
            )
            .add_systems(
                Update,
                enemy_collision_remove_system
                    .before(Labels::CollisionRemove)
                    .run_if(in_state(IngameState::Running)),
            )
            .add_systems(
                Update,
                (
                    shot::damage_per_time::damage_system,
                    shot::damage_per_time::visual_system,
                    shot::damage_per_time::despawn_system,
                    shot::damage_in_radius_enemy_locked::fly_system,
                    shot::damage_in_radius_enemy_locked::visual_system,
                    shot::damage_in_radius_enemy_locked::damage_and_despawn_system,
                )
                    .run_if(in_state(IngameState::Running)),
            )
            .add_systems(OnEnter(IngameState::GameOver), end_game)
            .add_systems(
                Update,
                (game_over_screen).run_if(in_state(IngameState::GameOver)),
            )
            .add_systems(
                Update,
                (wave_system)
                    .before(Labels::Actions)
                    .run_if(in_state(WaveState::Running)),
            );
    }
}
