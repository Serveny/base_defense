use super::{
    tower_base_shape, tower_circle_shape, tower_range_circle_shape, Tower, TowerCannon,
    TowerRangeCircle, TowerValues,
};
use crate::{board::visualisation::TILE_SIZE, utils::Vec2Board};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

impl Tower {
    pub fn laser(pos: Vec2Board) -> Self {
        Self::Laser(TowerValues::laser(pos))
    }
}

pub(super) fn spawn_laser_tower<TScreen: Component + Default>(
    cmds: &mut Commands,
    vals: TowerValues,
) {
    let color = Color::RED;
    let bundle = cmds
        .spawn_bundle(tower_base_shape(vals.pos.to_scaled_vec3(1.), color))
        .with_children(|parent| laser_tower_children(parent, &vals, color))
        .insert(Tower::Laser(vals))
        .insert(TScreen::default());
}

fn laser_tower_children(parent: &mut ChildBuilder, vals: &TowerValues, color: Color) {
    // Tower circle
    parent.spawn_bundle(tower_circle_shape());

    // Tower cannon
    parent
        .spawn_bundle(tower_laser_cannon())
        .insert(TowerCannon);

    // Range circle
    let mut range_circle = tower_range_circle_shape(vals.range_radius, color);
    range_circle.visibility.is_visible = false;
    parent
        .spawn_bundle(range_circle)
        .insert(TowerRangeCircle(vals.pos.as_uvec2()));

    // Laser
}

fn tower_laser_cannon() -> ShapeBundle {
    let shape = shapes::Rectangle {
        origin: RectangleOrigin::CustomCenter(Vec2::new(0., TILE_SIZE / 4.)),
        extents: Vec2::new(TILE_SIZE / 6., TILE_SIZE / 2.),
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::SILVER),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, TILE_SIZE / 16.),
        },
        Transform {
            translation: Vec3::new(0., 0., 0.1),
            rotation: Quat::from_rotation_z(0.),
            ..Default::default()
        },
    )
}
