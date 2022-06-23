use std::time::Duration;

use self::{
    controls::{keyboard_input, mouse_input},
    enemies::{enemies_walk_until_wave_end, Enemy},
    visualisation::Visualisation,
};
use crate::{
    board::ActionBoard,
    game::enemies::spawn_enemies,
    utils::{despawn_all_of, Difficulty, Energy, Materials},
    GameState,
};
use bevy::{prelude::*, utils::Instant};

mod controls;
mod enemies;
mod visualisation;

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(keyboard_input)
                    .with_system(mouse_input)
                    .with_system(game),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(despawn_all_of::<GameScreen>),
            );
    }
}

struct Wave {
    wave_no: u32,
    next_wave_time: Option<Instant>,
    enemies_spawned: u32,
    next_enemy_spawn: Option<Instant>,
}

impl Wave {
    fn new(next_wave_time: Option<Instant>) -> Self {
        Self {
            wave_no: 0,
            next_wave_time,
            enemies_spawned: 0,
            next_enemy_spawn: next_wave_time,
        }
    }
    fn start(&mut self) {
        self.wave_no += 1;
        self.enemies_spawned = 0;
        self.next_enemy_spawn = self.next_wave_time;
        self.next_wave_time = None;
    }
    fn end(&mut self, next_wave_time: Option<Instant>) {
        self.next_wave_time = next_wave_time;
    }
}

#[allow(dead_code)]
pub(crate) struct Game {
    action_board: ActionBoard,
    difficulty: Difficulty,
    energy: Energy,
    materials: Materials,
    wave: Wave,
}

impl Game {
    pub fn new(board: ActionBoard, difficulty: Difficulty) -> Self {
        Self {
            action_board: board,
            difficulty,
            energy: 100,
            materials: 100,
            wave: Wave::new(Some(Instant::now() + Duration::from_secs(1))),
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
fn game(
    mut cmds: Commands,
    mut game: ResMut<Game>,
    visu: Res<Visualisation>,
    time: Res<Time>,
    query: Query<(Entity, &mut Enemy, &mut Transform), With<Enemy>>,
) {
    if let Some(last_update) = time.last_update() {
        if let Some(next_wave_time) = game.wave.next_wave_time {
            if last_update >= next_wave_time {
                game.wave.start();
            }
        } else {
            spawn_enemies(&mut cmds, &mut game, &visu, last_update);
            if enemies_walk_until_wave_end(
                &mut cmds,
                time.delta(),
                query,
                &visu,
                game.action_board.road_tile_posis(),
            ) && game.wave.enemies_spawned >= game.wave.wave_no * 4
            {
                game.wave.end(Some(Instant::now() + Duration::from_secs(1)));
            }
        }
    }
}
