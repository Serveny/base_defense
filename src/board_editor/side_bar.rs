use super::LEFT_BAR_WIDTH_PX;
use crate::board::Tile;
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Label, RadioButton, SidePanel},
    EguiContexts,
};

#[derive(States, Clone, Copy, Eq, PartialEq, Debug, Hash, Default)]
pub(super) enum SettileState {
    #[default]
    TowerGround,
    BuildingGround,
    Road,
}

impl SettileState {
    pub fn as_tile(&self) -> Tile {
        match self {
            SettileState::TowerGround => Tile::TowerGround,
            SettileState::BuildingGround => Tile::BuildingGround,
            SettileState::Road => Tile::Road,
        }
    }
}

pub(super) fn add_side_bar(
    mut egui_ctx: EguiContexts,
    mut set_tile_state: ResMut<NextState<SettileState>>,
    tile_state: Res<State<SettileState>>,
) {
    SidePanel::left("map_editor_left_bar")
        .resizable(false)
        .default_width(LEFT_BAR_WIDTH_PX)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.add_sized([LEFT_BAR_WIDTH_PX - 20., 40.], Label::new("tile type"));
            add_tile_radio_button(
                &mut set_tile_state,
                &tile_state,
                SettileState::TowerGround,
                "Tower",
                ui,
            );
            add_tile_radio_button(
                &mut set_tile_state,
                &tile_state,
                SettileState::BuildingGround,
                "Building",
                ui,
            );
            add_tile_radio_button(
                &mut set_tile_state,
                &tile_state,
                SettileState::Road,
                "Road",
                ui,
            );
        });
}

fn add_tile_radio_button(
    set_current_state: &mut NextState<SettileState>,
    current_state: &State<SettileState>,
    state: SettileState,
    text: &str,
    ui: &mut egui::Ui,
) {
    if ui
        .add(RadioButton::new(*current_state == state, text))
        .clicked()
    {
        set_current_state.set(state);
    }
}
