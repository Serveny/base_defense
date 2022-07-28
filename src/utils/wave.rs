use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::SvgPathShape};

use crate::board::visualisation::TILE_SIZE;

#[derive(Component)]
pub struct WaveText;

pub fn wave_symbol(transform: Transform, color: Color) -> ShapeBundle {
    let shape = SvgPathShape {
        svg_doc_size_in_px: Vec2::new(64., 64.),
        svg_path_string: String::from(
            "M63.5478249,30.8939056c-10.0498047-6.6310997-22.3525047-6.6280994-32.1084023,0.0079002
		c-10.0361004,6.8279991-19.8110008,6.8269997-29.8813-0.0009995c-0.4585-0.3111-1.0796-0.1879997-1.3877,0.2747002
		c-0.3086,0.4617004-0.1865,1.0877991,0.2715,1.3987999c5.4116001,3.6679993,10.7426996,5.5029984,16.0708008,5.5029984
		c5.3275986-0.0009995,10.6513996-1.8349991,16.0459003-5.5038986c9.0746956-6.1753006,20.5302963-6.1714001,29.8934975,0.0078011
		c0.4623985,0.3031998,1.0825996,0.1742973,1.3848-0.2914009C64.139122,31.8242073,64.0098267,31.1991062,63.5478249,30.8939056z",
        ),
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Stroke(StrokeMode::new(color, TILE_SIZE / 80.)),
        transform,
    )
}
