use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: 50.0,
            speed: 150.0,
        }
    }
}
