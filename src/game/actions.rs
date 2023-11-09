use crate::{
    board::visualisation::BoardHoverCross,
    utils::{towers::TowerRangeCircle, visible, GameState},
};
use bevy::prelude::*;

use self::{
    build_menu::{
        BuildMenuBuildEvent, BuildMenuCloseEvent, BuildMenuHideEvent, BuildMenuOpenEvent,
        BuildMenuScrollEvent,
    },
    collision::{
        on_enemy_collision_add, on_enemy_collision_remove, EnemyCollisionAddEvent,
        EnemyCollisionRemoveEvent,
    },
    damage::{on_damage, DamageEvent},
    explosions::{on_explosions, ExplosionEvent},
    resources::{on_change_resources, ResourcesEvent},
    tile::{on_tile_actions, TileActionsEvent},
    tower::{on_tower_actions, TowerActionsEvent},
    wave::{on_wave_actions, WaveActionsEvent},
};

use super::{
    build_menus::BuildMenuScreen, systems::wave::WaveState, Game, GameScreen, IngameState,
};

pub(super) mod build_menu;
pub(super) mod collision;
pub(super) mod damage;
pub(super) mod explosions;
pub(super) mod resources;
pub(super) mod tile;
pub(super) mod tower;
pub(super) mod wave;

type RangeCircleQuery<'w, 's, 'a> = Query<
    'w,
    's,
    (&'a mut Visibility, &'a TowerRangeCircle),
    (Without<BuildMenuScreen>, Without<BoardHoverCross>),
>;

type GameScreenQuery<'w, 's> = Query<'w, 's, Entity, With<GameScreen>>;

#[derive(Event)]
pub enum GameActionEvent {
    BackToMainMenu,
    ActivateOverview,
    DeactivateOverview,
    SpeedUp,
    SpeedDown,
    Speed(f32),
    Pause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum Labels {
    Actions,
    CollisionAdd,
    CollisionRemove,
}

pub struct GameActions;

impl Plugin for GameActions {
    fn build(&self, app: &mut App) {
        app.add_event::<GameActionEvent>()
            .add_event::<WaveActionsEvent>()
            .add_event::<ResourcesEvent>()
            .add_event::<TileActionsEvent>()
            .add_event::<TowerActionsEvent>()
            .add_event::<DamageEvent>()
            .add_event::<ExplosionEvent>()
            .add_event::<EnemyCollisionAddEvent>()
            .add_event::<EnemyCollisionRemoveEvent>()
            .add_event::<BuildMenuScrollEvent>()
            .add_event::<BuildMenuOpenEvent>()
            .add_event::<BuildMenuCloseEvent>()
            .add_event::<BuildMenuHideEvent>()
            .add_event::<BuildMenuBuildEvent>()
            .add_systems(
                Update,
                (
                    on_change_resources,
                    on_damage,
                    on_explosions,
                    on_tower_actions,
                    on_game_actions,
                    on_tile_actions,
                    build_menu::on_open,
                    build_menu::on_scroll,
                    build_menu::on_close,
                    build_menu::on_hide,
                    build_menu::on_build,
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                (/*on_game_actions,*/on_wave_actions)
                    .in_set(Labels::Actions)
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                on_enemy_collision_add
                    .in_set(Labels::CollisionAdd)
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                on_enemy_collision_remove
                    .in_set(Labels::CollisionRemove)
                    .run_if(in_state(GameState::Game)),
            );
    }
}

#[allow(clippy::too_many_arguments)]
fn on_game_actions(
    mut cmds: Commands,
    mut game: ResMut<Game>,
    mut set_game_state: ResMut<NextState<GameState>>,
    mut set_wave_state: ResMut<NextState<WaveState>>,
    mut game_actions: EventReader<GameActionEvent>,
    mut q_game_screen: GameScreenQuery,
    mut q_range_circle: RangeCircleQuery,
    mut set_ingame_state: ResMut<NextState<IngameState>>,
) {
    if !game_actions.is_empty() {
        for event in game_actions.read() {
            use GameActionEvent::*;
            match event {
                BackToMainMenu => back_to_main_menu(
                    &mut cmds,
                    &mut set_game_state,
                    &mut set_ingame_state,
                    &mut set_wave_state,
                    &mut q_game_screen,
                ),
                ActivateOverview => {
                    game.is_overview = true;
                    set_range_circles(&mut q_range_circle, true);
                }
                DeactivateOverview => {
                    game.is_overview = false;
                    set_range_circles(&mut q_range_circle, false);
                }
                SpeedUp => game.speed = (game.speed + 1.).clamp(0., 30.),
                SpeedDown => game.speed = (game.speed - 1.).clamp(0., 30.),
                Speed(speed) => {
                    game.speed = if game.speed == 0. && *speed == 0. {
                        1.
                    } else {
                        *speed
                    }
                }

                Pause => set_ingame_state.set(IngameState::Pause),
            }
        }
    }
}

fn set_range_circles(query: &mut RangeCircleQuery, is_visible: bool) {
    query.for_each_mut(|(mut visi, _)| *visi = visible(is_visible));
}

fn back_to_main_menu(
    cmds: &mut Commands,
    set_game_state: &mut NextState<GameState>,
    set_ingame_state: &mut NextState<IngameState>,
    set_wave_state: &mut NextState<WaveState>,
    query: &mut GameScreenQuery,
) {
    for entity in query.iter() {
        cmds.entity(entity).despawn_recursive();
    }
    set_wave_state.set(WaveState::None);
    set_ingame_state.set(IngameState::None);
    set_game_state.set(GameState::Menu);
}
