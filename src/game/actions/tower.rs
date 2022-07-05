use crate::utils::shots::{laser_shape, Shot};
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
    cmds.spawn_bundle(laser_shape(1.)).insert(shot.clone());
    println!("Shoot: {:?}", shot);
}
