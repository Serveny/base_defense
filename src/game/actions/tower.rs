use crate::{
    game::GameScreen,
    utils::shots::{laser::spawn_shot_laser, rocket::spawn_shot_rocket, Shot},
};
use bevy::prelude::*;

pub enum TowerActionsEvent {
    Shoot(Shot),
}

pub fn on_tower_actions(mut cmds: Commands, mut actions: EventReader<TowerActionsEvent>) {
    if !actions.is_empty() {
        for action in actions.iter() {
            match action {
                TowerActionsEvent::Shoot(shot) => shoot(&mut cmds, shot),
            }
        }
    }
}

fn shoot(cmds: &mut Commands, shot: &Shot) {
    println!("{:?}", shot);
    match shot {
        Shot::Laser(_) => spawn_shot_laser::<GameScreen>(cmds, shot),
        Shot::Rocket(_) => spawn_shot_rocket::<GameScreen>(cmds, shot),
    }
}
