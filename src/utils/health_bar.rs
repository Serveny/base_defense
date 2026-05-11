use bevy::color::palettes::css::{GREEN, SILVER};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
pub struct HealthBarPercentage;

pub fn health_bar(parent: &mut ChildSpawnerCommands, bar_width_px: f32) {
    parent
        .spawn(health_bar_background_shape(
            bar_width_px,
            Vec3::new(0., 0., 0.1),
        ))
        .insert(HealthBar);
    parent
        .spawn(health_bar_percentage_shape(bar_width_px))
        .insert(HealthBar)
        .insert(HealthBarPercentage);
}

fn health_bar_background_shape(bar_width: f32, translation: Vec3) -> impl Bundle {
    (
        ShapeBuilder::with(&shapes::Rectangle {
            origin: RectangleOrigin::Center,
            extents: Vec2::new(bar_width, bar_width / 4.),
            radii: None,
        })
        .fill(SILVER)
        .stroke(Stroke::new(Color::BLACK, bar_width / 16.))
        .build(),
        Transform::from_translation(translation),
    )
}

fn health_bar_percentage_shape(bar_width: f32) -> impl Bundle {
    let margin = 0.01;
    (
        ShapeBuilder::with(&shapes::Rectangle {
            origin: RectangleOrigin::BottomLeft,
            extents: Vec2::new(bar_width - margin, bar_width / 4. - margin),
            radii: None,
        })
        .fill(GREEN)
        .build(),
        Transform::from_translation(Vec3::new(
            -bar_width / 2. + margin,
            -bar_width / 8. + margin,
            0.2,
        )),
    )
}
