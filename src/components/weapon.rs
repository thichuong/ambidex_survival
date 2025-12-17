use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum WeaponType {
    Shuriken,
    Sword,
    Bow,
    Shield,
    Magic,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub damage: f32,
    pub cooldown: f32,
    pub last_shot: f32, // Time of last shot
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            weapon_type: WeaponType::Shuriken,
            damage: 10.0,
            cooldown: 0.5,
            last_shot: 0.0,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum ShieldMode {
    Absorb,
    Reflect,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct ShieldState {
    pub is_active: bool,
    pub mode: ShieldMode,
    pub accumulated_damage: f32,
    pub shield_entity: Option<Entity>,
}

#[derive(Component)]
pub struct ShieldCollider {
    pub owner_hand: Entity, // Link back to Hand to check ShieldState
}

impl Default for ShieldState {
    fn default() -> Self {
        Self {
            is_active: false,
            mode: ShieldMode::Absorb,
            accumulated_damage: 0.0,
            shield_entity: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum SpellType {
    EnergyBolt,
    Laser,
    Nova,
    Blink,
    Global,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct MagicLoadout {
    pub primary: SpellType,
    pub secondary: SpellType,
}

impl Default for MagicLoadout {
    fn default() -> Self {
        Self {
            primary: SpellType::EnergyBolt,
            secondary: SpellType::Blink,
        }
    }
}

#[derive(Component)]
#[allow(dead_code)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub direction: Vec2,
    pub owner_entity: Entity,
}

#[derive(Component)]
pub struct Lifetime {
    pub timer: Timer,
}
