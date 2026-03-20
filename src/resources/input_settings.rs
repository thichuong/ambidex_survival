use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct InputSettings {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,

    pub left_skill: ActionInput,
    pub right_skill: ActionInput,
    pub touch_cursor_sensitivity: f32,
}

#[derive(Resource, Debug, Clone, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct VirtualInput {
    pub axis: Vec2,
    pub cursor_world: Vec2,
    /// Offset from camera position, used to keep cursor fixed on screen
    pub cursor_offset: Vec2,
    pub is_active: bool,
    pub left_skill: bool,
    pub right_skill: bool,
    pub left_skill_clicked: bool,
    pub right_skill_clicked: bool,
    pub joystick_start: Option<Vec2>,
    pub touch_cursor_last_pos: Option<Vec2>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionInput {
    Keyboard(KeyCode),
    Mouse(MouseButton),
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,

            left_skill: ActionInput::Keyboard(KeyCode::KeyQ),
            right_skill: ActionInput::Keyboard(KeyCode::KeyE),
            touch_cursor_sensitivity: 1.5,
        }
    }
}

impl ActionInput {
    pub fn is_just_pressed(
        &self,
        input: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        match self {
            Self::Keyboard(key) => input.just_pressed(*key),
            Self::Mouse(button) => mouse.just_pressed(*button),
        }
    }
}
