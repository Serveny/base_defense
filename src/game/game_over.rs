use super::GameScreen;
use crate::assets::StandardAssets;
use bevy::prelude::*;

pub(super) fn setup_game_over_screen(mut cmds: Commands, assets: Res<StandardAssets>) {
    cmds.spawn_bundle(NodeBundle {
        color: UiColor::from(Color::rgba(0., 0., 0., 0.8)),
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            ..Default::default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(
            TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 200.,
                    color: Color::RED,
                },
            )
            .with_text_alignment(TextAlignment::CENTER)
            .with_style(Style {
                align_self: AlignSelf::Center,
                ..default()
            }),
        );
    })
    .insert(GameScreen);
}
