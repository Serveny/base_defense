#![allow(unused)]
use crate::{
    board::{draw_board, ActionBoard, Board},
    utils::despawn_all_of,
    GameState,
};
use bevy::prelude::*;

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(game_setup)
                .with_system(draw_ingame_board),
        )
        .add_system_set(SystemSet::on_update(GameState::Game).with_system(game))
        .add_system_set(
            SystemSet::on_exit(GameState::Game).with_system(despawn_all_of::<OnGameScreen>),
        );
    }
}

pub(crate) struct Game {
    board: ActionBoard,
}
impl Game {
    pub fn new(board: ActionBoard) -> Self {
        Self { board }
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

fn game_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// Tick the timer, and change state when finished
fn game() {}

fn draw_ingame_board(mut cmds: Commands, windows: Res<Windows>, game: Res<Game>) {
    let window = windows.get_primary().unwrap();
    draw_board(
        &mut cmds,
        &game.board,
        Vec2::new(400., 400.),
        Vec2::new(window.width() - 20., window.height() - 20.),
    );
}
