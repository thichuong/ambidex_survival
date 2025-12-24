use bevy::prelude::*;
use bevy::window::Window;

/// Reference height for UI design (standard 1080p).
const REFERENCE_HEIGHT: f32 = 1080.0;
/// Minimum scale factor to prevent UI from becoming too small (unreadable).
const MIN_SCALE: f32 = 0.5;
/// Maximum scale factor to prevent UI from becoming comically large.
const MAX_SCALE: f32 = 2.0;

/// Updates the `UiScale` resource based on the primary window's height.
/// This ensures uniform scaling of texts, buttons, and procedurally drawn icons.
pub fn update_ui_scale(
    mut ui_scale: ResMut<UiScale>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    if let Some(window) = window_query.iter().next() {
        let height = window.height();
        // Calculate scale based on height ratio
        let scale = (height / REFERENCE_HEIGHT).clamp(MIN_SCALE, MAX_SCALE);

        // Update UiScale if changed significantly (to avoid float jitter)
        if (ui_scale.0 - scale).abs() > 0.01 {
            ui_scale.0 = scale;
        }
    }
}
