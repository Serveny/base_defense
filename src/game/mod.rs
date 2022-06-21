use self::{
    controls::{keyboard_input, mouse_input},
    visualisation::Visualisation,
};
use crate::{
    board::ActionBoard,
    utils::{despawn_all_of, Difficulty},
    GameState,
};
use bevy::prelude::*;

mod controls;
mod visualisation;

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(game)
                    .with_system(keyboard_input)
                    .with_system(mouse_input),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(despawn_all_of::<GameScreen>),
            );
    }
}

#[allow(dead_code)]
pub(crate) struct Game {
    action_board: ActionBoard,
    difficulty: Difficulty,
}

impl Game {
    pub fn new(board: ActionBoard, difficulty: Difficulty) -> Self {
        Self {
            action_board: board,
            difficulty,
        }
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct GameScreen;

fn game_setup(mut cmds: Commands, windows: Res<Windows>, game: Res<Game>) {
    let win = windows.get_primary().unwrap();
    let visu = Visualisation::new(win, &game.action_board, 0.);
    visu.draw_board(&mut cmds, &game.action_board);
    cmds.insert_resource(visu);
}

// Tick the timer, and change state when finished
fn game() {}
