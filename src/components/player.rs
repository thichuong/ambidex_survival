use super::physics::{Collider, Velocity};
use super::weapon::{GunState, MagicLoadout, SwordState, Weapon, WeaponType};
use bevy::prelude::*;

#[derive(Component)]
pub struct Currency {
    pub gold: u32,
}

impl Default for Currency {
    fn default() -> Self {
        Self {
            gold: crate::configs::player::STARTING_GOLD,
        }
    }
}

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub invulnerability_timer: Timer,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
            invulnerability_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct PlayerStats {
    pub speed: f32,
    pub damage_multiplier: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            speed: 300.0,
            damage_multiplier: 1.0,
        }
    }
}

#[derive(Component)]
pub struct CombatStats {
    pub lifesteal: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub cooldown_reduction: f32,
}

impl Default for CombatStats {
    fn default() -> Self {
        Self {
            lifesteal: 0.0,
            crit_chance: 0.0,
            crit_damage: 2.0,
            cooldown_reduction: 0.0,
        }
    }
}

#[derive(Component, Default)]
pub struct Progression {
    pub heal_count: u32,
    pub damage_upgrades: u32,
    pub max_health_upgrades: u32,
    pub crit_damage_upgrades: u32,
    pub crit_chance_upgrades: u32,
    pub lifesteal_upgrades: u32,
    pub cdr_upgrades: u32,
    pub nova_core: u32,
}

#[derive(Component, Default)]
#[require(
    Transform,
    Visibility,
    Velocity,
    Collider,
    Currency,
    Health,
    PlayerStats,
    CombatStats,
    Progression
)]
pub struct Player;

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
