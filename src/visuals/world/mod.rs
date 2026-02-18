//! Visual effects spawning for weapons and spells
//! Contains functions to spawn child entities with visual meshes for attack animations

mod force;
mod melee;
mod projectiles;
mod spells;

pub use force::*;
pub use melee::*;
pub use projectiles::*;
pub use spells::*;
