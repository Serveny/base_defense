use crate::{
    game::{
        statistics::{LaserShotsFired, RocketsFired},
        GameScreen,
    },
    utils::{
        shots::{
            laser::spawn_shot_laser, rocket::spawn_shot_rocket, DamageInRadiusTargetPosShotValues,
            DamagePerTimeShotValues,
        },
        IngameTime, Vec2Board,
    },
};
use bevy::prelude::*;

#[derive(Message)]
pub enum TowerActionsMessage {
    ShootLaser(DamagePerTimeShotValues, Entity),
    ShootRocket(DamageInRadiusTargetPosShotValues, Entity, Vec2Board),
}

pub fn on_tower_actions(
    mut cmds: Commands,
    mut actions: MessageReader<TowerActionsMessage>,
    mut laser_count: ResMut<LaserShotsFired>,
    mut rocket_count: ResMut<RocketsFired>,
    time: Res<IngameTime>,
) {
    use TowerActionsMessage::*;
    for action in actions.read() {
        match action {
            ShootLaser(shot, enemy_entity) => {
                spawn_shot_laser::<GameScreen>(
                    &mut cmds,
                    shot.new_shot(*enemy_entity, time.now() + shot.lifetime),
                );
                laser_count.0 += 1;
            }
            ShootRocket(shot, enemy, enemy_pos) => {
                spawn_shot_rocket::<GameScreen>(
                    &mut cmds,
                    shot.new_rocket_shot_from_cannon(*enemy, *enemy_pos),
                );
                rocket_count.0 += 1;
            }
        }
    }
}
