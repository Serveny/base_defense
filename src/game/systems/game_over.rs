use crate::{
    board::visualisation::{BoardRoadEndMark, GameOverCountDownText},
    game::{statistics::EnemyKillCount, Game, IngameState, GAME_OVER_COUNTDOWN_TIME},
    utils::{add_text_row, GameState, IngameTime, IngameTimestamp},
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{CentralPanel, Frame, Label, RichText, ScrollArea, Stroke, TopBottomPanel},
    EguiContext,
};
use bevy_prototype_lyon::prelude::DrawMode;

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
    mut ingame_state: ResMut<State<IngameState>>,
    go_timer: Res<GameOverTimer>,
    time: Res<IngameTime>,
) {
    if let GameOverTimer::Active(time_game_over) = go_timer.as_ref() {
        if *time_game_over <= time.now() {
            ingame_state.set(IngameState::GameOver).unwrap();
        }
    }
}

fn format_secs_time(secs: f64) -> String {
    let hours = (secs / 3600.).floor();
    let mins = ((secs % 3600.) / 60.).floor();
    let secs = (secs % 60.).floor();
    format!("{:02}:{:02}:{:02}", hours, mins, secs)
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

pub(super) fn game_over_screen(
    mut egui_ctx: ResMut<EguiContext>,
    mut game_state: ResMut<State<GameState>>,
    game: Res<Game>,
    kill_count: Res<EnemyKillCount>,
    time: Res<IngameTime>,
) {
    CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(ui.available_height());
        ui.vertical_centered(|ui| ui.add(Label::new(RichText::new("GAME OVER").heading())));

        // Game Over Infos
        ScrollArea::vertical().show(ui, |ui| {
            add_text_row(
                "Ingame Time",
                &format_secs_time(time.elapsed_secs_f64()),
                ui,
            );
            add_text_row("Wave", &format!("{}", game.wave_no), ui);
            add_text_row("Enemies Killed", &format!("{}", kill_count.0), ui);
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
                        game_state.set(GameState::Menu).unwrap();
                    }
                });
            });
    });
}
