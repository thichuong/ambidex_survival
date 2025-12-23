#![allow(clippy::cast_precision_loss)]
//! Visual effects for projectile attacks (Shuriken, Gun Bullet)

use bevy::prelude::*;

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

/// Elite Shuriken visuals - Purple/Magenta
pub fn spawn_elite_shuriken_visuals(
    parent: &mut ChildSpawnerCommands,
    cached: &crate::resources::cached_assets::CachedAssets,
) {
    for i in 0..4 {
        let angle_base = (i as f32) * std::f32::consts::FRAC_PI_2;
        let blade_len = 12.0;

        // Light side of the blade
        parent.spawn((
            Mesh2d(cached.unit_square.clone()),
            MeshMaterial2d(cached.mat_shuriken_elite_light.clone()),
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
            MeshMaterial2d(cached.mat_shuriken_elite_dark.clone()),
            Transform::from_xyz(0.0, 0.0, -0.1)
                .with_rotation(Quat::from_rotation_z(angle_base))
                .with_scale(Vec3::new(blade_len, 4.0, 1.0))
                .with_translation(
                    Quat::from_rotation_z(angle_base) * Vec3::new(blade_len * 0.4, -1.5, 0.0),
                ),
        ));
    }

    // Outer metallic ring
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_shuriken_elite_dark.clone()),
        Transform::from_xyz(0.0, 0.0, 0.1).with_scale(Vec3::splat(5.0)),
    ));
    // Inner light
    parent.spawn((
        Mesh2d(cached.unit_circle.clone()),
        MeshMaterial2d(cached.mat_white.clone()),
        Transform::from_xyz(0.0, 0.0, 0.2).with_scale(Vec3::splat(2.5)),
    ));
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
