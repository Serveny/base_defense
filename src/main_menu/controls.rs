use bevy::{app::AppExit, prelude::*};

pub(super) fn keyboard_input(mut app_exit_events: EventWriter<AppExit>, keys: Res<Input<KeyCode>>) {
    if keys.just_released(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}
