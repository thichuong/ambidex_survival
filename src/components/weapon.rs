use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum WeaponType {
    Shuriken,
    Sword,
    Bow,
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

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum SpellType {
    EnergyBolt,
    Laser,
    Nova,
    Blink,
    Global,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActiveSpellSlot {
    Primary,
    Secondary,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct MagicLoadout {
    pub primary: SpellType,
    pub secondary: SpellType,
    pub active_slot: ActiveSpellSlot,
}

impl Default for MagicLoadout {
    fn default() -> Self {
        Self {
            primary: SpellType::EnergyBolt,
            secondary: SpellType::Blink,
            active_slot: ActiveSpellSlot::Primary,
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
pub struct ExplodingProjectile {
    pub radius: f32,
    pub damage: f32,
}

#[derive(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SwingState {
    Windup,
    Swinging,
    Recover,
}

#[derive(Component)]
pub struct SwordSwing {
    pub state: SwingState,
    pub timer: Timer,
    pub base_angle: f32,
    pub owner_entity: Entity,
    pub damage: f32,
    pub range: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwordMode {
    Normal,
    Shattered,
}

#[derive(Component)]
pub struct SwordState {
    pub mode: SwordMode,
}

impl Default for SwordState {
    fn default() -> Self {
        Self {
            mode: SwordMode::Normal,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BowMode {
    Single,
    Multishot,
    Rapid,
}

#[derive(Component)]
pub struct BowState {
    pub mode: BowMode,
}

impl Default for BowState {
    fn default() -> Self {
        Self {
            mode: BowMode::Single,
        }
    }
}
