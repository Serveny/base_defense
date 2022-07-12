use crate::{
    game::GameScreen,
    utils::{
        shots::{
            laser::spawn_shot_laser, rocket::spawn_shot_rocket,
            DamageInRadiusEnemyLockedShotValues, DamagePerTimeShotValues,
        },
        IngameTime,
    },
};
use bevy::prelude::*;

pub enum TowerActionsEvent {
    ShootLaser(DamagePerTimeShotValues, Entity),
    ShootRocket(DamageInRadiusEnemyLockedShotValues, Entity),
}

pub fn on_tower_actions(
    mut cmds: Commands,
    mut actions: EventReader<TowerActionsEvent>,
    time: Res<IngameTime>,
) {
    use TowerActionsEvent::*;
    for action in actions.iter() {
        match action {
            ShootLaser(shot, enemy_entity) => {
                spawn_shot_laser::<GameScreen>(
                    &mut cmds,
                    shot.new_shot(*enemy_entity, time.now() + shot.lifetime),
                );
            }
            ShootRocket(shot, enemy_entity) => {
                spawn_shot_rocket::<GameScreen>(&mut cmds, shot.new_shot(*enemy_entity))
            }
        }
    }
}
