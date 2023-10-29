use super::wave::WaveState;
use crate::{
    board::visualisation::{BoardRoadEndMark, GameOverCountDownText},
    game::{
        statistics::{EnemyKillCount, LaserShotsFired, RocketsFired},
        Game, IngameState, GAME_OVER_COUNTDOWN_TIME,
    },
    utils::{add_text_row, GameState, IngameTime, IngameTimestamp},
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{CentralPanel, Frame, Label, RichText, ScrollArea, Stroke, TopBottomPanel},
    EguiContexts,
};
use bevy_prototype_lyon::prelude::Fill;

#[derive(Resource)]
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
    q_base: Query<&mut Fill, With<BoardRoadEndMark>>,
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
            *text.1 = Visibility::Visible;
        }
    } else if let GameOverTimer::Active(_) = *go_timer {
        *go_timer = GameOverTimer::Inactive;
        set_base_color(q_base, IngameTimestamp(0.5));
        let mut text = q_go_text.single_mut();
        text.0.sections[0].value = format!("{}", GAME_OVER_COUNTDOWN_TIME.as_secs());
        *text.1 = Visibility::Hidden;
    }
}

fn set_base_color(mut q_base: Query<&mut Fill, With<BoardRoadEndMark>>, time: IngameTimestamp) {
    q_base.for_each_mut(|mut fill| {
        let rg_val = *time % 0.5;
        fill.color = Color::rgb(0.5 + rg_val, 0.5 - rg_val, 0.);
    });
}

pub(super) fn game_over_system(
    mut set_ingame_state: ResMut<NextState<IngameState>>,
    go_timer: Res<GameOverTimer>,
    time: Res<IngameTime>,
) {
    if let GameOverTimer::Active(time_game_over) = go_timer.as_ref() {
        if *time_game_over <= time.now() {
            set_ingame_state.set(IngameState::GameOver);
        }
    }
}

fn format_secs_time(secs: f64) -> String {
    let hours = (secs / 3600.).floor();
    let mins = ((secs % 3600.) / 60.).floor();
    let secs = (secs % 60.).floor();
    format!("{:02}:{:02}:{:02}", hours, mins, secs)
}

pub(super) fn end_game(mut set_wave_state: ResMut<NextState<WaveState>>) {
    set_wave_state.set(WaveState::None);
}

#[allow(clippy::too_many_arguments)]
pub(super) fn game_over_screen(
    mut egui_ctx: EguiContexts,
    mut set_game_state: ResMut<NextState<GameState>>,
    mut set_ingame_state: ResMut<NextState<IngameState>>,
    game: Res<Game>,
    kill_count: Res<EnemyKillCount>,
    laser_count: Res<LaserShotsFired>,
    rocket_count: Res<RocketsFired>,
    time: Res<IngameTime>,
) {
    CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(ui.available_height());
        ui.vertical_centered(|ui| {
            ui.add(Label::new(RichText::new("GAME OVER").heading()));

            // Game Over Infos
            ScrollArea::vertical().max_width(400.).show(ui, |ui| {
                let time = time.elapsed_secs_f64();
                add_text_row("Ingame Time", &format_secs_time(time), ui);
                add_text_row("Wave", &format!("{}", game.wave_no), ui);
                add_text_row("Energy", &format!("{}", game.energy), ui);
                add_text_row("Materials", &format!("{}", game.materials), ui);
                add_text_row("Enemies Killed", &format!("{}", kill_count.0), ui);
                add_text_row("Laser Shots Fired", &format!("{}", laser_count.0), ui);
                add_text_row("Rockets Fired", &format!("{}", rocket_count.0), ui);
            });
        });

        // Back to main menu button
        TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .default_height(60.)
            .frame(Frame {
                stroke: Stroke {
                    width: 0.,
                    ..Default::default()
                },
                ..Default::default()
            })
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    if ui
                        .add_sized(
                            [400., 60.],
                            bevy_egui::egui::widgets::Button::new("Back To Main Menu"),
                        )
                        .clicked()
                    {
                        set_ingame_state.set(IngameState::None);
                        set_game_state.set(GameState::Menu);
                    }
                });
            });
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format_secs_time_1() {
        assert_eq!(format_secs_time(3661.), String::from("01:01:01"));
    }
    #[test]
    fn test_format_secs_time_2() {
        assert_eq!(format_secs_time(1.), String::from("00:00:01"));
    }
}
