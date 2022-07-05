use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
pub struct HealthBarPercentage;

pub fn health_bar(parent: &mut ChildBuilder, bar_width_px: f32) {
    parent
        .spawn_bundle(health_bar_background_shape(
            bar_width_px,
            Vec3::new(0., 0., 0.1),
        ))
        .insert(HealthBar);
    parent
        .spawn_bundle(health_bar_percentage_shape(bar_width_px))
        .insert(HealthBar)
        .insert(HealthBarPercentage);
}

fn health_bar_background_shape(bar_width: f32, translation: Vec3) -> ShapeBundle {
    let shape = shapes::Rectangle {
        origin: RectangleOrigin::Center,
        extents: Vec2::new(bar_width, bar_width / 4.),
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

fn health_bar_percentage_shape(bar_width: f32) -> ShapeBundle {
    let margin = 0.01;
    let shape = shapes::Rectangle {
        origin: RectangleOrigin::BottomLeft,
        extents: Vec2::new(bar_width - margin, bar_width / 4. - margin),
    };

    GeometryBuilder::build_as(
        &shape,
        DrawMode::Fill(FillMode::color(Color::GREEN)),
        Transform {
            translation: Vec3::new(-bar_width / 2. + margin, -bar_width / 8. + margin, 0.2),
            ..Default::default()
        },
    )
}
