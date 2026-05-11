use bevy::{app::AppExit, prelude::*};

pub(super) fn keyboard_input(
    mut app_exit_events: MessageWriter<AppExit>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_released(KeyCode::Escape) {
        app_exit_events.write(AppExit);
    }
}
