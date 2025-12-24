//! Visual effects for melee weapons (Sword)

use bevy::prelude::*;
use rand::Rng;

/// Spawn visual effects for Sword Normal attack - realistic sword shape
pub fn spawn_sword_normal_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
    range: f32,
) {
    let grip_offset = 12.0;
    let pivot_offset = 40.0;
    let hilt_total = grip_offset + pivot_offset;
    let blade_length = (range - hilt_total).max(20.0);
    let blade_width = 14.0;

    spawn_sword_hilt(parent, cached, blade_width, grip_offset, pivot_offset, 0.4);

    // --- Blade Logic ---
    let tip_len = blade_width;
    let body_len = blade_length - tip_len;

    // Blade Body (Steel)
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_steel.clone()),
        Transform::from_xyz(body_len.mul_add(0.5, grip_offset) + pivot_offset, 0.0, 0.0)
            .with_scale(Vec3::new(body_len, blade_width, 1.0)),
    ));

    // Blade Ridge
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

    // Blade Tip
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_steel.clone()),
        Transform::from_xyz(grip_offset + body_len + pivot_offset, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4))
            .with_scale(Vec3::splat(blade_width * 0.707)),
    ));
}

/// Spawn visual effects for Sword Shattered attack - broken blade fragments in a line
pub fn spawn_sword_shattered_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
    range: f32,
) {
    let blade_width = 14.0;
    let grip_offset = 12.0;
    let pivot_offset = 40.0;
    let break_x = grip_offset + pivot_offset;
    let total_range = range;

    // 1. Hilt
    spawn_sword_hilt(parent, cached, blade_width, grip_offset, pivot_offset, 0.5);

    // 2. "Ghost Blade" Energy Field
    spawn_shattered_energy_field(
        parent,
        cached,
        total_range,
        blade_width,
        grip_offset,
        pivot_offset,
    );

    // 3. Fragments & Particles
    spawn_shattered_fragments(
        parent,
        cached,
        total_range,
        grip_offset,
        pivot_offset,
        break_x,
    );
}

fn spawn_sword_hilt(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
    blade_width: f32,
    grip_offset: f32,
    pivot_offset: f32,
    gem_scale_mult: f32,
) {
    // Main Crossbar
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_gold_polished.clone()),
        Transform::from_xyz(grip_offset + pivot_offset, 0.0, 0.2).with_scale(Vec3::new(
            6.0,
            blade_width * 3.5,
            1.0,
        )),
    ));

    // Guard Gem
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_cyan_70.clone()),
        Transform::from_xyz(grip_offset + pivot_offset, 0.0, 0.25)
            .with_scale(Vec3::splat(blade_width * gem_scale_mult)),
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
}

fn spawn_shattered_energy_field(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
    total_range: f32,
    blade_width: f32,
    grip_offset: f32,
    pivot_offset: f32,
) {
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_cyan_30.clone()),
        Transform::from_xyz(
            total_range.mul_add(0.5, grip_offset) + pivot_offset,
            0.0,
            -0.1,
        )
        .with_scale(Vec3::new(total_range, blade_width * 2.5, 1.0)),
    ));
    parent.spawn((
        Mesh2d(cached.unit_square.clone()),
        MeshMaterial2d(cached.mat_cyan_50.clone()),
        Transform::from_xyz(
            total_range.mul_add(0.5, grip_offset) + pivot_offset,
            0.0,
            -0.05,
        )
        .with_scale(Vec3::new(total_range * 0.9, blade_width * 0.3, 1.0)),
    ));
}

fn spawn_shattered_fragments(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
    total_range: f32,
    grip_offset: f32,
    pivot_offset: f32,
    break_x: f32,
) {
    let mut rng = rand::thread_rng();

    // 1. Large Chunks
    let num_chunks: u16 = 8;
    for i in 0..num_chunks {
        let span_start = break_x + 10.0;
        let span_end = total_range * 0.9;
        let step = (span_end - span_start) / f32::from(num_chunks);
        let base_x = f32::from(i).mul_add(step, span_start);

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

    // 2. Medium Shards
    let num_shards = 50;
    for _ in 0..num_shards {
        let dist = rng.gen_range(10.0..total_range);
        let y_off = rng.gen_range(-15.0..15.0);
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

    // 3. Energy Particles
    let num_particles = 80;
    for _ in 0..num_particles {
        let dist = rng.gen_range(10.0..total_range);
        let y_off = rng.gen_range(-25.0..25.0);
        let size = rng.gen_range(2.0..5.0);

        parent.spawn((
            Mesh2d(cached.unit_triangle.clone()),
            MeshMaterial2d(cached.mat_cyan_70.clone()),
            Transform::from_xyz(dist + grip_offset + pivot_offset, y_off, 0.2)
                .with_rotation(Quat::from_rotation_z(rng.gen_range(0.0..3.0)))
                .with_scale(Vec3::splat(size)),
        ));
    }
}
