use crate::components::enemy::Enemy;
use crate::components::physics::UniformGrid;
use crate::components::player::{CombatStats, GameCamera, Player, Progression};
use crate::components::weapon::{Lifetime, Projectile};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod collision;
pub mod elite_ai;
pub mod events;
pub mod gun;
pub mod magic;
pub mod player_collision;
pub mod shuriken;
pub mod sword;
pub mod sword_mechanics;
pub mod yellow_ai;

pub use collision::*;
pub use events::*;
pub use gun::*;
pub use magic::*;
pub use player_collision::*;
pub use shuriken::*;
pub use sword::*;
pub use sword_mechanics::*;

#[derive(Event, Debug, Clone, Copy)]
pub struct DamageEvent {
    pub entity: Entity,
    pub damage: f32,
    pub crit: bool,
}

/// Marker component for projectiles that have hit a target and should be despawned.
/// Used to prevent double-damage by marking projectiles immediately on first hit.
#[derive(Component)]
pub struct PendingDespawn;

#[derive(SystemParam)]
pub struct CombatInputParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub time: Res<'w, Time>,
    pub mouse_input: Res<'w, ButtonInput<MouseButton>>,
    pub key_input: Res<'w, ButtonInput<KeyCode>>,
    pub window: Single<'w, 's, &'static Window, With<PrimaryWindow>>,
    pub camera: Single<'w, 's, (&'static Camera, &'static GlobalTransform), With<GameCamera>>,
    pub cached_assets: Res<'w, crate::resources::cached_assets::CachedAssets>,
    pub projectile_query: Query<
        'w,
        's,
        (
            Entity,
            &'static GlobalTransform,
            &'static Projectile,
            &'static Lifetime,
        ),
        Without<Player>,
    >,
}

pub struct CombatContext<'a> {
    pub owner_entity: Entity,
    pub transform: &'a mut Transform,
    pub cursor_pos: Vec2,
    pub spawn_pos: Vec2,
    pub damage_multiplier: f32,
    pub combat_stats: &'a CombatStats,
    pub progression: &'a Progression,
}

/// Update enemy grid for spatial partitioning - rebuilds grid each frame
#[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
pub fn update_enemy_grid(
    mut grid: ResMut<UniformGrid>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    grid.clear();
    for (entity, transform) in &enemy_query {
        grid.insert(entity, transform.translation.truncate());
    }
}

#[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
pub fn manage_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_manage_lifetime() {
        let mut app = App::new();

        let entity = app
            .world_mut()
            .spawn(Lifetime {
                timer: Timer::new(Duration::from_millis(100), TimerMode::Once),
            })
            .id();

        app.init_resource::<Time>();
        app.add_systems(Update, manage_lifetime);

        // One frame pass (50ms)
        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(Duration::from_millis(50));
        }
        app.update();

        // Entity should still exist. In Bevy 0.17, world.get_entity returns Result.
        assert!(app.world().get_entity(entity).is_ok());

        // Another frame pass (100ms total)
        let mut time = app.world_mut().resource_mut::<Time>();
        time.advance_by(Duration::from_millis(100));

        app.update();

        // Entity should be despawned
        assert!(app.world().get_entity(entity).is_err());
    }
}
