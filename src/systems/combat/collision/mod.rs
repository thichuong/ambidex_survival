//! Collision detection and handling module
//!
//! This module handles:
//! - Collision detection between projectiles and enemies
//! - Damage processing when collisions occur
//! - Projectile effects (explosions, despawning)
//! - Enemy death handling

mod damage;
mod detection;
mod effects;
mod enemy_death;

pub use damage::*;
pub use detection::*;
pub use effects::*;
pub use enemy_death::*;

use super::PendingDespawn;
use crate::components::physics::{Collider, IgnoreGrid};
use crate::components::weapon::{AoEProjectile, Projectile};
use bevy::prelude::*;

/// Query type for projectile collision detection
pub type ProjectileQueryItem<'a> = (
    Entity,
    &'a Projectile,
    &'a Transform,
    &'a Collider,
    Option<Mut<'a, AoEProjectile>>,
    Option<&'a IgnoreGrid>,
    &'a Visibility,
    Option<&'a PendingDespawn>,
);
