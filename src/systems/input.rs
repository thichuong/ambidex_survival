use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::resources::input_settings::{InputSettings, VirtualInput};
use crate::components::player::GameCamera;

#[allow(clippy::needless_pass_by_value)]
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

    // Only set from keyboard if touch is not overriding it
    if !virtual_input.is_active || axis != Vec2::ZERO {
        virtual_input.axis = axis;
    }

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

#[allow(clippy::needless_pass_by_value)]
pub fn handle_touch_input(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut touches: MessageReader<TouchInput>,
    input_settings: Res<InputSettings>,
    mut virtual_input: ResMut<VirtualInput>,
) {
    if !input_settings.touch_support {
        virtual_input.is_active = false;
        return;
    }

    let half_width = window.width() / 2.0;
    let (camera, camera_transform) = *camera;

    for touch in touches.read() {
        let pos = touch.position;

        if pos.x < half_width {
            // LEFT SIDE: Movement (Joystick)
            match touch.phase {
                bevy::input::touch::TouchPhase::Started => {
                    virtual_input.joystick_start = Some(pos);
                    virtual_input.is_active = true;
                }
                bevy::input::touch::TouchPhase::Moved => {
                    if let Some(start) = virtual_input.joystick_start {
                        let diff = pos - start;
                        if diff.length_squared() > 100.0 {
                            // Deadzone
                            virtual_input.axis = diff.normalize();
                        } else {
                            virtual_input.axis = Vec2::ZERO;
                        }
                    }
                }
                bevy::input::touch::TouchPhase::Ended | bevy::input::touch::TouchPhase::Canceled => {
                    virtual_input.joystick_start = None;
                    virtual_input.axis = Vec2::ZERO;
                    virtual_input.is_active = false;
                }
            }
        } else {
            // RIGHT SIDE: Aiming (Direct)
            match touch.phase {
                bevy::input::touch::TouchPhase::Started | bevy::input::touch::TouchPhase::Moved => {
                    if let Ok(ray) = camera.viewport_to_world(camera_transform, pos) {
                        virtual_input.cursor_world = ray.origin.truncate();
                    }
                }
                _ => {}
            }
        }
    }
}
