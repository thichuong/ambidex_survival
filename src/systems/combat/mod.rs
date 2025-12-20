use crate::components::enemy::Enemy;
use crate::components::physics::UniformGrid;
use crate::components::player::{GameCamera, Player};
use crate::components::weapon::{Lifetime, Projectile};
use crate::systems::object_pooling::{PooledEffect, VisualEffectPool};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod collision;
pub mod player_collision;
pub mod sword_mechanics;
pub mod weapon_logic;

pub use collision::*;
pub use player_collision::*;
pub use sword_mechanics::*;
pub use weapon_logic::*;

#[derive(Event, Message, Debug)]
pub struct DamageEvent {
    pub damage: f32,
    pub position: Vec2,
    pub is_crit: bool,
}

#[derive(SystemParam)]
pub struct CombatInputParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub time: Res<'w, Time>,
    pub mouse_input: Res<'w, ButtonInput<MouseButton>>,
    pub key_input: Res<'w, ButtonInput<KeyCode>>,
    pub window_query: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
    pub camera_query: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<GameCamera>>,
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
pub struct CombatResources<'w, 's> {
    pub shake: ResMut<'w, crate::resources::polish::ScreenShake>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<ColorMaterial>>,
    pub cached_assets: Res<'w, crate::resources::cached_assets::CachedAssets>,
    pub exploding_query: Query<'w, 's, &'static crate::components::weapon::ExplodingProjectile>,
    pub effect_pool: ResMut<'w, VisualEffectPool>,
}

/// Update enemy grid for spatial partitioning - rebuilds grid each frame
#[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
pub fn update_enemy_grid(
    mut grid: ResMut<UniformGrid>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) -> Result<(), String> {
    grid.clear();
    for (entity, transform) in &enemy_query {
        grid.insert(entity, transform.translation.truncate());
    }
    Ok(())
}

#[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
pub fn manage_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime, Option<&PooledEffect>)>,
    mut effect_pool: ResMut<VisualEffectPool>,
) -> Result<(), String> {
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
    Ok(())
}
