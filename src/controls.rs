use bevy::prelude::{KeyCode, MouseButton};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) enum ControlAction {
    ActivateOverview,
    DeactivateOverview,
    SpeedDown,
    SpeedUp,
    FastForward,
    NormalSpeed,
    Pause,
    BuildMenuPrevious,
    BuildMenuNext,
    BuildSelected,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct KeyBinding {
    pub(crate) key_code: KeyCode,
    pub(crate) label: &'static str,
    pub(crate) on_press: Option<ControlAction>,
    pub(crate) on_release: Option<ControlAction>,
}

pub(crate) static KEY_BINDINGS: [KeyBinding; 9] = [
    KeyBinding {
        key_code: KeyCode::Escape,
        label: "Pause / continue",
        on_press: None,
        on_release: Some(ControlAction::Pause),
    },
    KeyBinding {
        key_code: KeyCode::ShiftLeft,
        label: "Show overview",
        on_press: Some(ControlAction::ActivateOverview),
        on_release: Some(ControlAction::DeactivateOverview),
    },
    KeyBinding {
        key_code: KeyCode::Comma,
        label: "Decrease game speed",
        on_press: Some(ControlAction::SpeedDown),
        on_release: None,
    },
    KeyBinding {
        key_code: KeyCode::Period,
        label: "Increase game speed",
        on_press: Some(ControlAction::SpeedUp),
        on_release: None,
    },
    KeyBinding {
        key_code: KeyCode::KeyF,
        label: "Hold 4x game speed",
        on_press: Some(ControlAction::FastForward),
        on_release: Some(ControlAction::NormalSpeed),
    },
    KeyBinding {
        key_code: KeyCode::KeyP,
        label: "Pause / continue",
        on_press: None,
        on_release: Some(ControlAction::Pause),
    },
    KeyBinding {
        key_code: KeyCode::ArrowUp,
        label: "Previous tower/building",
        on_press: None,
        on_release: Some(ControlAction::BuildMenuPrevious),
    },
    KeyBinding {
        key_code: KeyCode::ArrowDown,
        label: "Next tower/building",
        on_press: None,
        on_release: Some(ControlAction::BuildMenuNext),
    },
    KeyBinding {
        key_code: KeyCode::Enter,
        label: "Place selected tower/building",
        on_press: None,
        on_release: Some(ControlAction::BuildSelected),
    },
];

pub(crate) const BUILD_MENU_MOUSE_BUTTON: MouseButton = MouseButton::Left;
pub(crate) const CLOSE_BUILD_MENU_MOUSE_BUTTON: MouseButton = MouseButton::Right;

#[derive(Clone, Copy, Debug)]
pub(crate) struct MouseBinding {
    pub(crate) input: &'static str,
    pub(crate) description: &'static str,
}

pub(crate) static MOUSE_BINDINGS: [MouseBinding; 4] = [
    MouseBinding {
        input: "left click",
        description: "Open the tower/building menu on a buildable tile",
    },
    MouseBinding {
        input: "second left click",
        description: "Place the selected tower or building",
    },
    MouseBinding {
        input: "mouse wheel",
        description: "Scroll through the open tower/building menu",
    },
    MouseBinding {
        input: "right click",
        description: "Close the tower/building menu",
    },
];

pub(crate) const fn key_label(key_code: KeyCode) -> &'static str {
    match key_code {
        KeyCode::Escape => "Esc",
        KeyCode::ShiftLeft => "Left Shift",
        KeyCode::Comma => ",",
        KeyCode::Period => ".",
        KeyCode::KeyF => "F",
        KeyCode::KeyP => "P",
        KeyCode::ArrowUp => "Arrow Up",
        KeyCode::ArrowDown => "Arrow Down",
        KeyCode::Enter => "Enter",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_bindings_have_at_least_one_action() {
        for binding in KEY_BINDINGS.iter() {
            assert!(binding.on_press.is_some() || binding.on_release.is_some());
        }
    }
}
