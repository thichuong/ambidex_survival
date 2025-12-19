use bevy::prelude::*;

use crate::components::physics::Velocity;

/// Apply velocity to transform for all entities with Velocity component
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) -> Result<(), String> {
    let dt = time.delta_secs();
    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.linvel.x * dt;
        transform.translation.y += velocity.linvel.y * dt;
        transform.rotate_z(velocity.angvel * dt);
    }
    Ok(())
}
