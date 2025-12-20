//! Cached mesh and material handles to avoid creating new assets on every spawn
//! This improves performance by reusing GPU resources

use bevy::prelude::*;

/// Resource containing pre-cached mesh and material handles
#[derive(Resource)]
pub struct CachedAssets {
    // Common circle meshes (by radius * 10 as integer key for common sizes)
    pub circle_2: Handle<Mesh>,
    pub circle_3: Handle<Mesh>,
    pub circle_4: Handle<Mesh>,
    pub circle_5: Handle<Mesh>,
    pub circle_6: Handle<Mesh>,
    pub circle_8: Handle<Mesh>,
    pub circle_10: Handle<Mesh>,
    pub circle_11: Handle<Mesh>,
    pub circle_12: Handle<Mesh>,
    pub circle_14: Handle<Mesh>,
    pub circle_16: Handle<Mesh>,
    pub circle_25: Handle<Mesh>,

    // Common rectangle meshes for sword/gun
    pub rect_sword_blade: Handle<Mesh>,    // 120x8
    pub rect_sword_edge: Handle<Mesh>,     // 115x3
    pub rect_sword_tip: Handle<Mesh>,      // 15x6
    pub rect_sword_guard: Handle<Mesh>,    // 6x20
    pub rect_sword_handle: Handle<Mesh>,   // 25x6
    pub rect_shuriken_blade: Handle<Mesh>, // 12x5
    pub rect_shuriken_inner: Handle<Mesh>, // 10x3
    pub rect_gun_outer: Handle<Mesh>,      // 28x10
    pub rect_gun_mid: Handle<Mesh>,        // 24x7
    pub rect_gun_core: Handle<Mesh>,       // 20x4
    pub rect_gun_inner: Handle<Mesh>,      // 16x2

    // Common materials by color category
    pub mat_white: Handle<ColorMaterial>,
    pub mat_white_90: Handle<ColorMaterial>,
    pub mat_steel: Handle<ColorMaterial>,
    pub mat_steel_bright: Handle<ColorMaterial>,
    pub mat_bronze: Handle<ColorMaterial>,
    pub mat_brown: Handle<ColorMaterial>,
    pub mat_cyan_30: Handle<ColorMaterial>,
    pub mat_cyan_50: Handle<ColorMaterial>,
    pub mat_purple_20: Handle<ColorMaterial>,
    pub mat_purple_40: Handle<ColorMaterial>,
    pub mat_purple_80: Handle<ColorMaterial>,
    pub mat_magenta_40: Handle<ColorMaterial>,
    pub mat_orange_25: Handle<ColorMaterial>,
    pub mat_orange_60: Handle<ColorMaterial>,
    pub mat_yellow_100: Handle<ColorMaterial>,
}

impl CachedAssets {
    pub fn new(meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) -> Self {
        Self {
            // Circle meshes
            circle_2: meshes.add(Circle::new(2.0)),
            circle_3: meshes.add(Circle::new(3.0)),
            circle_4: meshes.add(Circle::new(4.0)),
            circle_5: meshes.add(Circle::new(5.0)),
            circle_6: meshes.add(Circle::new(6.0)),
            circle_8: meshes.add(Circle::new(8.0)),
            circle_10: meshes.add(Circle::new(10.0)),
            circle_11: meshes.add(Circle::new(11.0)),
            circle_12: meshes.add(Circle::new(12.0)),
            circle_14: meshes.add(Circle::new(14.0)),
            circle_16: meshes.add(Circle::new(16.0)),
            circle_25: meshes.add(Circle::new(25.0)),

            // Rectangle meshes for weapons
            rect_sword_blade: meshes.add(Rectangle::new(120.0, 8.0)),
            rect_sword_edge: meshes.add(Rectangle::new(115.0, 3.0)),
            rect_sword_tip: meshes.add(Rectangle::new(15.0, 6.0)),
            rect_sword_guard: meshes.add(Rectangle::new(6.0, 20.0)),
            rect_sword_handle: meshes.add(Rectangle::new(25.0, 6.0)),
            rect_shuriken_blade: meshes.add(Rectangle::new(12.0, 5.0)),
            rect_shuriken_inner: meshes.add(Rectangle::new(10.0, 3.0)),
            rect_gun_outer: meshes.add(Rectangle::new(28.0, 10.0)),
            rect_gun_mid: meshes.add(Rectangle::new(24.0, 7.0)),
            rect_gun_core: meshes.add(Rectangle::new(20.0, 4.0)),
            rect_gun_inner: meshes.add(Rectangle::new(16.0, 2.0)),

            // Materials - common colors
            mat_white: materials.add(Color::srgba(1.0, 1.0, 1.0, 1.0)),
            mat_white_90: materials.add(Color::srgba(1.0, 1.0, 1.0, 0.9)),
            mat_steel: materials.add(Color::srgba(0.75, 0.75, 0.8, 1.0)),
            mat_steel_bright: materials.add(Color::srgba(0.9, 0.9, 0.95, 1.0)),
            mat_bronze: materials.add(Color::srgba(0.3, 0.25, 0.2, 1.0)),
            mat_brown: materials.add(Color::srgba(0.4, 0.25, 0.15, 1.0)),
            mat_cyan_30: materials.add(Color::srgba(0.0, 0.8, 1.0, 0.3)),
            mat_cyan_50: materials.add(Color::srgba(0.0, 0.9, 0.8, 0.5)),
            mat_purple_20: materials.add(Color::srgba(0.6, 0.0, 0.9, 0.2)),
            mat_purple_40: materials.add(Color::srgba(0.8, 0.2, 1.0, 0.4)),
            mat_purple_80: materials.add(Color::srgba(0.9, 0.4, 1.0, 0.8)),
            mat_magenta_40: materials.add(Color::srgba(1.0, 0.5, 1.0, 0.4)),
            mat_orange_25: materials.add(Color::srgba(1.0, 0.6, 0.0, 0.25)),
            mat_orange_60: materials.add(Color::srgba(1.0, 0.9, 0.0, 0.6)),
            mat_yellow_100: materials.add(Color::srgba(1.0, 1.0, 0.8, 1.0)),
        }
    }
}
