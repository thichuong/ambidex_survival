//! Visual effects spawning for weapons and spells
//! Contains functions to spawn child entities with visual meshes for attack animations

use bevy::prelude::*;
use rand::Rng;

use crate::configs::spells::{global, laser, nova};
use crate::configs::weapons::sword;

/// Spawn visual effects for Energy Bolt spell
pub fn spawn_energy_bolt_visuals(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    // Outer aura - large, transparent purple
    parent.spawn((
        Mesh2d(meshes.add(Circle::new(16.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.6, 0.0, 0.9, 0.2))),
        Transform::from_xyz(0.0, 0.0, -0.3),
    ));
    // Mid glow - medium magenta
    parent.spawn((
        Mesh2d(meshes.add(Circle::new(11.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.8, 0.2, 1.0, 0.4))),
        Transform::from_xyz(0.0, 0.0, -0.2),
    ));
    // Core - bright purple
    parent.spawn((
        Mesh2d(meshes.add(Circle::new(8.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.9, 0.4, 1.0, 0.8))),
        Transform::from_xyz(0.0, 0.0, -0.1),
    ));
    // Inner core - white center
    parent.spawn((
        Mesh2d(meshes.add(Circle::new(4.0))),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 0.9, 1.0, 1.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    // Orbiting particles
    for i in 0..4 {
        let angle = (i as f32) * std::f32::consts::FRAC_PI_2;
        let x = angle.cos() * 10.0;
        let y = angle.sin() * 10.0;
        parent.spawn((
            Mesh2d(meshes.add(Circle::new(2.0))),
            MeshMaterial2d(materials.add(Color::srgba(1.0, 0.5, 1.0, 0.7))),
            Transform::from_xyz(x, y, 0.1),
        ));
    }
}

/// Spawn visual effects for Laser spell
pub fn spawn_laser_visuals(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    // Outer glow - wide, transparent cyan
    parent.spawn((
        Mesh2d(meshes.add(Rectangle::new(laser::LENGTH, laser::WIDTH * 2.5))),
        MeshMaterial2d(materials.add(Color::srgba(0.0, 0.8, 1.0, 0.15))),
        Transform::from_xyz(laser::LENGTH / 2.0, 0.0, -0.3),
    ));
    // Mid beam - brighter
    parent.spawn((
        Mesh2d(meshes.add(Rectangle::new(laser::LENGTH, laser::WIDTH * 1.5))),
        MeshMaterial2d(materials.add(Color::srgba(0.2, 0.9, 1.0, 0.4))),
        Transform::from_xyz(laser::LENGTH / 2.0, 0.0, -0.2),
    ));
    // Core beam - main laser
    parent.spawn((
        Mesh2d(meshes.add(Rectangle::new(laser::LENGTH, laser::WIDTH))),
        MeshMaterial2d(materials.add(Color::srgba(0.5, 1.0, 1.0, 0.7))),
        Transform::from_xyz(laser::LENGTH / 2.0, 0.0, -0.1),
    ));
    // Inner core - brightest white
    parent.spawn((
        Mesh2d(meshes.add(Rectangle::new(laser::LENGTH, laser::WIDTH * 0.4))),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 1.0, 0.9))),
        Transform::from_xyz(laser::LENGTH / 2.0, 0.0, 0.0),
    ));
    // Electric sparks along the beam
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let dist = rng.gen_range(20.0..laser::LENGTH - 20.0);
        let jitter_y = rng.gen_range(-laser::WIDTH * 0.6..laser::WIDTH * 0.6);
        parent.spawn((
            Mesh2d(meshes.add(Circle::new(rng.gen_range(2.0..4.0)))),
            MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 1.0, rng.gen_range(0.5..1.0)))),
            Transform::from_xyz(dist, jitter_y, 0.1),
        ));
    }
    // Impact glow at end
    parent.spawn((
        Mesh2d(meshes.add(Circle::new(laser::WIDTH))),
        MeshMaterial2d(materials.add(Color::srgba(0.0, 1.0, 1.0, 0.5))),
        Transform::from_xyz(laser::LENGTH, 0.0, -0.15),
    ));
}

/// Spawn visual effects for Nova spell
pub fn spawn_nova_visuals(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    // Outer pulse - large, fading
    parent.spawn((
        Mesh2d(meshes.add(Circle::new(nova::RADIUS))),
        MeshMaterial2d(materials.add(Color::srgba(0.8, 0.0, 0.9, 0.15))),
        Transform::from_xyz(0.0, 0.0, -0.3),
    ));
    // Mid ring - medium magenta
    parent.spawn((
        Mesh2d(meshes.add(Circle::new(nova::RADIUS * 0.75))),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 0.2, 0.8, 0.3))),
        Transform::from_xyz(0.0, 0.0, -0.2),
    ));
    // Inner ring - brighter
    parent.spawn((
        Mesh2d(meshes.add(Circle::new(nova::RADIUS * 0.5))),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 0.4, 1.0, 0.4))),
        Transform::from_xyz(0.0, 0.0, -0.1),
    ));
    // Center glow - brightest white/pink
    parent.spawn((
        Mesh2d(meshes.add(Circle::new(nova::RADIUS * 0.2))),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 0.8, 1.0, 0.7))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    // Particle bursts around the edge
    let mut rng = rand::thread_rng();
    for i in 0..12 {
        let angle = (i as f32) * std::f32::consts::PI / 6.0;
        let radius = nova::RADIUS * rng.gen_range(0.6..0.95);
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;
        parent.spawn((
            Mesh2d(meshes.add(Circle::new(rng.gen_range(3.0..6.0)))),
            MeshMaterial2d(materials.add(Color::srgba(1.0, 0.5, 1.0, rng.gen_range(0.4..0.8)))),
            Transform::from_xyz(x, y, 0.1),
        ));
    }
}

/// Spawn visual effects for Global spell
pub fn spawn_global_visuals(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    // Multiple concentric fading rings
    for i in 0..5 {
        let ring_radius = global::RADIUS * (0.2 + (i as f32) * 0.2);
        let alpha = 0.08 - (i as f32) * 0.012;
        parent.spawn((
            Mesh2d(meshes.add(Circle::new(ring_radius))),
            MeshMaterial2d(materials.add(Color::srgba(0.8 - (i as f32) * 0.1, 0.9, 1.0, alpha))),
            Transform::from_xyz(0.0, 0.0, -0.3 + (i as f32) * 0.05),
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
        let color = match color_variant {
            0 => Color::srgba(0.5, 1.0, 1.0, 0.4), // Cyan
            1 => Color::srgba(1.0, 0.7, 1.0, 0.4), // Pink
            _ => Color::srgba(1.0, 1.0, 1.0, 0.5), // White
        };
        parent.spawn((
            Mesh2d(meshes.add(Circle::new(rng.gen_range(2.0..5.0)))),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x, y, 0.1),
        ));
    }
}

/// Spawn visual effects for Shuriken projectile
pub fn spawn_shuriken_visuals(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    // Outer glow - soft circle
    parent.spawn((
        Mesh2d(meshes.add(Circle::new(14.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.0, 0.8, 1.0, 0.25))),
        Transform::from_xyz(0.0, 0.0, -0.2),
    ));
    // Spin glow - pulsing ring
    parent.spawn((
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.2, 1.0, 0.6, 0.4))),
        Transform::from_xyz(0.0, 0.0, -0.1),
    ));
    // Core - bright center
    parent.spawn((
        Mesh2d(meshes.add(Circle::new(4.0))),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 1.0, 1.0))),
        Transform::from_xyz(0.0, 0.0, 0.1),
    ));
    // Shuriken blades - 4 pointed star shape
    for i in 0..4 {
        let angle = (i as f32) * std::f32::consts::FRAC_PI_2;
        let x = angle.cos() * 6.0;
        let y = angle.sin() * 6.0;
        // Blade outer
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(12.0, 5.0))),
            MeshMaterial2d(materials.add(Color::srgba(0.0, 0.9, 0.8, 0.7))),
            Transform::from_xyz(x, y, 0.0).with_rotation(Quat::from_rotation_z(angle)),
        ));
        // Blade inner
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(10.0, 3.0))),
            MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 1.0, 0.9))),
            Transform::from_xyz(x, y, 0.05).with_rotation(Quat::from_rotation_z(angle)),
        ));
    }
}

/// Spawn visual effects for Sword Normal attack - realistic sword shape
pub fn spawn_sword_normal_visuals(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    // Blade - main sword body (silver/steel color)
    parent.spawn((
        Mesh2d(meshes.add(Rectangle::new(120.0, 8.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.75, 0.75, 0.8, 1.0))), // Steel
        Transform::from_xyz(65.0, 0.0, 0.0),
    ));
    // Blade edge highlight (lighter)
    parent.spawn((
        Mesh2d(meshes.add(Rectangle::new(115.0, 3.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.9, 0.9, 0.95, 1.0))), // Bright edge
        Transform::from_xyz(62.5, 0.0, 0.1),
    ));
    // Sword tip - pointed end (triangle approximation with smaller rect)
    parent.spawn((
        Mesh2d(meshes.add(Rectangle::new(15.0, 6.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.8, 0.8, 0.85, 1.0))),
        Transform::from_xyz(130.0, 0.0, 0.0),
    ));
    // Guard/crossguard (dark metal)
    parent.spawn((
        Mesh2d(meshes.add(Rectangle::new(6.0, 20.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.3, 0.25, 0.2, 1.0))), // Dark bronze
        Transform::from_xyz(5.0, 0.0, 0.2),
    ));
    // Handle/grip (brown leather)
    parent.spawn((
        Mesh2d(meshes.add(Rectangle::new(25.0, 6.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.4, 0.25, 0.15, 1.0))), // Brown
        Transform::from_xyz(-10.0, 0.0, 0.1),
    ));
    // Pommel (end cap)
    parent.spawn((
        Mesh2d(meshes.add(Circle::new(5.0))),
        MeshMaterial2d(materials.add(Color::srgba(0.35, 0.3, 0.25, 1.0))), // Dark bronze
        Transform::from_xyz(-22.0, 0.0, 0.2),
    ));
}

/// Spawn visual effects for Sword Shattered attack - broken blade fragments in a line
pub fn spawn_sword_shattered_visuals(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
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
        let steel_variation = rng.gen_range(0.6..0.85);

        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(frag_len, frag_width))),
            MeshMaterial2d(materials.add(Color::srgba(
                steel_variation,
                steel_variation,
                steel_variation + 0.05,
                1.0,
            ))),
            Transform::from_xyz(base_x, base_y, rng.gen_range(-0.1..0.1))
                .with_rotation(Quat::from_rotation_z(rotation)),
        ));
    }

    // Small debris particles
    for _ in 0..15 {
        let dist = rng.gen_range(20.0..total_length);
        let jitter_y = rng.gen_range(-blade_width / 2.0..blade_width / 2.0);
        let size = rng.gen_range(1.5..3.5);

        parent.spawn((
            Mesh2d(meshes.add(Circle::new(size))),
            MeshMaterial2d(materials.add(Color::srgba(0.7, 0.7, 0.75, 0.8))),
            Transform::from_xyz(dist, jitter_y, 0.1),
        ));
    }
}

/// Spawn visual effects for Gun bullet
pub fn spawn_gun_bullet_visuals(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    // Outer trail glow - orange fade
    parent.spawn((
        Mesh2d(meshes.add(Rectangle::new(28.0, 10.0))),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 0.6, 0.0, 0.25))),
        Transform::from_xyz(-4.0, 0.0, -0.2),
    ));
    // Mid glow - yellow
    parent.spawn((
        Mesh2d(meshes.add(Rectangle::new(24.0, 7.0))),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 0.9, 0.0, 0.6))),
        Transform::from_xyz(-2.0, 0.0, -0.1),
    ));
    // Core bullet - bright white/yellow
    parent.spawn((
        Mesh2d(meshes.add(Rectangle::new(20.0, 4.0))),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 0.8, 1.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    // Inner core - brightest
    parent.spawn((
        Mesh2d(meshes.add(Rectangle::new(16.0, 2.0))),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 1.0, 1.0))),
        Transform::from_xyz(0.0, 0.0, 0.1),
    ));
    // Tip glow - small circle at front
    parent.spawn((
        Mesh2d(meshes.add(Circle::new(5.0))),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 0.9, 0.3, 0.5))),
        Transform::from_xyz(10.0, 0.0, -0.15),
    ));
}
