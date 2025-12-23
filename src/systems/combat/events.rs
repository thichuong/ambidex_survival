use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub projectile: Entity,
    pub target: Entity, // Can be Enemy or Player
    pub position: Vec2,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct EnemyDeathEvent {
    pub entity: Entity,
    pub position: Vec2,
}
