use crate::{
    board::visualisation::TILE_SIZE,
    game::actions::resources::{ResourceAnimation, ResourceSymbolFade, ResourceTextFade},
    utils::IngameTime,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::Stroke;

pub fn resource_animation_system(
    mut cmds: Commands,
    mut q_anims: Query<(Entity, &mut Transform, &ResourceAnimation)>,
    time: Res<IngameTime>,
) {
    let delta = time.delta_secs();
    let now = time.now();
    for (entity, mut transform, anim) in q_anims.iter_mut() {
        if now >= anim.die_time {
            cmds.entity(entity).despawn_recursive();
        } else {
            transform.translation.y += delta * TILE_SIZE / 2.;
        }
    }
}

pub fn resource_text_fade_system(
    mut q_texts: Query<&mut Text, With<ResourceTextFade>>,
    time: Res<IngameTime>,
) {
    let delta = time.delta_secs();
    for mut text in q_texts.iter_mut() {
        if let Some(text) = text.sections.first_mut() {
            let color = &mut text.style.color;
            color.set_alpha(fade(color.alpha(), delta / 4.));
        }
    }
}

pub fn resource_symbol_fade_system(
    mut q_symbols: Query<&mut Stroke, With<ResourceSymbolFade>>,
    time: Res<IngameTime>,
) {
    let delta = time.delta_secs();
    for mut stroke in q_symbols.iter_mut() {
        let a = stroke.color.alpha();
        stroke.color.set_alpha(fade(a, delta / 4.));
    }
}

fn fade(old_val: f32, delta: f32) -> f32 {
    old_val - (delta / old_val)
}
