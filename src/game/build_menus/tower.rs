use super::{menu_circle_shape, BaseLevel, BuildMenu, BuildMenuCircle, BuildMenuScreen};
use crate::{board::visualisation::TILE_SIZE, game::actions::build_menu::BuildMenuActionsEvent};
use bevy::prelude::*;

pub fn draw_tower_build_menu(
    cmds: &mut Commands,
    mut actions: EventWriter<BuildMenuActionsEvent>,
    base_lvl: BaseLevel,
) {
    cmds.spawn_bundle(menu_circle_shape(TILE_SIZE))
        .insert(BuildMenuCircle)
        .insert(BuildMenuScreen);

    let mut towers = BuildMenu::available_towers(base_lvl);
    while let Some(tower) = towers.pop() {
        // println!("{:?}", tower);
        tower.draw_preview::<BuildMenuScreen>(cmds);
    }
    actions.send(BuildMenuActionsEvent::Close);
}
