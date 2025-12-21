use bevy::prelude::*;

#[derive(Event, Message, Debug)]
pub struct CollisionEvent {
    pub projectile: Entity,
    pub target: Entity,
    pub position: Vec2,
    #[allow(dead_code)]
    pub direction: Vec2,
}

#[derive(Event, Message, Debug)]
pub struct EnemyDeathEvent {
    pub entity: Entity,
    pub position: Vec2,
}
