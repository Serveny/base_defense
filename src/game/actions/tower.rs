use crate::{
    game::GameScreen,
    utils::{
        shots::{laser::spawn_shot_laser, DamagePerTimeShotValues},
        IngameTime,
    },
};
use bevy::prelude::*;

pub enum TowerActionsEvent {
    ShootLaser(DamagePerTimeShotValues, Entity),
}

pub fn on_tower_actions(
    mut cmds: Commands,
    mut actions: EventReader<TowerActionsEvent>,
    time: Res<IngameTime>,
) {
    for action in actions.iter() {
        match action {
            TowerActionsEvent::ShootLaser(shot, enemy_entity) => {
                spawn_shot_laser::<GameScreen>(
                    &mut cmds,
                    shot.new_shot(*enemy_entity, time.now() + shot.lifetime),
                );
            }
        }
    }
}
