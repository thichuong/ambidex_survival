//! Visual effects for melee weapons (Sword)

use bevy::prelude::*;
use rand::Rng;

use crate::configs::weapons::sword;

/// Spawn visual effects for Sword Normal attack - realistic sword shape
pub fn spawn_sword_normal_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    let blade_length = sword::NORMAL_RANGE;
    let blade_width = 14.0; // Slightly thinner for elegance

    let grip_offset = 12.0;
    let pivot_offset = 40.0;

    // --- Hilt Logic (Detailed) ---
    // Guard Analysis
    // Main Crossbar (Gold)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_gold_polished.clone()),
        Transform::from_xyz(grip_offset + pivot_offset, 0.0, 0.2).with_scale(Vec3::new(
            6.0,
            blade_width * 3.5,
            1.0,
        )),
    ));

    // Guard Gem (Cyan Power Source)
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_cyan_70.clone()),
        Transform::from_xyz(grip_offset + pivot_offset, 0.0, 0.25)
            .with_scale(Vec3::splat(blade_width * 0.4)),
    ));

    // Handle (Dark Wood / Leather)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_wood_dark.clone()),
        Transform::from_xyz(-10.0 + grip_offset + pivot_offset, 0.0, 0.1).with_scale(Vec3::new(
            20.0,
            blade_width * 0.5,
            1.0,
        )),
    ));

    // Pommel (Gold Sphere)
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_gold_polished.clone()),
        Transform::from_xyz(-22.0 + grip_offset + pivot_offset, 0.0, 0.25)
            .with_scale(Vec3::splat(blade_width * 0.8)),
    ));

    // --- Blade Logic ---
    // Calculate blade body length, leaving room for the tip
    let tip_len = blade_width; // Length contribution of the tip
    let body_len = blade_length - tip_len;

    // Blade Body (Steel)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_steel.clone()),
        Transform::from_xyz(body_len.mul_add(0.5, grip_offset) + pivot_offset, 0.0, 0.0)
            .with_scale(Vec3::new(body_len, blade_width, 1.0)),
    ));

    // Blade Ridge (Brighter center line, runs full length including tip area overlap)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_steel_bright.clone()),
        Transform::from_xyz(
            blade_length.mul_add(0.5, grip_offset) + pivot_offset - (tip_len * 0.5),
            0.0,
            0.1,
        )
        .with_scale(Vec3::new(
            blade_length - tip_len * 0.5,
            blade_width * 0.25,
            1.0,
        )),
    ));

    // Blade Tip (Rotated Square for point)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_steel.clone()),
        Transform::from_xyz(grip_offset + body_len + pivot_offset, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)) // 45 degrees
            .with_scale(Vec3::splat(blade_width * 0.707)), // Scale to match width
    ));
}

/// Spawn visual effects for Sword Shattered attack - broken blade fragments in a line
pub fn spawn_sword_shattered_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    let mut rng = rand::thread_rng();

    let total_range = sword::SHATTERED_RANGE;

    let blade_width = 14.0;

    let grip_offset = 12.0;
    let pivot_offset = 40.0;

    // --- Hilt Logic (Identical to Normal) ---
    // Guard
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_gold_polished.clone()),
        Transform::from_xyz(grip_offset + pivot_offset, 0.0, 0.2).with_scale(Vec3::new(
            6.0,
            blade_width * 3.5,
            1.0,
        )),
    ));

    // Guard Gem (More intense/different color for shattered?) - Let's keep it cyan but maybe overlapping
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_cyan_70.clone()),
        Transform::from_xyz(grip_offset + pivot_offset, 0.0, 0.25)
            .with_scale(Vec3::splat(blade_width * 0.5)), // Pulsing bigger
    ));

    // Handle
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_wood_dark.clone()),
        Transform::from_xyz(-10.0 + grip_offset + pivot_offset, 0.0, 0.1).with_scale(Vec3::new(
            20.0,
            blade_width * 0.5,
            1.0,
        )),
    ));

    // Pommel
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_gold_polished.clone()),
        Transform::from_xyz(-22.0 + grip_offset + pivot_offset, 0.0, 0.25)
            .with_scale(Vec3::splat(blade_width * 0.8)),
    ));

    // --- Stub Logic ---
    // Removed solid stub to make the blade appear "completely broken" as requested.
    // The hilt is the only solid part remaining.

    let break_x = grip_offset + pivot_offset; // Fragments start immediately from hilt

    // --- "Ghost Blade" Energy Field ---
    // Represents the magical range of the weapon
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_cyan_30.clone()), // Faint energy
        Transform::from_xyz(total_range * 0.5 + grip_offset + pivot_offset, 0.0, -0.1)
            .with_scale(Vec3::new(total_range, blade_width * 2.5, 1.0)),
    ));
    // Core brighter line
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_cyan_50.clone()), // Brighter core
        Transform::from_xyz(total_range * 0.5 + grip_offset + pivot_offset, 0.0, -0.05)
            .with_scale(Vec3::new(total_range * 0.9, blade_width * 0.3, 1.0)),
    ));

    // --- Fragments & Particles ---
    // 1. Large Chunks (The "rest" of the blade floating)
    let num_chunks = 8;
    for i in 0..num_chunks {
        let span_start = break_x + 10.0;
        let span_end = total_range * 0.9;
        let step = (span_end - span_start) / num_chunks as f32;
        let base_x = span_start + i as f32 * step;

        let size_x = rng.gen_range(15.0..30.0);
        let size_y = rng.gen_range(10.0..16.0);
        let y_off = rng.gen_range(-5.0..5.0);
        let rot = rng.gen_range(-0.2..0.2);

        parent.spawn((
            Mesh2d(cached.unit_triangle.clone()),
            MeshMaterial2d(cached.mat_steel.clone()),
            Transform::from_xyz(base_x + rng.gen_range(-10.0..10.0), y_off, 0.1)
                .with_rotation(Quat::from_rotation_z(rot))
                .with_scale(Vec3::new(size_x, size_y, 1.0)),
        ));
    }

    // 2. Medium Shards (Scattered)
    let num_shards = 50;
    for _ in 0..num_shards {
        let dist = rng.gen_range(10.0..total_range); // Start closer to hilt
        let y_off = rng.gen_range(-15.0..15.0); // Wider scatter
        let size_major = rng.gen_range(8.0..15.0);
        let size_minor = rng.gen_range(2.0..6.0);
        let rot = rng.gen_range(-1.0..1.0);

        parent.spawn((
            Mesh2d(cached.unit_triangle.clone()),
            MeshMaterial2d(cached.mat_steel_bright.clone()),
            Transform::from_xyz(dist + grip_offset + pivot_offset, y_off, 0.15)
                .with_rotation(Quat::from_rotation_z(rot))
                .with_scale(Vec3::new(size_major, size_minor, 1.0)),
        ));
    }

    // 3. Energy Particles / Dust (Far reach)
    let num_particles = 80;
    for _ in 0..num_particles {
        let dist = rng.gen_range(10.0..total_range); // Start closer to hilt
        let y_off = rng.gen_range(-25.0..25.0); // Very wide scatter
        let size = rng.gen_range(2.0..5.0);

        parent.spawn((
            Mesh2d(cached.unit_triangle.clone()),
            MeshMaterial2d(cached.mat_cyan_70.clone()), // Glowing bits
            Transform::from_xyz(dist + grip_offset + pivot_offset, y_off, 0.2)
                .with_rotation(Quat::from_rotation_z(rng.gen_range(0.0..3.0)))
                .with_scale(Vec3::splat(size)),
        ));
    }
}
