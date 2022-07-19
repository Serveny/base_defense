use crate::board::visualisation::TILE_SIZE;

use super::Vec2Board;
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

const LINE_WIDTH: f32 = TILE_SIZE / 80.;

#[derive(Component)]
struct ResourceBar;

#[derive(Component)]
pub struct ResourceBarPercentage;

pub fn spawn_resource_bar<TScreen: Component + Default>(
    parent: &mut ChildBuilder,
    bar_height_px: f32,
    pos: Vec2Board,
    color: Color,
) {
    parent
        .spawn_bundle(resource_bar_background_shape(
            bar_height_px,
            pos.to_scaled_vec3(3.),
        ))
        .insert(TScreen::default())
        .insert(ResourceBar);
    parent
        .spawn_bundle(resource_bar_percentage_shape(
            bar_height_px,
            pos.to_scaled_vec3(10.),
            color,
        ))
        .insert(TScreen::default())
        .insert(ResourceBar)
        .insert(ResourceBarPercentage);
}

fn resource_bar_background_shape(bar_height: f32, translation: Vec3) -> ShapeBundle {
    let shape = shapes::Rectangle {
        origin: RectangleOrigin::Center,
        extents: Vec2::new(bar_height / 4., bar_height),
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::SILVER),
            outline_mode: StrokeMode::new(Color::BLACK, LINE_WIDTH),
        },
        Transform {
            translation,
            ..Default::default()
        },
    )
}

fn resource_bar_percentage_shape(bar_height: f32, translation: Vec3, color: Color) -> ShapeBundle {
    let margin = LINE_WIDTH / 2.;
    let shape = shapes::Rectangle {
        origin: RectangleOrigin::BottomLeft,
        extents: Vec2::new(bar_height / 4. - (margin * 2.), bar_height - (margin * 2.)),
    };

    GeometryBuilder::build_as(
        &shape,
        DrawMode::Fill(FillMode::color(color)),
        Transform {
            translation: Vec3::new(
                translation.x - bar_height / 8. + margin,
                translation.y - bar_height / 2. + margin,
                translation.z,
            ),
            ..Default::default()
        },
    )
}
