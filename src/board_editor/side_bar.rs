use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Label, RadioButton, SidePanel},
    EguiContext,
};

use crate::board::Tile;

use super::LEFT_BAR_WIDTH_PX;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub(super) enum SettileState {
    TowerGround,
    BuildingGround,
    Road,
}

pub(super) fn add_side_bar(
    mut egui_ctx: ResMut<EguiContext>,
    mut set_tile_state: ResMut<State<SettileState>>,
) {
    SidePanel::left("map_editor_left_bar")
        .resizable(false)
        .default_width(LEFT_BAR_WIDTH_PX)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.add_sized([LEFT_BAR_WIDTH_PX - 20., 40.], Label::new("tile type"));
            add_tile_radio_button(&mut set_tile_state, SettileState::TowerGround, "Tower", ui);
            add_tile_radio_button(
                &mut set_tile_state,
                SettileState::BuildingGround,
                "Building",
                ui,
            );
            add_tile_radio_button(&mut set_tile_state, SettileState::Road, "Road", ui);
        });
}

fn add_tile_radio_button(
    current_state: &mut ResMut<State<SettileState>>,
    state: SettileState,
    text: &str,
    ui: &mut egui::Ui,
) {
    if ui
        .add(RadioButton::new(*current_state.current() == state, text))
        .clicked()
    {
        current_state.set(state).unwrap_or_default();
    }
}

pub(super) fn settile_state_to_tile(settile_state: SettileState) -> Tile {
    match settile_state {
        SettileState::TowerGround => Tile::TowerGround(None),
        SettileState::BuildingGround => Tile::BuildingGround(None),
        SettileState::Road => Tile::Road,
    }
}
