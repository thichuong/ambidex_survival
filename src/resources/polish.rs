use bevy::prelude::*;
use rand::Rng;

#[derive(Resource, Default)]
pub struct ScreenShake {
    pub trauma: f32, // 0.0 to 1.0
}

impl ScreenShake {}

#[allow(clippy::needless_pass_by_value)]
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
    shake.trauma = (shake.trauma - time.delta_secs()).max(0.0);

    let shake_amount = shake.trauma.powi(2) * 20.0; // Max 20px shake

    let mut rng = rand::thread_rng();
    let offset_x = rng.gen_range(-1.0..1.0) * shake_amount;
    let offset_y = rng.gen_range(-1.0..1.0) * shake_amount;

    // Camera follows player + Shake
    if let Ok(player_tf) = player_query.single() {
        for mut cam_tf in &mut camera_query {
            cam_tf.translation.x = player_tf.translation.x + offset_x;
            cam_tf.translation.y = player_tf.translation.y + offset_y;
            // Keep Z
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn spawn_trails(
    mut commands: Commands,
    _time: Res<Time>,
    projectile_query: Query<(&Transform, &crate::components::weapon::Projectile)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Basic interval check? Or every frame?
    // Every frame might be too much. Let's rely on frame rate.
    // Actually, distinct Trail component on projectile to configure it?
    // For now, let's just make ALL projectiles leave a trail for polish.

    for (transform, proj) in projectile_query.iter() {
        if proj.speed > 100.0 {
            // Only moving projectiles
            commands.spawn((
                (
                    Mesh2d(meshes.add(Circle::new(3.0))), // Small dot
                    MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 1.0, 0.3))), // Faint white
                    Transform::from_translation(transform.translation - transform.local_x() * 10.0), // Behind slightly?
                ),
                crate::components::weapon::Lifetime {
                    timer: Timer::from_seconds(0.3, TimerMode::Once),
                },
            ));
        }
    }
}
