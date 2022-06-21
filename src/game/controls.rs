use bevy::prelude::*;

use crate::board::Tile;

use super::{
    visualisation::{HoverCross, Visualisation},
    Game,
};

pub(super) fn keyboard_input(keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        // TODO: Open ingame menu
    }
}

pub(super) fn mouse_input(
    mut cmds: Commands,
    windows: Res<Windows>,
    visu: Res<Visualisation>,
    game: Res<Game>,
    query_hover_cross: Query<(Entity, &mut Transform), With<HoverCross>>,
) {
    let win = windows.get_primary().unwrap();
    if let Some(cursor_pos) = win.cursor_position() {
        let pos = visu.cursor_px_to_board_pos(Vec2::new(
            cursor_pos.x - win.width() / 2.,
            cursor_pos.y - win.height() / 2.,
        ));
        if pos.x < 0. || pos.y < 0. {
            Visualisation::delete_hover_cross(&mut cmds, query_hover_cross);
        } else if let Some(tile) = game.action_board.try_get_tile(pos.as_uvec2()) {
            //println!("{:?}{:?}{:?}", pos, pos.as_uvec2(), tile);
            match tile {
                Tile::TowerGround(_) => visu.draw_hover_cross(&mut cmds, query_hover_cross, pos),
                Tile::BuildingGround(_) => visu.draw_hover_cross(&mut cmds, query_hover_cross, pos),
                _ => Visualisation::delete_hover_cross(&mut cmds, query_hover_cross),
            }
        } else {
            Visualisation::delete_hover_cross(&mut cmds, query_hover_cross);
        }
    }
}
