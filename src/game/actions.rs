use crate::{
    board::visualisation::BoardHoverCross,
    utils::{towers::TowerRangeCircle, visible, GameState},
};
use bevy::prelude::*;

use self::{
    build_menu::{
        BuildMenuBuildMessage, BuildMenuCloseMessage, BuildMenuHideMessage, BuildMenuOpenMessage,
        BuildMenuScrollMessage,
    },
    collision::{
        on_enemy_collision_add, on_enemy_collision_remove, EnemyCollisionAddMessage,
        EnemyCollisionRemoveMessage,
    },
    damage::{on_damage, DamageMessage},
    explosions::{on_explosions, ExplosionMessage},
    resources::{on_change_resources, ResourcesMessage},
    tile::{on_tile_actions, TileActionsMessage},
    tower::{on_tower_actions, TowerActionsMessage},
    wave::{on_wave_actions, WaveActionsMessage},
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

#[derive(Message)]
pub enum GameActionMessage {
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
        app.add_message::<GameActionMessage>()
            .add_message::<WaveActionsMessage>()
            .add_message::<ResourcesMessage>()
            .add_message::<TileActionsMessage>()
            .add_message::<TowerActionsMessage>()
            .add_message::<DamageMessage>()
            .add_message::<ExplosionMessage>()
            .add_message::<EnemyCollisionAddMessage>()
            .add_message::<EnemyCollisionRemoveMessage>()
            .add_message::<BuildMenuScrollMessage>()
            .add_message::<BuildMenuOpenMessage>()
            .add_message::<BuildMenuCloseMessage>()
            .add_message::<BuildMenuHideMessage>()
            .add_message::<BuildMenuBuildMessage>()
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
    mut game_actions: MessageReader<GameActionMessage>,
    mut q_game_screen: GameScreenQuery,
    mut q_range_circle: RangeCircleQuery,
    mut set_ingame_state: ResMut<NextState<IngameState>>,
) {
    if !game_actions.is_empty() {
        for event in game_actions.read() {
            use GameActionMessage::*;
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
    query
        .iter_mut()
        .for_each(|(mut visi, _)| *visi = visible(is_visible));
}

fn back_to_main_menu(
    cmds: &mut Commands,
    set_game_state: &mut NextState<GameState>,
    set_ingame_state: &mut NextState<IngameState>,
    set_wave_state: &mut NextState<WaveState>,
    query: &mut GameScreenQuery,
) {
    for entity in query.iter() {
        cmds.entity(entity).despawn();
    }
    set_wave_state.set(WaveState::None);
    set_ingame_state.set(IngameState::None);
    set_game_state.set(GameState::Menu);
}
