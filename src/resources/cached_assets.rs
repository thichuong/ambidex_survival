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
}

impl CachedAssets {
    pub fn new(meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) -> Self {
        Self {
            // Circle meshes
            // Unit meshes
            unit_circle: meshes.add(Circle::new(1.0)),
            unit_square: meshes.add(Rectangle::new(1.0, 1.0)),

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
        }
    }
}
