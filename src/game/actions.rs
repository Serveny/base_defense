use crate::utils::{towers::TowerRangeCircle, GameState};
use bevy::prelude::*;

use self::{
    damage::{on_damage, DamageEvent},
    tile::{on_tile_actions, TileActionsEvent},
    tower::{on_tower_actions, TowerActionsEvent},
    tower_menu::{on_tower_menu_actions, TowerMenuActionsEvent},
    wave::{on_wave_actions, WaveActionsEvent},
};

use super::{systems::wave::WaveState, tower_build_menu::TowerMenuScreen, Game, GameScreen};

pub(super) mod damage;
pub(super) mod tile;
pub(super) mod tower;
pub(super) mod tower_menu;
pub(super) mod wave;

type RangeCircleQuery<'w, 's, 'a> =
    Query<'w, 's, (&'a mut Visibility, &'a TowerRangeCircle), Without<TowerMenuScreen>>;

type GameScreenQuery<'w, 's> = Query<'w, 's, Entity, With<GameScreen>>;

type GameActionQueries<'w, 's, 'a> =
    ParamSet<'w, 's, (GameScreenQuery<'w, 's>, RangeCircleQuery<'w, 's, 'a>)>;

pub enum GameActionEvent {
    BackToMainMenu,
    ActivateOverview,
    DeactivateOverview,
}

pub struct GameActions;

impl Plugin for GameActions {
    fn build(&self, app: &mut App) {
        app.add_event::<GameActionEvent>()
            .add_event::<WaveActionsEvent>()
            .add_event::<TileActionsEvent>()
            .add_event::<TowerActionsEvent>()
            .add_event::<TowerMenuActionsEvent>()
            .add_event::<DamageEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(on_game_actions.label("actions"))
                    .with_system(on_tower_actions)
                    .with_system(on_wave_actions.label("actions"))
                    .with_system(on_tower_menu_actions)
                    .with_system(on_damage)
                    .with_system(on_tile_actions),
            );
    }
}

fn on_game_actions(
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
