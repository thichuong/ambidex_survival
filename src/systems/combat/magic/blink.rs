use bevy::prelude::*;

pub fn perform_blink(player_transform: &mut Transform, cursor_pos: Vec2) {
    player_transform.translation = cursor_pos.extend(0.0);
}
