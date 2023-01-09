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
        IngameTime,
    },
};
use bevy::prelude::*;

pub enum TowerActionsEvent {
    ShootLaser(DamagePerTimeShotValues, Entity),
    ShootRocket(DamageInRadiusTargetPosShotValues, Entity),
}

pub fn on_tower_actions(
    mut cmds: Commands,
    mut actions: EventReader<TowerActionsEvent>,
    mut laser_count: ResMut<LaserShotsFired>,
    mut rocket_count: ResMut<RocketsFired>,
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
                laser_count.0 += 1;
            }
            ShootRocket(shot, enemy) => {
                spawn_shot_rocket::<GameScreen>(&mut cmds, shot.new_shot(*enemy));
                rocket_count.0 += 1;
            }
        }
    }
}
