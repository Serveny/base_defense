use crate::board::visualisation::TILE_SIZE;
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::SvgPathShape};

pub const ENERGY_COLOR: Color = Color::YELLOW;

#[derive(Component)]
pub struct EnergyText;

pub fn energy_symbol(transform: Transform, color: Color) -> impl Bundle {
    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&SvgPathShape {
                svg_doc_size_in_px: Vec2::new(512., 512.),
                svg_path_string: String::from(
                    "m412.324,209.102c-5.547-10.516-16.438-17.102-28.328-17.102h-60.219l72.844-145.688c4.953-9.922 4.422-21.703-1.406-31.133-5.829-9.437-16.125-15.179-27.219-15.179h-160c-13.781,0-26,8.813-30.359,21.883l-80,240c-3.25,9.758-1.609,20.484 4.406,28.828 6.016,8.344 15.672,13.289 25.953,13.289h74.703l-26.328,171.133c-2.266,14.75 5.953,29.117 19.828,34.617 3.844,1.523 7.844,2.25 11.781,2.25 10.297,0 20.266-4.977 26.391-13.867l176-256c6.734-9.797 7.484-22.516 1.953-33.031z",
                ),
            }),
            transform,
            ..default()
        },
        Stroke::new(color, TILE_SIZE / 15.),
    )
}

//pub fn energy_symbol_mesh() -> Mesh {
//let mut symbol = Mesh::new(PrimitiveTopology::TriangleList);

//// Positions
//let v_pos = vec![
//[-1.16399, -0.181450, -0.000000],
//[0.017469, 0.253399, 0.000000],
//[-1.104305, 1.81855, 0.000000],
//[0.605796, 2.20224, 0.000000],
//[-0.352365, 0.563218, 0.000000],
//[1.21626, 1.140569, 0.000000],
//[-0.696923, -2.142407, -0.000000],
//];
//symbol.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

//// Color
//let mut v_color = Vec::new();
//v_color.extend_from_slice(&[Color::YELLOW.as_linear_rgba_u32(); 7]);

//// Indices
//symbol.insert_attribute(
//MeshVertexAttribute::new("Vertex_Color", 1, VertexFormat::Uint32),
//v_color,
//);
//let indices = vec![0, 3, 2, 5, 6, 4, 0, 1, 3];
//symbol.set_indices(Some(Indices::U32(indices)));

//symbol
//}
