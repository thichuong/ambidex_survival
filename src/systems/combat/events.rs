use bevy::prelude::*;

#[derive(Event, Message, Debug)]
pub struct CollisionEvent {
    pub projectile: Entity,
    pub target: Entity, // Can be Enemy or Player
    pub position: Vec2,
}

#[derive(Event, Message, Debug, Clone, Copy)]
pub struct EnemyDeathEvent {
    pub entity: Entity,
    pub position: Vec2,
}
