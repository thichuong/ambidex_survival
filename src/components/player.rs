use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self { speed: 300.0 }
    }
}

use super::weapon::WeaponType;

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
pub enum HandType {
    Left,
    Right,
}

#[derive(Component)]
pub struct Hand {
    pub hand_type: HandType,
    #[allow(dead_code)]
    pub offset: Vec3,
    pub equipped_weapon: Option<WeaponType>,
}

#[derive(Component)]
pub struct GameCamera;
