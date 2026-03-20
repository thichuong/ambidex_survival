use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::resources::input_settings::{InputSettings, VirtualInput};
use crate::components::player::GameCamera;

pub fn update_virtual_input(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<GameCamera>>,
    input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    input_settings: Res<InputSettings>,
    mut virtual_input: ResMut<VirtualInput>,
) {
    // 1. Update Navigation Axis (Keyboard)
    let mut axis = Vec2::ZERO;
    if input.pressed(input_settings.move_up) {
        axis.y += 1.0;
    }
    if input.pressed(input_settings.move_down) {
        axis.y -= 1.0;
    }
    if input.pressed(input_settings.move_left) {
        axis.x -= 1.0;
    }
    if input.pressed(input_settings.move_right) {
        axis.x += 1.0;
    }

    if axis != Vec2::ZERO {
        axis = axis.normalize();
    }
    virtual_input.axis = axis;

    // 2. Update Virtual Cursor (Mouse)
    let (camera, camera_transform) = *camera;
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        virtual_input.cursor_world = world_position;
    }

    // 3. Update Skills
    virtual_input.left_skill = input_settings.left_skill.is_just_pressed(&input, &mouse_input);
    virtual_input.right_skill = input_settings.right_skill.is_just_pressed(&input, &mouse_input);
}

pub fn handle_touch_input(
    mut touches: MessageReader<TouchInput>,
    mut virtual_input: ResMut<VirtualInput>,
) {
    // Basic touch implementation - can be expanded with joysticks later
    for touch in touches.read() {
        match touch.phase {
            bevy::input::touch::TouchPhase::Started | bevy::input::touch::TouchPhase::Moved => {
                virtual_input.is_active = true;
                // Note: converting touch to world position would require camera & window
                // For now, we just mark it active.
            }
            bevy::input::touch::TouchPhase::Ended | bevy::input::touch::TouchPhase::Canceled => {
                // If no more touches, we might want to set is_active to false
            }
        }
    }
}
