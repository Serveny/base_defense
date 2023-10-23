use crate::board::visualisation::TILE_SIZE;

use super::Vec2Board;
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

const LINE_WIDTH: f32 = TILE_SIZE / 80.;

#[derive(Component)]
struct ResourceBar;

#[derive(Component)]
pub struct ResourceBarPercentage;

pub fn spawn_resource_bar(
    parent: &mut ChildBuilder,
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
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                origin: RectangleOrigin::Center,
                extents: Vec2::new(bar_height / 4., bar_height),
            }),
            transform: Transform::from_translation(translation),
            ..default()
        },
        Fill::color(Color::SILVER),
        Stroke::new(Color::BLACK, LINE_WIDTH),
    )
}

fn resource_bar_percentage_shape(bar_height: f32, translation: Vec3, color: Color) -> impl Bundle {
    let margin = LINE_WIDTH / 2.;
    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                origin: RectangleOrigin::BottomLeft,
                extents: Vec2::new(bar_height / 4. - (margin * 2.), bar_height - (margin * 2.)),
            }),
            transform: Transform::from_translation(Vec3::new(
                translation.x - bar_height / 8. + margin,
                translation.y - bar_height / 2. + margin,
                translation.z,
            )),
            ..default()
        },
        Fill::color(color),
    )
}
