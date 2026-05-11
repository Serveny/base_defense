use crate::board::visualisation::TILE_SIZE;

use super::Vec2Board;
use bevy::color::palettes::css::SILVER;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const LINE_WIDTH: f32 = TILE_SIZE / 80.;

#[derive(Component)]
struct ResourceBar;

#[derive(Component)]
pub struct ResourceBarPercentage;

pub fn spawn_resource_bar(
    parent: &mut ChildSpawnerCommands,
    bar_height_px: f32,
    pos: Vec2Board,
    color: Color,
) {
    parent
        .spawn(resource_bar_background_shape(
            bar_height_px,
            pos.to_scaled_vec3(0.1),
        ))
        .insert(ResourceBar);
    parent
        .spawn(resource_bar_percentage_shape(
            bar_height_px,
            pos.to_scaled_vec3(0.2),
            color,
        ))
        .insert(ResourceBar)
        .insert(ResourceBarPercentage);
}

fn resource_bar_background_shape(bar_height: f32, translation: Vec3) -> impl Bundle {
    (
        ShapeBuilder::with(&shapes::Rectangle {
            origin: RectangleOrigin::Center,
            extents: Vec2::new(bar_height / 4., bar_height),
            radii: None,
        })
        .fill(SILVER)
        .stroke(Stroke::new(Color::BLACK, LINE_WIDTH))
        .build(),
        Transform::from_translation(translation),
    )
}

fn resource_bar_percentage_shape(bar_height: f32, translation: Vec3, color: Color) -> impl Bundle {
    let margin = LINE_WIDTH / 2.;
    (
        ShapeBuilder::with(&shapes::Rectangle {
            origin: RectangleOrigin::BottomLeft,
            extents: Vec2::new(bar_height / 4. - (margin * 2.), bar_height - (margin * 2.)),
            radii: None,
        })
        .fill(color)
        .build(),
        Transform::from_translation(Vec3::new(
            translation.x - bar_height / 8. + margin,
            translation.y - bar_height / 2. + margin,
            translation.z,
        )),
    )
}
