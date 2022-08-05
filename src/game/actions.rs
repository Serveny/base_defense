use crate::utils::{towers::TowerRangeCircle, GameState};
use bevy::prelude::*;

use self::{
    build_menu::{on_tower_menu_actions, BuildMenuActionsEvent},
    damage::{on_damage, DamageEvent},
    explosions::{on_explosions, ExplosionEvent},
    resources::{on_change_resources, ResourcesEvent},
    tile::{on_tile_actions, TileActionsEvent},
    tower::{on_tower_actions, TowerActionsEvent},
    wave::{on_wave_actions, WaveActionsEvent},
};

use super::{build_menus::BuildMenuScreen, systems::wave::WaveState, Game, GameScreen};

pub(super) mod build_menu;
pub(super) mod damage;
pub(super) mod explosions;
pub(super) mod resources;
pub(super) mod tile;
pub(super) mod tower;
pub(super) mod wave;

type RangeCircleQuery<'w, 's, 'a> =
    Query<'w, 's, (&'a mut Visibility, &'a TowerRangeCircle), Without<BuildMenuScreen>>;

type GameScreenQuery<'w, 's> = Query<'w, 's, Entity, With<GameScreen>>;

type GameActionQueries<'w, 's, 'a> =
    ParamSet<'w, 's, (GameScreenQuery<'w, 's>, RangeCircleQuery<'w, 's, 'a>)>;

pub enum GameActionEvent {
    BackToMainMenu,
    ActivateOverview,
    DeactivateOverview,
    Speed(f32),
}

pub struct GameActions;

impl Plugin for GameActions {
    fn build(&self, app: &mut App) {
        app.add_event::<GameActionEvent>()
            .add_event::<WaveActionsEvent>()
            .add_event::<ResourcesEvent>()
            .add_event::<TileActionsEvent>()
            .add_event::<TowerActionsEvent>()
            .add_event::<BuildMenuActionsEvent>()
            .add_event::<DamageEvent>()
            .add_event::<ExplosionEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(on_game_actions.label("actions"))
                    .with_system(on_wave_actions.label("actions"))
                    .with_system(on_change_resources)
                    .with_system(on_tile_actions)
                    .with_system(on_tower_actions)
                    .with_system(on_tower_menu_actions)
                    .with_system(on_damage)
                    .with_system(on_explosions),
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
            use GameActionEvent::*;
            match event {
                BackToMainMenu => back_to_main_menu(
                    &mut cmds,
                    &mut game_state,
                    &mut wave_state,
                    &mut queries.p0(),
                ),
                ActivateOverview => {
                    game.is_overview = true;
                    set_range_circles(&mut queries.p1(), true);
                }
                DeactivateOverview => {
                    game.is_overview = false;
                    set_range_circles(&mut queries.p1(), false);
                }
                Speed(speed) => game.speed = *speed,
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
