use crate::{
    game::Game,
    utils::{energy::EnergyText, materials::MaterialsText},
};
use bevy::prelude::*;
type QueryEnergyText<'w, 's, 'a> = Query<'w, 's, &'a mut Text, With<EnergyText>>;
type QueryMaterialsText<'w, 's, 'a> = Query<'w, 's, &'a mut Text, With<MaterialsText>>;

pub(super) fn base_system(
    game: Res<Game>,
    mut queries: ParamSet<(QueryEnergyText, QueryMaterialsText)>,
) {
    if let Ok(mut text) = queries.p0().single_mut() {
        text.0 = format!("{}", game.energy)
    };
    if let Ok(mut text) = queries.p1().single_mut() {
        text.0 = format!("{}", game.materials)
    };
}
