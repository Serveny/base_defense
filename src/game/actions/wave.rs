use crate::{
    game::{
        systems::wave::{Wave, WaveState},
        Game,
    },
    utils::{wave::WaveText, IngameTime, IngameTimestamp},
};
use bevy::prelude::*;
use std::time::Duration;

pub enum WaveActionsEvent {
    StartWave,
    EndWave,
}

pub(in crate::game) fn on_wave_actions(
    mut cmds: Commands,
    mut actions: EventReader<WaveActionsEvent>,
    mut game: ResMut<Game>,
    mut wave_state: ResMut<State<WaveState>>,
    mut q_wave_text: Query<&mut Text, With<WaveText>>,
    time: Res<IngameTime>,
) {
    if !actions.is_empty() {
        for action in actions.iter() {
            match action {
                WaveActionsEvent::StartWave => {
                    start_wave(&mut cmds, &mut game, &mut wave_state, time.now());
                    q_wave_text.single_mut().sections[0].value = format!("{}", game.wave_no);
                }
                WaveActionsEvent::EndWave => {
                    end_wave_and_prepare_next(&mut game, &mut wave_state, time.now())
                }
            }
        }
    }
}

fn start_wave(
    cmds: &mut Commands,
    game: &mut Game,
    wave_state: &mut State<WaveState>,
    now: IngameTimestamp,
) {
    game.next_wave_spawn = None;
    game.wave_no += 1;

    cmds.insert_resource(Wave::new(game.wave_no, now));
    wave_state.set(WaveState::Running).unwrap();
}

fn end_wave_and_prepare_next(
    game: &mut Game,
    wave_state: &mut State<WaveState>,
    now: IngameTimestamp,
) {
    wave_state.set(WaveState::None).unwrap();
    game.next_wave_spawn = Some(now + Duration::from_secs(1));
}
