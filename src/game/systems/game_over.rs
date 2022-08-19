use crate::{
    board::visualisation::{BoardRoadEndMark, GameOverCountDownText},
    game::{Game, GAME_OVER_COUNTDOWN_TIME},
    utils::{GameState, IngameTime, IngameTimestamp},
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::DrawMode;

pub(in crate::game) enum GameOverTimer {
    Active(IngameTimestamp),
    Inactive,
}

impl Default for GameOverTimer {
    fn default() -> Self {
        Self::Inactive
    }
}

pub(super) fn game_over_timer_system(
    mut go_timer: ResMut<GameOverTimer>,
    mut q_go_text: Query<(&mut Text, &mut Visibility), With<GameOverCountDownText>>,
    q_base: Query<&mut DrawMode, With<BoardRoadEndMark>>,
    time: Res<IngameTime>,
    game: Res<Game>,
) {
    if game.energy < 0. || game.materials < 0. {
        let mut text = q_go_text.single_mut();
        if let GameOverTimer::Active(game_over_time) = go_timer.as_ref() {
            set_base_color(q_base, time.now());
            text.0.sections[0].value = format!("{}", *(*game_over_time - *time.now()) as u32);
        } else {
            *go_timer = GameOverTimer::Active(time.now() + GAME_OVER_COUNTDOWN_TIME);
            text.1.is_visible = true;
        }
    } else if let GameOverTimer::Active(_) = *go_timer {
        *go_timer = GameOverTimer::Inactive;
        set_base_color(q_base, IngameTimestamp(0.5));
        let mut text = q_go_text.single_mut();
        text.0.sections[0].value = format!("{}", GAME_OVER_COUNTDOWN_TIME.as_secs());
        text.1.is_visible = false;
    }
}

fn set_base_color(mut q_base: Query<&mut DrawMode, With<BoardRoadEndMark>>, time: IngameTimestamp) {
    q_base.for_each_mut(|mut draw_mode| {
        let rg_val = *time % 0.5;
        if let DrawMode::Outlined {
            fill_mode,
            outline_mode: _,
        } = &mut *draw_mode
        {
            fill_mode.color = Color::rgb(0.5 + rg_val, 0.5 - rg_val, 0.)
        }
    });
}

pub(super) fn game_over_system(
    mut game_state: ResMut<State<GameState>>,
    go_timer: Res<GameOverTimer>,
    time: Res<IngameTime>,
) {
    if let GameOverTimer::Active(time_game_over) = go_timer.as_ref() {
        if *time_game_over <= time.now() {
            game_state.set(GameState::GameOver).unwrap();
        }
    }
}
