//! Enemy death handling, loot dropping, and death effects

use crate::components::physics::Velocity;
use crate::components::player::{Currency, Player};
use crate::components::weapon::Lifetime;
use bevy::prelude::*;
use rand::Rng;

/// Handle Despawning of dead enemies, loot, and on-death effects
#[allow(clippy::type_complexity)]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
pub fn enemy_death_system(
    trigger: On<crate::systems::combat::EnemyDeathEvent>,
    mut commands: Commands,
    mut player_query: Query<&mut Currency, With<Player>>,
    elite_query: Query<&crate::components::enemy::EliteEnemy>,
    res: Res<crate::resources::cached_assets::CachedAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let event = trigger.event();

    // Give Gold
    if let Some(mut currency) = player_query.iter_mut().next() {
        let gold_reward = if elite_query.get(event.entity).is_ok() {
            crate::configs::enemy::ELITE_GOLD_REWARD
        } else {
            crate::configs::enemy::GOLD_REWARD
        };
        currency.gold += gold_reward;
    }

    commands.entity(event.entity).despawn();

    // Spawn particles
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let dir = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize_or_zero();
        commands.spawn((
            Mesh2d(res.unit_circle.clone()),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
            Transform::from_translation(event.position.extend(0.0)).with_scale(Vec3::splat(3.0)),
            Velocity {
                linvel: dir * 100.0,
                angvel: 0.0,
            },
            Lifetime {
                timer: Timer::from_seconds(0.5, TimerMode::Once),
            },
        ));
    }
}
