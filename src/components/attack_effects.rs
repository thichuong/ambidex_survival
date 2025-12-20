#![allow(dead_code)]
use bevy::prelude::*;

/// Component cho attack trail effect - tạo vệt sáng theo sau projectile
#[derive(Component)]
pub struct AttackTrail {
    pub color: Color,
    pub width: f32,
    pub fade_rate: f32,
}

/// Component cho particle burst effect - particles bay ra từ điểm impact
#[derive(Component)]
pub struct ParticleBurst {
    pub count: usize,
    pub base_color: Color,
    pub speed_range: (f32, f32),
    pub size_range: (f32, f32),
    pub lifetime: f32,
}

impl Default for ParticleBurst {
    fn default() -> Self {
        Self {
            count: 10,
            base_color: Color::WHITE,
            speed_range: (50.0, 150.0),
            size_range: (2.0, 5.0),
            lifetime: 0.3,
        }
    }
}

/// Marker component cho slash arc visuals
#[derive(Component)]
pub struct SlashArc {
    pub progress: f32,
    pub max_alpha: f32,
}

impl Default for SlashArc {
    fn default() -> Self {
        Self {
            progress: 0.0,
            max_alpha: 0.8,
        }
    }
}

/// Component cho rotating glow effect (dùng cho shuriken)
#[derive(Component)]
pub struct SpinGlow {
    pub rotation_speed: f32,
    pub glow_color: Color,
    pub pulse_speed: f32,
}

impl Default for SpinGlow {
    fn default() -> Self {
        Self {
            rotation_speed: 15.0,
            glow_color: Color::srgba(0.0, 1.0, 1.0, 0.5),
            pulse_speed: 5.0,
        }
    }
}

/// Component cho muzzle flash effect
#[derive(Component)]
pub struct MuzzleFlash {
    pub intensity: f32,
    pub decay_rate: f32,
}

impl Default for MuzzleFlash {
    fn default() -> Self {
        Self {
            intensity: 1.0,
            decay_rate: 10.0,
        }
    }
}

/// Trail particle - single particle in a trail
#[derive(Component)]
pub struct TrailParticle {
    pub alpha: f32,
    pub fade_speed: f32,
}

/// Component for afterimage effects
#[derive(Component)]
pub struct Afterimage {
    pub alpha: f32,
    pub fade_speed: f32,
}
