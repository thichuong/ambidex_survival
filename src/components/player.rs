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
    pub lifesteal: f32,          // % of damage dealt
    pub crit_chance: f32,        // 0.0 to 1.0
    pub crit_damage: f32,        // multiplier, default 2.0
    pub cooldown_reduction: f32, // 0.0 to 1.0
    // Upgrade counters for blue cards
    pub crit_chance_upgrades: u32,
    pub lifesteal_upgrades: u32,
    pub cdr_upgrades: u32,
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
            lifesteal: 0.0,
            crit_chance: 0.0,
            crit_damage: 2.0,
            cooldown_reduction: 0.0,
            crit_chance_upgrades: 0,
            lifesteal_upgrades: 0,
            cdr_upgrades: 0,
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
