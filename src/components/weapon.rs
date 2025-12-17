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
