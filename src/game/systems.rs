use self::{
    base::base_system,
    building::{factory_system, power_plant_system},
    collision::enemy_collision_remove_system,
    death::death_system,
    enemy::{enemy_collision_add_system, enemy_walk_system},
    explosions::explosion_system,
    game_over::{game_over_screen, game_over_system, game_over_timer_system},
    health_bar::health_bar_system,
    resource::{resource_animation_system, resource_symbol_fade_system, resource_text_fade_system},
    resource_bar::resource_bar_system,
    speed::acceleration_system,
    tower::{tower_overheat_system, tower_rotation_system, tower_target_system},
    wave::{wave_spawn_system, wave_system, WaveState},
};
use bevy::prelude::*;

use super::IngameState;

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
        app.add_state(WaveState::None)
            .add_system_set(
                SystemSet::on_update(IngameState::Running)
                    .with_system(wave_spawn_system)
                    .with_system(acceleration_system)
                    .with_system(enemy_walk_system)
                    .with_system(enemy_collision_add_system.before("collision_add"))
                    .with_system(enemy_collision_remove_system.before("collision_remove"))
                    .with_system(shot::damage_per_time::damage_system)
                    .with_system(shot::damage_per_time::visual_system)
                    .with_system(shot::damage_per_time::despawn_system)
                    .with_system(shot::damage_in_radius_enemy_locked::fly_system)
                    .with_system(shot::damage_in_radius_enemy_locked::visual_system)
                    .with_system(shot::damage_in_radius_enemy_locked::damage_and_despawn_system)
                    .with_system(power_plant_system)
                    .with_system(factory_system)
                    .with_system(resource_animation_system)
                    .with_system(resource_text_fade_system)
                    .with_system(resource_symbol_fade_system)
                    .with_system(tower_target_system)
                    .with_system(tower_rotation_system)
                    .with_system(tower_overheat_system)
                    .with_system(health_bar_system)
                    .with_system(resource_bar_system)
                    .with_system(base_system)
                    .with_system(explosion_system)
                    .with_system(death_system)
                    .with_system(game_over_timer_system)
                    .with_system(game_over_system),
            )
            .add_system_set(
                SystemSet::on_update(IngameState::GameOver).with_system(game_over_screen),
            )
            .add_system_set(
                SystemSet::on_update(WaveState::Running).with_system(wave_system.before("actions")),
            );
    }
}
