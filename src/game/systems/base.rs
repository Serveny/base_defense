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
    queries.p0().single_mut().sections[0].value = format!("{}", game.energy);
    queries.p1().single_mut().sections[0].value = format!("{}", game.materials);
}
