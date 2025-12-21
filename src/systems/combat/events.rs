use bevy::prelude::*;

#[derive(Event, Message, Debug)]
pub struct CollisionEvent {
    pub projectile: Entity,
    pub target: Entity,
    pub position: Vec2,
    pub direction: Vec2,
}
