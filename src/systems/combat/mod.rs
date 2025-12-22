use crate::components::enemy::Enemy;
use crate::components::physics::UniformGrid;
use crate::components::player::{GameCamera, Player};
use crate::components::weapon::{Lifetime, Projectile};
use crate::systems::object_pooling::{PooledEffect, VisualEffectPool};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod collision;
pub mod events;
pub mod gun;
pub mod magic;
pub mod player_collision;
pub mod shuriken;
pub mod sword;
pub mod sword_mechanics;

pub use collision::*;
pub use events::*;
pub use gun::*;
pub use magic::*;
pub use player_collision::*;
pub use shuriken::*;
pub use sword::*;
pub use sword_mechanics::*;

#[derive(Event, Message, Debug, Clone, Copy)]
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

#[derive(SystemParam)]
pub struct CombatResources<'w> {
    pub cached_assets: Res<'w, crate::resources::cached_assets::CachedAssets>,
    pub effect_pool: ResMut<'w, VisualEffectPool>,
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
    mut query: Query<(Entity, &mut Lifetime, Option<&PooledEffect>)>,
    mut effect_pool: ResMut<VisualEffectPool>,
) {
    for (entity, mut lifetime, pooled) in &mut query {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.is_finished() {
            if let Some(pooled) = pooled {
                effect_pool.return_to_pool(entity, pooled.kind, &mut commands);
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}
