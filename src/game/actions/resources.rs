use crate::{
    game::{Game, GameScreen},
    utils::{
        bold_text_bundle, energy::energy_symbol, materials::materials_symbol, Energy, IngameTime,
        IngameTimestamp, Materials, Vec2Board,
    },
};
use bevy::color::palettes::css::{GREEN, RED};
use bevy::prelude::*;
use std::time::Duration;

pub const RESOURCE_ANIMATION_TIME: Duration = Duration::from_secs(2);

#[derive(Message, Debug)]
pub enum ResourcesMessage {
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
    mut events: MessageReader<ResourcesMessage>,
    mut game: ResMut<Game>,
    assets: Res<AssetServer>,
    time: Res<IngameTime>,
) {
    for ev in events.read() {
        match ev {
            ResourcesMessage::Energy(energy, pos) => {
                game.energy += energy;
                spawn_energy_animation(&mut cmds, *energy, *pos, &assets, time.now())
            }
            ResourcesMessage::Materials(materials, pos) => {
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
    let mut text_pos = pos;
    text_pos.x += 0.42;
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
        text_pos.to_scaled_vec3(6.1),
    ))
    .insert(ResourceAnimation::new(now + RESOURCE_ANIMATION_TIME))
    .insert(GameScreen)
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
    let mut text_pos = pos;
    text_pos.x += 0.42;
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
        text_pos.to_scaled_vec3(6.1),
    ))
    .insert(ResourceAnimation::new(now + RESOURCE_ANIMATION_TIME))
    .insert(GameScreen)
    .insert(ResourceTextFade);
}

fn resource_text(
    number: f32,
    color: Color,
    assets: &AssetServer,
    translation: Vec3,
) -> impl Bundle {
    bold_text_bundle(
        &format!("{number}"),
        color,
        assets,
        translation,
        crate::board::visualisation::TILE_SIZE / 3.5,
    )
}

pub fn consume(
    res_actions: &mut MessageWriter<ResourcesMessage>,
    resources: (Energy, Materials),
    pos: Vec2Board,
) {
    res_actions.write(ResourcesMessage::Energy(resources.0, pos));
    res_actions.write(ResourcesMessage::Materials(resources.1, pos));
}
