use crate::{
    board::visualisation::TILE_SIZE,
    game::{Game, GameScreen},
    utils::{
        energy::energy_symbol, materials::materials_symbol, text_bundle, Energy, IngameTime,
        IngameTimestamp, Materials, Vec2Board,
    },
};
use bevy::color::palettes::css::{GREEN, RED};
use bevy::prelude::*;
use std::time::Duration;

pub const RESOURCE_ANIMATION_TIME: Duration = Duration::from_secs(2);
const WIDTH: f32 = TILE_SIZE / 2.;

#[derive(Event, Debug)]
pub enum ResourcesEvent {
    Energy(Energy, Vec2Board),
    Materials(Materials, Vec2Board),
}

#[derive(Component)]
pub struct ResourceAnimation {
    pub die_time: IngameTimestamp,
}

#[derive(Component)]
pub struct ResourceTextFade;

#[derive(Component)]
pub struct ResourceSymbolFade;

impl ResourceAnimation {
    fn new(die_time: IngameTimestamp) -> Self {
        Self { die_time }
    }
}

pub(super) fn on_change_resources(
    mut cmds: Commands,
    mut events: EventReader<ResourcesEvent>,
    mut game: ResMut<Game>,
    assets: Res<AssetServer>,
    time: Res<IngameTime>,
) {
    for ev in events.read() {
        match ev {
            ResourcesEvent::Energy(energy, pos) => {
                game.energy += energy;
                spawn_energy_animation(&mut cmds, *energy, *pos, &assets, time.now())
            }
            ResourcesEvent::Materials(materials, pos) => {
                game.materials += materials;
                spawn_materials_animation(&mut cmds, *materials, *pos, &assets, time.now());
            }
        }
    }
}

fn color_and_pos(number: f32) -> (Color, f32) {
    if number > 0. {
        (GREEN.into(), 0.1)
    } else {
        (RED.into(), -0.1)
    }
}

fn spawn_energy_animation(
    cmds: &mut Commands,
    energy: Energy,
    mut pos: Vec2Board,
    assets: &AssetServer,
    now: IngameTimestamp,
) {
    let (color, pos_y_add) = color_and_pos(energy);
    pos.x -= 0.4;
    pos.y += pos_y_add;
    cmds.spawn(energy_symbol(
        Transform {
            translation: pos.to_scaled_vec3(6.),
            scale: Vec3::new(0.75, 0.75, 1.),
            ..default()
        },
        color,
    ))
    .insert(ResourceSymbolFade)
    .insert(ResourceAnimation::new(now + RESOURCE_ANIMATION_TIME))
    .insert(GameScreen);
    cmds.spawn(resource_text(
        energy,
        color,
        assets,
        Val::Px(pos.x),
        Val::Px(pos.y),
    ))
    .insert(ResourceTextFade);
}

fn spawn_materials_animation(
    cmds: &mut Commands,
    materials: Materials,
    mut pos: Vec2Board,
    assets: &AssetServer,
    now: IngameTimestamp,
) {
    let (color, pos_y_add) = color_and_pos(materials);
    pos.x += 0.4;
    pos.y += pos_y_add;
    cmds.spawn(materials_symbol(
        Transform {
            translation: pos.to_scaled_vec3(6.),
            scale: Vec3::new(0.75, 0.75, 1.),
            ..default()
        },
        color,
    ))
    .insert(ResourceAnimation::new(now + RESOURCE_ANIMATION_TIME))
    .insert(GameScreen)
    .insert(ResourceSymbolFade);
    cmds.spawn(resource_text(
        materials,
        color,
        assets,
        Val::Px(pos.x),
        Val::Px(pos.y),
    ))
    .insert(ResourceTextFade);
}

fn resource_text(
    number: f32,
    color: Color,
    assets: &AssetServer,
    left: Val,
    bottom: Val,
) -> impl Bundle {
    text_bundle(&format!("{number}"), color, assets, left, bottom)
}

pub fn consume(
    res_actions: &mut EventWriter<ResourcesEvent>,
    resources: (Energy, Materials),
    pos: Vec2Board,
) {
    res_actions.send(ResourcesEvent::Energy(resources.0, pos));
    res_actions.send(ResourcesEvent::Materials(resources.1, pos));
}
