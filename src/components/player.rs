use super::physics::{Collider, Velocity};
use super::weapon::{GunState, MagicLoadout, SwordState, Weapon, WeaponType};
use bevy::prelude::*;

#[derive(Component)]
#[require(Transform, Visibility, Velocity, Collider)]
pub struct Player {
    pub speed: f32,
    pub health: f32,
    pub invulnerability_timer: Timer,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 300.0,
            health: 100.0,
            invulnerability_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum HandType {
    #[default]
    Left,
    Right,
}

#[derive(Component)]
#[require(Transform, Visibility, Weapon, MagicLoadout, SwordState, GunState)]
pub struct Hand {
    pub side: HandType,
    #[allow(dead_code)]
    pub offset: Vec3,
    pub equipped_weapon: Option<WeaponType>,
}

#[derive(Component)]
pub struct GameCamera;
