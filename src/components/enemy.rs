use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
    #[allow(dead_code)]
    pub damage: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: 50.0,
            speed: 150.0,
            damage: 10.0,
        }
    }
}
