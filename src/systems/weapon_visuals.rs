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
    // Main "Arrow" shape indicating direction (pointing +X)
    let segments = [
        // Rear segment
        (Vec3::new(-8.0, 0.0, 0.0), Vec3::new(10.0, 3.0, 1.0), 0.0),
        // Middle zigzag
        (Vec3::new(0.0, 3.0, 0.0), Vec3::new(12.0, 2.5, 1.0), -0.4),
        (Vec3::new(6.0, -3.0, 0.0), Vec3::new(12.0, 2.5, 1.0), 0.4),
        // Front/Tip segment
        (Vec3::new(12.0, 0.0, 0.0), Vec3::new(8.0, 2.0, 1.0), 0.0),
    ];

    for (pos, scale, rot) in &segments {
        // Core (White)
        parent.spawn((
            Mesh2d(cached.unit_square.clone()),
            MeshMaterial2d(cached.mat_bolt_core.clone()),
            Transform::from_translation(*pos)
                .with_rotation(Quat::from_rotation_z(*rot))
                .with_scale(*scale),
        ));
        // Glow (Purple) - larger and behind
        parent.spawn((
            Mesh2d(cached.unit_square.clone()),
            MeshMaterial2d(cached.mat_bolt_glow.clone()),
            Transform::from_translation(*pos + Vec3::new(0.0, 0.0, -0.1))
                .with_rotation(Quat::from_rotation_z(*rot))
                .with_scale(*scale * 2.0),
        ));
    }

    // Impact / Head Glow
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_bolt_glow.clone()),
        Transform::from_xyz(14.0, 0.0, -0.2).with_scale(Vec3::splat(16.0)),
    ));

    // Core Head Brightness
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_white.clone()),
        Transform::from_xyz(14.0, 0.0, 0.1).with_scale(Vec3::splat(6.0)),
    ));

    // Trailing particles (simulating a tail)
    let mut rng = rand::thread_rng();
    for _ in 0..6 {
        let x_off = rng.gen_range(-15.0..-5.0);
        let y_off = rng.gen_range(-5.0..5.0);
        let size = rng.gen_range(2.0..4.0);

        parent.spawn((
            Mesh2d(cached.unit_circle.clone()),
            MeshMaterial2d(cached.mat_purple_40.clone()),
            Transform::from_xyz(x_off, y_off, 0.1).with_scale(Vec3::splat(size)),
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
        MeshMaterial2d(cached.mat_teal_dark.clone()), // Darker teal outer
        Transform::from_xyz(laser::LENGTH / 2.0, 0.0, -0.3).with_scale(Vec3::new(
            laser::LENGTH,
            laser::WIDTH * 3.0,
            1.0,
        )),
    ));
    // Mid beam - brighter cyan
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_teal_light.clone()),
        Transform::from_xyz(laser::LENGTH / 2.0, 0.0, -0.2).with_scale(Vec3::new(
            laser::LENGTH,
            laser::WIDTH * 1.5,
            1.0,
        )),
    ));
    // Core beam - intense white
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_white.clone()),
        Transform::from_xyz(laser::LENGTH / 2.0, 0.0, -0.1).with_scale(Vec3::new(
            laser::LENGTH,
            laser::WIDTH * 0.5,
            1.0,
        )),
    ));

    // Electric sparks/particles along the beam
    let mut rng = rand::thread_rng();
    for _ in 0..15 {
        let dist = rng.gen_range(20.0..laser::LENGTH - 20.0);
        let jitter_y = rng.gen_range(-laser::WIDTH * 0.8..laser::WIDTH * 0.8);
        let size = rng.gen_range(2.0..5.0);
        parent.spawn((
            Mesh2d(cached.unit_circle.clone()),
            MeshMaterial2d(cached.mat_white_90.clone()),
            Transform::from_xyz(dist, jitter_y, 0.1).with_scale(Vec3::splat(size)),
        ));
    }
}

/// Spawn visual effects for Nova spell
pub fn spawn_nova_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    // Rings expanding
    let ring_colors = [
        cached.mat_purple_20.clone(),
        cached.mat_magenta_40.clone(),
        cached.mat_purple_80.clone(),
    ];

    for (i, mat) in ring_colors.iter().enumerate() {
        let scale_factor = (i as f32).mul_add(-0.25, 1.0);
        parent.spawn((
            Mesh2d(cached.unit_circle.clone()),
            MeshMaterial2d(mat.clone()),
            Transform::from_xyz(0.0, 0.0, -0.1 * (i as f32))
                .with_scale(Vec3::splat(nova::RADIUS * scale_factor)),
        ));
    }

    // "Rays" - simulated by thin long rectangles rotated around
    for i in 0..8 {
        let angle = (i as f32) * std::f32::consts::PI / 4.0;
        parent.spawn((
            Mesh2d(cached.unit_square.clone()),
            MeshMaterial2d(cached.mat_magenta_40.clone()),
            Transform::from_xyz(0.0, 0.0, -0.05)
                .with_rotation(Quat::from_rotation_z(angle))
                .with_scale(Vec3::new(nova::RADIUS * 2.0, 4.0, 1.0)),
        ));
    }

    // Core Flash
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_white_90.clone()),
        Transform::from_xyz(0.0, 0.0, 0.1).with_scale(Vec3::splat(nova::RADIUS * 0.2)),
    ));
}

/// Spawn visual effects for Global spell
pub fn spawn_global_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    // Concentric clean rings
    for i in 0..4 {
        let r = global::RADIUS * (i as f32).mul_add(0.2, 0.4);
        // Varying opacity/color
        let mat = if i % 2 == 0 {
            cached.mat_cyan_30.clone()
        } else {
            cached.mat_purple_40.clone()
        };

        parent.spawn((
            Mesh2d(cached.unit_circle.clone()),
            MeshMaterial2d(mat),
            Transform::from_xyz(0.0, 0.0, (i as f32).mul_add(0.1, -0.5)).with_scale(Vec3::splat(r)),
        ));
    }

    // Grid-like overlay (Lat/Long lines simulated) - simplified
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_cyan_30.clone()), // Use a transparent one?
        Transform::from_xyz(0.0, 0.0, -0.1).with_scale(Vec3::splat(global::RADIUS * 0.95)),
    ));

    // Orbiting "Satellites"
    let mut rng = rand::thread_rng();
    for _ in 0..16 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let r = global::RADIUS * rng.gen_range(0.5..1.1);
        let x = angle.cos() * r;
        let y = angle.sin() * r;

        parent.spawn((
            Mesh2d(cached.unit_circle.clone()),
            MeshMaterial2d(cached.mat_white_90.clone()),
            Transform::from_xyz(x, y, 0.2).with_scale(Vec3::splat(3.0)),
        ));
    }
}

/// Spawn visual effects for Shuriken projectile
pub fn spawn_shuriken_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    // Shadow (offset and dark)
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_gun_black.clone()), // Dark shadow
        Transform::from_xyz(2.0, -2.0, -0.3).with_scale(Vec3::splat(14.0)),
    ));

    // Shuriken blades - 4 pointed star shape with "3D" lighting
    for i in 0..4 {
        let angle_base = (i as f32) * std::f32::consts::FRAC_PI_2;
        let blade_len = 12.0;

        // We simulate the diamond blade shape using two rotated rectangles or just careful placement
        // Light side of the blade
        parent.spawn((
            Mesh2d(cached.unit_square.clone()),
            MeshMaterial2d(cached.mat_teal_light.clone()),
            Transform::from_xyz(0.0, 0.0, -0.1)
                .with_rotation(Quat::from_rotation_z(angle_base))
                .with_scale(Vec3::new(blade_len, 4.0, 1.0))
                .with_translation(
                    Quat::from_rotation_z(angle_base) * Vec3::new(blade_len * 0.4, 1.5, 0.0),
                ),
        ));
        // Dark side of the blade
        parent.spawn((
            Mesh2d(cached.unit_square.clone()),
            MeshMaterial2d(cached.mat_teal_dark.clone()),
            Transform::from_xyz(0.0, 0.0, -0.1)
                .with_rotation(Quat::from_rotation_z(angle_base))
                .with_scale(Vec3::new(blade_len, 4.0, 1.0))
                .with_translation(
                    Quat::from_rotation_z(angle_base) * Vec3::new(blade_len * 0.4, -1.5, 0.0),
                ),
        ));
    }

    // Center Bearing/Ring
    // Outer metallic ring
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_teal_dark.clone()),
        Transform::from_xyz(0.0, 0.0, 0.1).with_scale(Vec3::splat(5.0)),
    ));
    // Inner light
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_white.clone()),
        Transform::from_xyz(0.0, 0.0, 0.2).with_scale(Vec3::splat(2.5)),
    ));
}

/// Spawn visual effects for Sword Normal attack - realistic sword shape
pub fn spawn_sword_normal_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    let blade_length = sword::NORMAL_RANGE;
    let blade_width = 16.0;

    // Blade Body (Steel)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_steel.clone()),
        Transform::from_xyz(blade_length * 0.4, 0.0, 0.0).with_scale(Vec3::new(
            blade_length,
            blade_width,
            1.0,
        )),
    ));

    // Blade Ridge (Brighter center line)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_steel_bright.clone()),
        Transform::from_xyz(blade_length * 0.4, 0.0, 0.1).with_scale(Vec3::new(
            blade_length * 0.95,
            blade_width * 0.4,
            1.0,
        )),
    ));

    // Guard (Gold)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_gold_polished.clone()),
        Transform::from_xyz(-blade_length * 0.1, 0.0, 0.2).with_scale(Vec3::new(
            8.0,
            blade_width * 2.8,
            1.0,
        )),
    ));

    // Handle (Dark Wood)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_wood_dark.clone()),
        Transform::from_xyz(-blade_length * 0.2, 0.0, 0.1).with_scale(Vec3::new(
            blade_length * 0.2,
            blade_width * 0.6,
            1.0,
        )),
    ));

    // Pommel (Gold)
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_gold_polished.clone()),
        Transform::from_xyz(-blade_length * 0.3, 0.0, 0.25)
            .with_scale(Vec3::splat(blade_width * 0.7)),
    ));
}

/// Spawn visual effects for Sword Shattered attack - broken blade fragments in a line
pub fn spawn_sword_shattered_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    let mut rng = rand::thread_rng();

    // Stub is proportional to range? User asked "broken_blade_len depends on sword::SHATTERED_RANGE"
    let broken_blade_len = sword::SHATTERED_RANGE * 0.15;
    let blade_width = 16.0;

    // --- Hilt Logic (Same as normal sword, but broken blade) ---
    // Guard (Gold)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_gold_polished.clone()),
        Transform::from_xyz(-10.0, 0.0, 0.2).with_scale(Vec3::new(8.0, blade_width * 2.8, 1.0)),
    ));
    // Handle (Dark Wood)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_wood_dark.clone()),
        Transform::from_xyz(-20.0, 0.0, 0.1).with_scale(Vec3::new(24.0, blade_width * 0.6, 1.0)),
    ));
    // Pommel (Gold)
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_gold_polished.clone()),
        Transform::from_xyz(-32.0, 0.0, 0.25).with_scale(Vec3::splat(blade_width * 0.7)),
    ));
    // Broken Blade Stub
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_steel.clone()), // Darker break
        Transform::from_xyz(5.0, 0.0, 0.0).with_scale(Vec3::new(
            broken_blade_len,
            blade_width,
            1.0,
        )),
    ));

    // --- Fragment Logic ---
    let num_fragments = 60;
    let total_range = sword::SHATTERED_RANGE;

    for _ in 0..num_fragments {
        let dist = rng.gen_range(20.0..total_range);
        // Scatter Y slightly
        let y_off = rng.gen_range(-10.0..10.0);

        let size_x = rng.gen_range(4.0..12.0);
        let size_y = rng.gen_range(2.0..8.0);
        let rot = rng.gen_range(-0.5..0.5);

        parent.spawn((
            Mesh2d(cached.unit_square.clone()),
            MeshMaterial2d(cached.mat_steel_bright.clone()),
            Transform::from_xyz(dist, y_off, 0.1)
                .with_rotation(Quat::from_rotation_z(rot))
                .with_scale(Vec3::new(size_x, size_y, 1.0)),
        ));
    }
}

/// Spawn visual effects for Gun bullet
pub fn spawn_gun_bullet_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    // Bullet Core (Gold/Brass)
    // Sharper, longer shape
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_gold_polished.clone()),
        Transform::from_xyz(0.0, 0.0, 0.1).with_scale(Vec3::new(18.0, 4.0, 1.0)),
    ));

    // Trail / Shockwave (Orange)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_orange_60.clone()),
        Transform::from_xyz(-8.0, 0.0, -0.1).with_scale(Vec3::new(30.0, 8.0, 1.0)),
    ));

    // Muzzle/Impact tip glow
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_yellow_100.clone()),
        Transform::from_xyz(8.0, 0.0, 0.2).with_scale(Vec3::splat(6.0)),
    ));
}
