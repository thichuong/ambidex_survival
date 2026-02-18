#![allow(clippy::cast_precision_loss)]
//! Visual effects for Force Magic (Push/Pull)

use bevy::prelude::*;
use rand::Rng;

use crate::configs::spells::force;

/// Spawn visual effects for Force Push
/// Expanding Arc effect (Orange/Yellow - matches Icon)
pub fn spawn_force_push_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    let radius = force::RADIUS;

    // 1. Core expanding ring (Bright Orange)
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_orange_60.clone()), // Matches push icon core
        Transform::from_xyz(0.0, 0.0, -0.2).with_scale(Vec3::splat(radius)),
    ));

    // 2. Outer shockwave (Faint Orange)
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_orange_25.clone()),
        Transform::from_xyz(0.0, 0.0, -0.3).with_scale(Vec3::splat(radius * 0.9)),
    ));

    // 3. Directional Arrows radiating outward (Matches Icon Motif)
    for i in 0..4 {
        let angle = (i as f32) * std::f32::consts::PI / 2.0; // 0, 90, 180, 270 degrees
        let dist = radius * 0.6;

        // Triangle Arrow Tip pointing OUT
        parent.spawn((
            Mesh2d(cached.unit_triangle.clone()),
            MeshMaterial2d(cached.mat_yellow_100.clone()), // Bright yellow tips
            Transform::from_xyz(angle.cos() * dist, angle.sin() * dist, -0.1)
                .with_rotation(Quat::from_rotation_z(angle - std::f32::consts::PI / 2.0)) // Point outward
                .with_scale(Vec3::new(radius * 0.15, radius * 0.15, 1.0)),
        ));

        // Arc segments
        // Simulated by elongated rectangles or just extra particles in arc formation
        // For simplicity and performance, using particles to suggest the arc
        for j in 0..5 {
            let arc_angle = (j as f32 - 2.0).mul_add(0.15, angle);
            let arc_dist = radius * 0.4;

            parent.spawn((
                Mesh2d(cached.unit_square.clone()),
                MeshMaterial2d(cached.mat_orange_60.clone()),
                Transform::from_xyz(
                    arc_angle.cos() * arc_dist,
                    arc_angle.sin() * arc_dist,
                    -0.15,
                )
                .with_rotation(Quat::from_rotation_z(arc_angle))
                .with_scale(Vec3::new(radius * 0.05, radius * 0.1, 1.0)),
            ));
        }
    }

    // 4. Center flash
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_white_90.clone()),
        Transform::from_xyz(0.0, 0.0, 0.1).with_scale(Vec3::splat(radius * 0.1)),
    ));
}

/// Spawn visual effects for Force Pull
/// Spiral/Implosion effect (Blue/Indigo - matches Icon)
pub fn spawn_force_pull_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    let radius = force::RADIUS;

    // 1. Suction Zone (Blue/Indigo background)
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_blue_50.clone()), // New blue material
        Transform::from_xyz(0.0, 0.0, -0.3).with_scale(Vec3::splat(radius)),
    ));

    // 2. Spirals (simulated by rotated arcs/particles)
    let mut rng = rand::thread_rng();
    for i in 0..8 {
        let angle = (i as f32) * std::f32::consts::PI / 4.0;
        let dist = radius * rng.gen_range(0.3..0.8);

        // "Spiral" particle
        parent.spawn((
            Mesh2d(cached.unit_square.clone()),
            MeshMaterial2d(cached.mat_blue_50.clone()),
            Transform::from_xyz(angle.cos() * dist, angle.sin() * dist, -0.2)
                .with_rotation(Quat::from_rotation_z(angle + 0.5)) // Slight twist for spiral look
                .with_scale(Vec3::new(radius * 0.3, radius * 0.05, 1.0)),
        ));
    }

    // 3. Inward facing Arrows (Matches Icon Motif)
    for i in 0..4 {
        let angle = (i as f32) * std::f32::consts::PI / 2.0 + std::f32::consts::PI / 4.0; // 45, 135, etc.
        let dist = radius * 0.5;

        // Triangle Arrow Tip pointing IN
        parent.spawn((
            Mesh2d(cached.unit_triangle.clone()),
            MeshMaterial2d(cached.mat_bolt_core.clone()), // White/Blueish
            Transform::from_xyz(angle.cos() * dist, angle.sin() * dist, -0.1)
                .with_rotation(Quat::from_rotation_z(angle + std::f32::consts::PI / 2.0)) // Point inward
                .with_scale(Vec3::new(radius * 0.12, radius * 0.12, 1.0)),
        ));
    }

    // 4. Singularity Center (Dark Blue/Black Void)
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_blue_dark.clone()), // New dark blue material
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(radius * 0.15)),
    ));
}
