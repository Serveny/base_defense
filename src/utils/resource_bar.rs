use super::materials::MATERIALS_COLOR;
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

#[derive(Component)]
struct ResourceBar;

#[derive(Component)]
pub struct ResourceBarPercentage;

pub fn resource_bar(parent: &mut ChildBuilder, bar_height_px: f32) {
    parent
        .spawn_bundle(resource_bar_background_shape(
            bar_height_px,
            Vec3::new(0., 0., 0.1),
        ))
        .insert(ResourceBar);
    parent
        .spawn_bundle(resource_bar_percentage_shape(bar_height_px))
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
            outline_mode: StrokeMode::new(Color::BLACK, 0.01),
        },
        Transform {
            translation,
            ..Default::default()
        },
    )
}

fn resource_bar_percentage_shape(bar_height: f32) -> ShapeBundle {
    let margin = 0.01;
    let shape = shapes::Rectangle {
        origin: RectangleOrigin::BottomLeft,
        extents: Vec2::new(bar_height / 4. - margin, bar_height - margin),
    };

    GeometryBuilder::build_as(
        &shape,
        DrawMode::Fill(FillMode::color(MATERIALS_COLOR)),
        Transform {
            translation: Vec3::new(-bar_height / 8. + margin, -bar_height / 2. + margin, 0.2),
            ..Default::default()
        },
    )
}
