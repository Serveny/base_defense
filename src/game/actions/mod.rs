use crate::utils::{towers::TowerRangeCircle, GameState};
use bevy::prelude::*;

use super::{systems::wave::WaveState, Game, GameScreen};

pub(super) mod tile;
pub(super) mod tower;
pub(super) mod tower_menu;
pub(super) mod wave;

type RangeCircleQuery<'w, 's, 'a> =
    Query<'w, 's, (&'a mut Visibility, &'a TowerRangeCircle), With<TowerRangeCircle>>;

type GameScreenQuery<'w, 's> = Query<'w, 's, Entity, With<GameScreen>>;

type GameActionQueries<'w, 's, 'a> =
    ParamSet<'w, 's, (GameScreenQuery<'w, 's>, RangeCircleQuery<'w, 's, 'a>)>;

pub enum GameActionEvent {
    BackToMainMenu,
    ActivateOverview,
    DeactivateOverview,
}

pub(super) fn game_actions(
    mut cmds: Commands,
    mut game: ResMut<Game>,
    mut game_state: ResMut<State<GameState>>,
    mut wave_state: ResMut<State<WaveState>>,
    mut game_actions: EventReader<GameActionEvent>,
    mut queries: GameActionQueries,
) {
    if !game_actions.is_empty() {
        for event in game_actions.iter() {
            match event {
                GameActionEvent::BackToMainMenu => back_to_main_menu(
                    &mut cmds,
                    &mut game_state,
                    &mut wave_state,
                    &mut queries.p0(),
                ),
                GameActionEvent::ActivateOverview => {
                    game.is_overview = true;
                    set_range_circles(&mut queries.p1(), true);
                }
                GameActionEvent::DeactivateOverview => {
                    game.is_overview = false;
                    set_range_circles(&mut queries.p1(), false);
                }
            }
        }
    }
}

fn set_range_circles(query: &mut RangeCircleQuery, is_visible: bool) {
    query.for_each_mut(|(mut visi, _)| visi.is_visible = is_visible);
}

fn back_to_main_menu(
    cmds: &mut Commands,
    game_state: &mut State<GameState>,
    wave_state: &mut State<WaveState>,
    query: &mut GameScreenQuery,
) {
    for entity in query.iter() {
        cmds.entity(entity).despawn_recursive();
    }
    wave_state.set(WaveState::None).unwrap_or_default();
    game_state.set(GameState::Menu).unwrap();
}
