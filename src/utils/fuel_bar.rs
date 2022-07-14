use super::materials::MATERIALS_COLOR;
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

#[derive(Component)]
struct FuelBar;

#[derive(Component)]
pub struct FuelBarPercentage;

pub fn fuel_bar(parent: &mut ChildBuilder, bar_height_px: f32) {
    parent
        .spawn_bundle(fuel_bar_background_shape(
            bar_height_px,
            Vec3::new(0., 0., 0.1),
        ))
        .insert(FuelBar);
    parent
        .spawn_bundle(fuel_bar_percentage_shape(bar_height_px))
        .insert(FuelBar)
        .insert(FuelBarPercentage);
}

fn fuel_bar_background_shape(bar_height: f32, translation: Vec3) -> ShapeBundle {
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

fn fuel_bar_percentage_shape(bar_height: f32) -> ShapeBundle {
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
