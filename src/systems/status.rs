use crate::components::physics::Velocity;
use crate::components::status::{StatusEffect, UnitStatus};
use bevy::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn tick_status_system(
    time: Res<Time>,
    mut query: Query<(&mut UnitStatus, Option<&mut Velocity>)>,
) {
    for (mut status, mut velocity) in &mut query {
        let mut forced_velocity = Vec2::ZERO;
        let mut has_forced_movement = false;

        status.effects.retain_mut(|effect| match effect {
            StatusEffect::Rooted { timer } => {
                timer.tick(time.delta());
                !timer.is_finished()
            }
            StatusEffect::ForcedMovement {
                timer,
                direction,
                speed,
                move_type: _,
            } => {
                timer.tick(time.delta());
                if timer.is_finished() {
                    false
                } else {
                    forced_velocity = *direction * *speed;
                    has_forced_movement = true;
                    true
                }
            }
        });

        if let Some(vel) = &mut velocity
            && has_forced_movement
        {
            vel.linvel = forced_velocity;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_rooted_status_expiry() {
        let mut app = App::new();
        app.add_systems(Update, tick_status_system);
        app.init_resource::<Time>();

        let entity = app.world_mut().spawn(UnitStatus::default()).id();

        // Root for 1 second
        {
            let mut status = app.world_mut().get_mut::<UnitStatus>(entity).unwrap();
            status.root(1.0);
            assert!(status.is_rooted());
        }

        // Tick 0.5s - still rooted
        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(Duration::from_millis(500));
        }
        app.update();

        {
            let status = app.world().get::<UnitStatus>(entity).unwrap();
            assert!(status.is_rooted());
        }

        // Tick another 0.6s - rooted expired
        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(Duration::from_millis(600));
        }
        app.update();

        {
            let status = app.world().get::<UnitStatus>(entity).unwrap();
            assert!(!status.is_rooted());
        }
    }

    #[test]
    fn test_forced_movement() {
        let mut app = App::new();
        app.add_systems(Update, tick_status_system);
        app.init_resource::<Time>();

        let entity = app
            .world_mut()
            .spawn((UnitStatus::default(), Velocity::default()))
            .id();

        let direction = Vec2::new(1.0, 0.0);
        let speed = 100.0;

        // Apply ForcedMovement
        {
            let mut status = app.world_mut().get_mut::<UnitStatus>(entity).unwrap();
            status.add(StatusEffect::ForcedMovement {
                timer: Timer::from_seconds(1.0, TimerMode::Once),
                direction,
                speed,
                move_type: crate::components::status::ForceType::Push,
            });
        }

        // Tick 0.1s
        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(Duration::from_millis(100));
        }
        app.update();

        // Check Velocity
        {
            let velocity = app.world().get::<Velocity>(entity).unwrap();
            assert_eq!(velocity.linvel, direction * speed);
        }

        // Tick past 1.0s
        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(Duration::from_secs(1));
        }
        app.update();

        // Check effect removed
        {
            let status = app.world().get::<UnitStatus>(entity).unwrap();
            assert!(status.effects.is_empty());
        }
    }
}
