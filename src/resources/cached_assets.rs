//! Cached mesh and material handles to avoid creating new assets on every spawn
//! This improves performance by reusing GPU resources

use bevy::prelude::*;

/// Resource containing pre-cached mesh and material handles
#[derive(Resource)]
#[allow(dead_code)]
pub struct CachedAssets {
    // Common circle meshes (by radius * 10 as integer key for common sizes)
    // Unit meshes for scaling
    pub unit_circle: Handle<Mesh>,
    pub unit_square: Handle<Mesh>,
    pub unit_triangle: Handle<Mesh>,

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
    pub mat_magenta_70: Handle<ColorMaterial>,
    pub mat_cyan_70: Handle<ColorMaterial>,
    pub mat_steel_dark: Handle<ColorMaterial>,
    pub mat_orange_25: Handle<ColorMaterial>,
    pub mat_orange_60: Handle<ColorMaterial>,
    pub mat_yellow_100: Handle<ColorMaterial>,

    // Enhanced Palette for Icon Matching
    pub mat_teal_light: Handle<ColorMaterial>, // Shuriken Light
    pub mat_teal_dark: Handle<ColorMaterial>,  // Shuriken Dark
    pub mat_shuriken_elite_light: Handle<ColorMaterial>,
    pub mat_shuriken_elite_dark: Handle<ColorMaterial>,
    pub mat_gold_polished: Handle<ColorMaterial>, // Sword/Gun details
    pub mat_wood_dark: Handle<ColorMaterial>,     // Sword Handle
    pub mat_gun_metal: Handle<ColorMaterial>,     // Gun Body
    pub mat_gun_black: Handle<ColorMaterial>,     // Gun Dark Parts
    pub mat_bolt_core: Handle<ColorMaterial>,     // Energy Bolt
    pub mat_bolt_glow: Handle<ColorMaterial>,     // Energy Bolt Glow
    pub mat_blue_50: Handle<ColorMaterial>,       // Force Pull Spiral
    pub mat_blue_dark: Handle<ColorMaterial>,     // Force Pull Void
}

impl CachedAssets {
    pub fn new(meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) -> Self {
        Self {
            // Circle meshes
            // Unit meshes
            unit_circle: meshes.add(Circle::new(1.0)),
            unit_square: meshes.add(Rectangle::new(1.0, 1.0)),
            unit_triangle: meshes.add(RegularPolygon::new(1.0, 3)),

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
            mat_magenta_70: materials.add(Color::srgba(1.0, 0.5, 1.0, 0.7)),
            mat_cyan_70: materials.add(Color::srgba(0.0, 0.9, 0.8, 0.7)),
            mat_steel_dark: materials.add(Color::srgba(0.5, 0.5, 0.55, 1.0)),
            mat_orange_25: materials.add(Color::srgba(1.0, 0.6, 0.0, 0.25)),
            mat_orange_60: materials.add(Color::srgba(1.0, 0.9, 0.0, 0.6)),
            mat_yellow_100: materials.add(Color::srgba(1.0, 1.0, 0.8, 1.0)),

            // Enhanced Palette
            mat_teal_light: materials.add(Color::srgba(0.0, 0.94, 0.94, 1.0)), // RGB(0, 240, 240)
            mat_teal_dark: materials.add(Color::srgba(0.0, 0.7, 0.7, 1.0)),    // RGB(0, 180, 180)
            mat_shuriken_elite_light: materials.add(Color::srgba(1.0, 0.5, 1.0, 1.0)),
            mat_shuriken_elite_dark: materials.add(Color::srgba(0.8, 0.2, 1.0, 1.0)),
            mat_gold_polished: materials.add(Color::srgba(0.85, 0.65, 0.12, 1.0)), // Gold
            mat_wood_dark: materials.add(Color::srgba(0.55, 0.27, 0.07, 1.0)), // Sadelebrown ish
            mat_gun_metal: materials.add(Color::srgba(0.31, 0.31, 0.35, 1.0)), // Dark Grey/Blue
            mat_gun_black: materials.add(Color::srgba(0.15, 0.15, 0.15, 1.0)), // Near Black
            mat_bolt_core: materials.add(Color::srgba(1.0, 0.94, 1.0, 1.0)),   // White/Pink tint
            mat_bolt_glow: materials.add(Color::srgba(0.7, 0.0, 1.0, 0.4)),    // Purple Glow
            mat_blue_50: materials.add(Color::srgba(0.4, 0.4, 1.0, 0.5)),      // Blue/Indigo
            mat_blue_dark: materials.add(Color::srgba(0.08, 0.08, 0.24, 1.0)), // Dark Blue Void
        }
    }
}
