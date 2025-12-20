#![allow(clippy::cast_precision_loss)]
//! Visual effects spawning for weapons and spells
//! Contains functions to spawn child entities with visual meshes for attack animations

use bevy::prelude::*;
use rand::Rng;

use crate::configs::spells::{global, laser, nova};
use crate::configs::weapons::sword;

/// Spawn visual effects for Energy Bolt spell
pub fn spawn_energy_bolt_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    // Outer aura - large, transparent purple
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_purple_20.clone()),
        Transform::from_xyz(0.0, 0.0, -0.3).with_scale(Vec3::splat(16.0)),
    ));
    // Mid glow - medium magenta
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_purple_40.clone()),
        Transform::from_xyz(0.0, 0.0, -0.2).with_scale(Vec3::splat(11.0)),
    ));
    // Core - bright purple
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_purple_80.clone()),
        Transform::from_xyz(0.0, 0.0, -0.1).with_scale(Vec3::splat(8.0)),
    ));
    // Inner core - white center
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_white.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(4.0)),
    ));
    // Orbiting particles
    for i in 0..4 {
        let angle = (i as f32) * std::f32::consts::FRAC_PI_2;
        let x = angle.cos() * 10.0;
        let y = angle.sin() * 10.0;
        parent.spawn((
            Mesh2d(cached.unit_circle.clone()),
            MeshMaterial2d(cached.mat_magenta_70.clone()),
            Transform::from_xyz(x, y, 0.1).with_scale(Vec3::splat(2.0)),
        ));
    }
}

/// Spawn visual effects for Energy Bolt explosion
pub fn spawn_bolt_explosion_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
    radius: f32,
) {
    let mut rng = rand::thread_rng();

    // Outer shockwave - large, fading purple
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_purple_20.clone()),
        Transform::from_xyz(0.0, 0.0, -0.3).with_scale(Vec3::splat(radius)),
    ));
    // Mid ring - orange/yellow
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_orange_25.clone()),
        Transform::from_xyz(0.0, 0.0, -0.2).with_scale(Vec3::splat(radius * 0.75)),
    ));
    // Inner blast - bright white/yellow
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_orange_60.clone()),
        Transform::from_xyz(0.0, 0.0, -0.1).with_scale(Vec3::splat(radius * 0.5)),
    ));
    // Core flash - brightest white
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_white_90.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(radius * 0.2)),
    ));
    // Explosion particles radiating outward
    for i in 0..12 {
        let angle = (i as f32) * std::f32::consts::PI / 6.0;
        let dist = radius * rng.gen_range(0.4..0.9);
        let x = angle.cos() * dist;
        let y = angle.sin() * dist;
        let size = rng.gen_range(3.0..6.0);

        // Color varies from orange to yellow
        let color_blend: f32 = rng.gen_range(0.0..1.0);
        let material = if color_blend > 0.5 {
            cached.mat_orange_60.clone()
        } else {
            cached.mat_orange_25.clone()
        };
        parent.spawn((
            Mesh2d(cached.unit_circle.clone()),
            MeshMaterial2d(material),
            Transform::from_xyz(x, y, 0.1).with_scale(Vec3::splat(size)),
        ));
    }
}

/// Spawn visual effects for Laser spell
pub fn spawn_laser_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    // Outer glow - wide, transparent cyan
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_cyan_30.clone()),
        Transform::from_xyz(laser::LENGTH / 2.0, 0.0, -0.3).with_scale(Vec3::new(
            laser::LENGTH,
            laser::WIDTH * 2.5,
            1.0,
        )),
    ));
    // Mid beam - brighter
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_cyan_50.clone()),
        Transform::from_xyz(laser::LENGTH / 2.0, 0.0, -0.2).with_scale(Vec3::new(
            laser::LENGTH,
            laser::WIDTH * 1.5,
            1.0,
        )),
    ));
    // Core beam - main laser
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_cyan_70.clone()),
        Transform::from_xyz(laser::LENGTH / 2.0, 0.0, -0.1).with_scale(Vec3::new(
            laser::LENGTH,
            laser::WIDTH,
            1.0,
        )),
    ));
    // Inner core - brightest white
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_white_90.clone()),
        Transform::from_xyz(laser::LENGTH / 2.0, 0.0, 0.0).with_scale(Vec3::new(
            laser::LENGTH,
            laser::WIDTH * 0.4,
            1.0,
        )),
    ));
    // Electric sparks along the beam
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let dist = rng.gen_range(20.0..laser::LENGTH - 20.0);
        let jitter_y = rng.gen_range(-laser::WIDTH * 0.6..laser::WIDTH * 0.6);
        let size = rng.gen_range(2.0..4.0);
        parent.spawn((
            Mesh2d(cached.unit_circle.clone()),
            MeshMaterial2d(cached.mat_white_90.clone()),
            Transform::from_xyz(dist, jitter_y, 0.1).with_scale(Vec3::splat(size)),
        ));
    }
    // Impact glow at end
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_cyan_50.clone()),
        Transform::from_xyz(laser::LENGTH, 0.0, -0.15).with_scale(Vec3::splat(laser::WIDTH)),
    ));
}

/// Spawn visual effects for Nova spell
pub fn spawn_nova_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    // Outer pulse - large, fading
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_purple_20.clone()),
        Transform::from_xyz(0.0, 0.0, -0.3).with_scale(Vec3::splat(nova::RADIUS)),
    ));
    // Mid ring - medium magenta
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_magenta_40.clone()),
        Transform::from_xyz(0.0, 0.0, -0.2).with_scale(Vec3::splat(nova::RADIUS * 0.75)),
    ));
    // Inner ring - brighter
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_magenta_40.clone()),
        Transform::from_xyz(0.0, 0.0, -0.1).with_scale(Vec3::splat(nova::RADIUS * 0.5)),
    ));
    // Center glow - brightest white/pink
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_magenta_70.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(nova::RADIUS * 0.2)),
    ));
    // Particle bursts around the edge
    let mut rng = rand::thread_rng();
    for i in 0..12 {
        let angle = (i as f32) * std::f32::consts::PI / 6.0;
        let radius = nova::RADIUS * rng.gen_range(0.6..0.95);
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;
        let size = rng.gen_range(3.0..6.0);
        parent.spawn((
            Mesh2d(cached.unit_circle.clone()),
            MeshMaterial2d(cached.mat_magenta_40.clone()),
            Transform::from_xyz(x, y, 0.1).with_scale(Vec3::splat(size)),
        ));
    }
}

/// Spawn visual effects for Global spell
pub fn spawn_global_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    // Multiple concentric fading rings - using approx cached materials
    // Note: To fully batch this, we use discrete opacity steps or accept slight visual change
    for i in 0..5 {
        let ring_radius = global::RADIUS * (i as f32).mul_add(0.2, 0.2);
        let material = match i {
            0..=1 => cached.mat_purple_80.clone(),
            2..=3 => cached.mat_purple_40.clone(),
            _ => cached.mat_purple_20.clone(),
        };

        parent.spawn((
            Mesh2d(cached.unit_circle.clone()),
            MeshMaterial2d(material),
            Transform::from_xyz(0.0, 0.0, (i as f32).mul_add(0.05, -0.3))
                .with_scale(Vec3::splat(ring_radius)),
        ));
    }
    // Scattered particles throughout
    let mut rng = rand::thread_rng();
    for _ in 0..24 {
        let angle: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
        let radius = global::RADIUS * rng.gen_range(0.1..0.9);
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;
        let color_variant = rng.gen_range(0..3);
        let material = match color_variant {
            0 => cached.mat_cyan_30.clone(),
            1 => cached.mat_magenta_40.clone(),
            _ => cached.mat_white_90.clone(),
        };
        let size = rng.gen_range(2.0..5.0);
        parent.spawn((
            Mesh2d(cached.unit_circle.clone()),
            MeshMaterial2d(material),
            Transform::from_xyz(x, y, 0.1).with_scale(Vec3::splat(size)),
        ));
    }
}

/// Spawn visual effects for Shuriken projectile
pub fn spawn_shuriken_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    // Outer glow - soft circle
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_cyan_30.clone()),
        Transform::from_xyz(0.0, 0.0, -0.2).with_scale(Vec3::splat(14.0)),
    ));
    // Spin glow - pulsing ring
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_cyan_50.clone()),
        Transform::from_xyz(0.0, 0.0, -0.1).with_scale(Vec3::splat(10.0)),
    ));
    // Core - bright center
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_white.clone()),
        Transform::from_xyz(0.0, 0.0, 0.1).with_scale(Vec3::splat(4.0)),
    ));
    // Shuriken blades - 4 pointed star shape
    for i in 0..4 {
        let angle = (i as f32) * std::f32::consts::FRAC_PI_2;
        let x = angle.cos() * 6.0;
        let y = angle.sin() * 6.0;
        // Blade outer
        parent.spawn((
            Mesh2d(cached.unit_square.clone()),
            MeshMaterial2d(cached.mat_cyan_70.clone()),
            Transform::from_xyz(x, y, 0.0)
                .with_rotation(Quat::from_rotation_z(angle))
                .with_scale(Vec3::new(12.0, 5.0, 1.0)),
        ));
        // Blade inner
        parent.spawn((
            Mesh2d(cached.unit_square.clone()),
            MeshMaterial2d(cached.mat_white_90.clone()),
            Transform::from_xyz(x, y, 0.05)
                .with_rotation(Quat::from_rotation_z(angle))
                .with_scale(Vec3::new(10.0, 3.0, 1.0)),
        ));
    }
}

/// Spawn visual effects for Sword Normal attack - realistic sword shape
pub fn spawn_sword_normal_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    // Blade - main sword body (silver/steel color)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_steel.clone()), // Steel
        Transform::from_xyz(65.0, 0.0, 0.0).with_scale(Vec3::new(120.0, 8.0, 1.0)),
    ));
    // Blade edge highlight (lighter)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_steel_bright.clone()), // Bright edge
        Transform::from_xyz(62.5, 0.0, 0.1).with_scale(Vec3::new(115.0, 3.0, 1.0)),
    ));
    // Sword tip - pointed end (triangle approximation with smaller rect)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_steel.clone()),
        Transform::from_xyz(130.0, 0.0, 0.0).with_scale(Vec3::new(15.0, 6.0, 1.0)),
    ));
    // Guard/crossguard (dark metal)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_bronze.clone()), // Dark bronze
        Transform::from_xyz(5.0, 0.0, 0.2).with_scale(Vec3::new(6.0, 20.0, 1.0)),
    ));
    // Handle/grip (brown leather)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_brown.clone()), // Brown
        Transform::from_xyz(-10.0, 0.0, 0.1).with_scale(Vec3::new(25.0, 6.0, 1.0)),
    ));
    // Pommel (end cap)
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_bronze.clone()), // Dark bronze
        Transform::from_xyz(-22.0, 0.0, 0.2).with_scale(Vec3::splat(5.0)),
    ));
}

/// Spawn visual effects for Sword Shattered attack - broken blade fragments in a line
pub fn spawn_sword_shattered_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    let mut rng = rand::thread_rng();

    // Fragment settings
    let num_fragments = 40; // More fragments
    let total_length = sword::SHATTERED_RANGE;
    let frag_size = 6.0; // Base fragment size
    let blade_width = frag_size * 4.0; // Width fits 4 fragments

    // Blade fragments randomly placed in a line band
    for _ in 0..num_fragments {
        // Random position along the length
        let base_x = rng.gen_range(15.0..total_length);
        // Random Y position within the blade width (4 fragments wide)
        let base_y = rng.gen_range(-blade_width / 2.0..blade_width / 2.0);

        // Random fragment size with variation
        let frag_len = rng.gen_range(8.0..16.0);
        let frag_width = rng.gen_range(4.0..8.0);

        // Random rotation for natural look
        let rotation = rng.gen_range(-0.3..0.3);

        // Steel color with variation
        let variant = rng.gen_range(0..3);
        let material = match variant {
            0 => cached.mat_steel.clone(),
            1 => cached.mat_steel_bright.clone(),
            _ => cached.mat_steel_dark.clone(),
        };

        parent.spawn((
            Mesh2d(cached.unit_square.clone()),
            MeshMaterial2d(material),
            Transform::from_xyz(base_x, base_y, rng.gen_range(-0.1..0.1))
                .with_rotation(Quat::from_rotation_z(rotation))
                .with_scale(Vec3::new(frag_len, frag_width, 1.0)),
        ));
    }

    // Small debris particles
    for _ in 0..15 {
        let dist = rng.gen_range(20.0..total_length);
        let jitter_y = rng.gen_range(-blade_width / 2.0..blade_width / 2.0);
        let size = rng.gen_range(1.5..3.5);

        parent.spawn((
            Mesh2d(cached.unit_circle.clone()),
            MeshMaterial2d(cached.mat_steel_bright.clone()),
            Transform::from_xyz(dist, jitter_y, 0.1).with_scale(Vec3::splat(size)),
        ));
    }
}

/// Spawn visual effects for Gun bullet
pub fn spawn_gun_bullet_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    // Outer trail glow - orange fade
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_orange_25.clone()),
        Transform::from_xyz(-4.0, 0.0, -0.2).with_scale(Vec3::new(28.0, 10.0, 1.0)),
    ));
    // Mid glow - yellow
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_orange_60.clone()),
        Transform::from_xyz(-2.0, 0.0, -0.1).with_scale(Vec3::new(24.0, 7.0, 1.0)),
    ));
    // Core bullet - bright white/yellow
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_yellow_100.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(20.0, 4.0, 1.0)),
    ));
    // Inner core - brightest
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_white.clone()),
        Transform::from_xyz(0.0, 0.0, 0.1).with_scale(Vec3::new(16.0, 2.0, 1.0)),
    ));
    // Tip glow - small circle at front
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_orange_60.clone()),
        Transform::from_xyz(10.0, 0.0, -0.15).with_scale(Vec3::splat(5.0)),
    ));
}
