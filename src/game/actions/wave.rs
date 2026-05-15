use crate::{
    balance::NEXT_WAVE_DELAY_SECS,
    game::{
        systems::wave::{Wave, WaveState},
        Game,
    },
    utils::{wave::WaveText, IngameTime, IngameTimestamp},
};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Message)]
pub enum WaveActionsMessage {
    StartWave,
    EndWave,
}

pub(in crate::game) fn on_wave_actions(
    mut cmds: Commands,
    mut actions: MessageReader<WaveActionsMessage>,
    mut game: ResMut<Game>,
    mut set_wave_state: ResMut<NextState<WaveState>>,
    mut q_wave_text: Query<&mut Text2d, With<WaveText>>,
    time: Res<IngameTime>,
) {
    if !actions.is_empty() {
        for action in actions.read() {
            match action {
                WaveActionsMessage::StartWave => {
                    start_wave(&mut cmds, &mut game, &mut set_wave_state, time.now());
                    if let Ok(mut text) = q_wave_text.single_mut() {
                        text.0 = format!("{}", game.wave_no);
                    }
                }
                WaveActionsMessage::EndWave => {
                    end_wave_and_prepare_next(&mut game, &mut set_wave_state, time.now())
                }
            }
        }
    }
}

fn start_wave(
    cmds: &mut Commands,
    game: &mut Game,
    set_wave_state: &mut NextState<WaveState>,
    now: IngameTimestamp,
) {
    game.next_wave_spawn = None;
    game.wave_no += 1;

    cmds.insert_resource(Wave::new(game.wave_no, now));
    set_wave_state.set(WaveState::Running);
}

fn end_wave_and_prepare_next(
    game: &mut Game,
    set_wave_state: &mut NextState<WaveState>,
    now: IngameTimestamp,
) {
    set_wave_state.set(WaveState::None);
    game.next_wave_spawn = Some(now + Duration::from_secs_f32(NEXT_WAVE_DELAY_SECS));
}
