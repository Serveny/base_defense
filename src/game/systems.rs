use self::{
    death::death_system,
    health_bar::health_bar_system,
    tower::{tower_overheat_system, tower_rotation_system, tower_target_system},
    wave::{wave_spawn_system, wave_system, WaveState},
};
use crate::utils::GameState;
use bevy::prelude::*;

pub mod death;
pub mod health_bar;
pub mod shot;
pub mod tower;
pub mod wave;

pub struct GameSystems;

impl Plugin for GameSystems {
    fn build(&self, app: &mut App) {
        app.add_state(WaveState::None)
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(wave_spawn_system)
                    .with_system(shot::damage_per_time::damage_system)
                    .with_system(shot::damage_per_time::visual_system)
                    .with_system(shot::damage_per_time::despawn_system)
                    .with_system(shot::damage_in_radius_enemy_locked::fly_system)
                    .with_system(shot::damage_in_radius_enemy_locked::visual_system)
                    .with_system(shot::damage_in_radius_enemy_locked::damage_and_despawn_system)
                    .with_system(tower_target_system)
                    .with_system(tower_rotation_system)
                    .with_system(tower_overheat_system)
                    .with_system(health_bar_system)
                    .with_system(death_system),
            )
            .add_system_set(
                SystemSet::on_update(WaveState::Running).with_system(wave_system.before("actions")),
            );
    }
}