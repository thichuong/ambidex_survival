use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct InputSettings {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,

    pub left_fire: ActionInput,
    pub right_fire: ActionInput,
    pub left_skill: ActionInput,
    pub right_skill: ActionInput,
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

            left_fire: ActionInput::Mouse(MouseButton::Left),
            right_fire: ActionInput::Mouse(MouseButton::Right),
            left_skill: ActionInput::Keyboard(KeyCode::KeyQ),
            right_skill: ActionInput::Keyboard(KeyCode::KeyE),
        }
    }
}

impl ActionInput {
    pub fn is_pressed(
        &self,
        input: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        match self {
            ActionInput::Keyboard(key) => input.pressed(*key),
            ActionInput::Mouse(button) => mouse.pressed(*button),
        }
    }

    pub fn is_just_pressed(
        &self,
        input: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        match self {
            ActionInput::Keyboard(key) => input.just_pressed(*key),
            ActionInput::Mouse(button) => mouse.just_pressed(*button),
        }
    }
}
