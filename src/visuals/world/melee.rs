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
    let blade_width = 16.0;

    let grip_offset = 10.0;

    // --- Blade Logic ---
    // Blade Body (Steel)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_steel.clone()),
        Transform::from_xyz(blade_length.mul_add(0.5, grip_offset), 0.0, 0.0)
            .with_scale(Vec3::new(blade_length, blade_width, 1.0)),
    ));

    // Blade Ridge (Brighter center line)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_steel_bright.clone()),
        Transform::from_xyz(blade_length.mul_add(0.5, grip_offset), 0.0, 0.1)
            .with_scale(Vec3::new(blade_length * 0.95, blade_width * 0.4, 1.0)),
    ));

    // --- Hilt Logic (Standardized) ---
    // Guard (Gold)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_gold_polished.clone()),
        Transform::from_xyz(grip_offset, 0.0, 0.2).with_scale(Vec3::new(
            8.0,
            blade_width * 2.8,
            1.0,
        )),
    ));

    // Handle (Dark Wood)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_wood_dark.clone()),
        Transform::from_xyz(-12.0 + grip_offset, 0.0, 0.1).with_scale(Vec3::new(
            24.0,
            blade_width * 0.6,
            1.0,
        )),
    ));

    // Pommel (Gold)
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_gold_polished.clone()),
        Transform::from_xyz(-24.0 + grip_offset, 0.0, 0.25)
            .with_scale(Vec3::splat(blade_width * 0.7)),
    ));
}

/// Spawn visual effects for Sword Shattered attack - broken blade fragments in a line
pub fn spawn_sword_shattered_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    let mut rng = rand::thread_rng();

    let broken_blade_len = sword::SHATTERED_RANGE * 0.15;
    let blade_width = 16.0;

    let grip_offset = 10.0;

    // --- Hilt Logic (Standardized) ---
    // Guard (Gold)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_gold_polished.clone()),
        Transform::from_xyz(grip_offset, 0.0, 0.2).with_scale(Vec3::new(
            8.0,
            blade_width * 2.8,
            1.0,
        )),
    ));

    // Handle (Dark Wood)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_wood_dark.clone()),
        Transform::from_xyz(-12.0 + grip_offset, 0.0, 0.1).with_scale(Vec3::new(
            24.0,
            blade_width * 0.6,
            1.0,
        )),
    ));

    // Pommel (Gold)
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_gold_polished.clone()),
        Transform::from_xyz(-24.0 + grip_offset, 0.0, 0.25)
            .with_scale(Vec3::splat(blade_width * 0.7)),
    ));

    // --- Blade Logic ---
    // Broken Blade Stub
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_steel.clone()),
        Transform::from_xyz(broken_blade_len * 0.5 + grip_offset, 0.0, 0.0).with_scale(Vec3::new(
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
        let y_off = rng.gen_range(-10.0..10.0);
        let size_x = rng.gen_range(4.0..12.0);
        let size_y = rng.gen_range(2.0..8.0);
        let rot = rng.gen_range(-0.5..0.5);

        parent.spawn((
            Mesh2d(cached.unit_square.clone()),
            MeshMaterial2d(cached.mat_steel_bright.clone()),
            Transform::from_xyz(dist + grip_offset, y_off, 0.1)
                .with_rotation(Quat::from_rotation_z(rot))
                .with_scale(Vec3::new(size_x, size_y, 1.0)),
        ));
    }
}
