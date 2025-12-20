use super::physics::{Collider, Velocity};
use super::weapon::{GunState, MagicLoadout, SwordState, Weapon, WeaponType};
use bevy::prelude::*;

#[derive(Component)]
#[require(Transform, Visibility, Velocity, Collider)]
pub struct Player {
    pub speed: f32,
    pub health: f32,
    pub max_health: f32,
    pub gold: u32,
    pub invulnerability_timer: Timer,
    pub damage_multiplier: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 300.0,
            health: 100.0,
            max_health: 100.0,
            gold: 0,
            invulnerability_timer: Timer::from_seconds(1.0, TimerMode::Once),
            damage_multiplier: 1.0,
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

    pub equipped_weapon: Option<WeaponType>,
}

#[derive(Component)]
pub struct GameCamera;
