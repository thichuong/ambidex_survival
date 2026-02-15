use crate::components::status::UnitStatus;
use bevy::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn tick_status_system(time: Res<Time>, mut query: Query<&mut UnitStatus>) {
    for mut status in &mut query {
        if status.is_rooted {
            status.rooted_timer.tick(time.delta());
            if status.rooted_timer.is_finished() {
                status.is_rooted = false;
            }
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

        let entity = app.world_mut().spawn(UnitStatus::default()).id();

        // Root for 1 second
        {
            let mut status = app.world_mut().get_mut::<UnitStatus>(entity).unwrap();
            status.root(1.0);
            assert!(status.is_rooted);
        }

        // Tick 0.5s - still rooted
        app.insert_resource(Time::<Real>::default());
        let mut time = Time::default();
        time.advance_by(Duration::from_millis(500));
        app.insert_resource(time);
        app.update();

        {
            let status = app.world().get::<UnitStatus>(entity).unwrap();
            assert!(status.is_rooted);
        }

        // Tick another 0.6s - rooted expired
        let mut time = app.world_mut().resource_mut::<Time>();
        time.advance_by(Duration::from_millis(600));
        app.update();

        {
            let status = app.world().get::<UnitStatus>(entity).unwrap();
            assert!(!status.is_rooted);
        }
    }
}
