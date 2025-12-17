use bevy::prelude::*;
use rand::Rng;

#[derive(Resource, Default)]
pub struct ScreenShake {
    pub trauma: f32, // 0.0 to 1.0
}

impl ScreenShake {
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).min(1.0);
    }
}

pub fn update_camera_shake(
    mut camera_query: Query<&mut Transform, With<crate::components::player::GameCamera>>,
    mut shake: ResMut<ScreenShake>,
    time: Res<Time>,
    player_query: Query<
        &Transform,
        (
            With<crate::components::player::Player>,
            Without<crate::components::player::GameCamera>,
        ),
    >,
) {
    // Decay trauma
    shake.trauma = (shake.trauma - time.delta_seconds()).max(0.0);

    let shake_amount = shake.trauma.powi(2) * 20.0; // Max 20px shake

    let mut rng = rand::thread_rng();
    let offset_x = rng.gen_range(-1.0..1.0) * shake_amount;
    let offset_y = rng.gen_range(-1.0..1.0) * shake_amount;

    // Camera follows player + Shake
    if let Ok(player_tf) = player_query.get_single() {
        for mut cam_tf in camera_query.iter_mut() {
            cam_tf.translation.x = player_tf.translation.x + offset_x;
            cam_tf.translation.y = player_tf.translation.y + offset_y;
            // Keep Z
        }
    }
}
