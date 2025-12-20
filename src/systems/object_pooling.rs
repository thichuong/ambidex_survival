use bevy::prelude::*;
use std::collections::HashMap;

use crate::components::weapon::Lifetime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectType {
    BoltExplosion,
}

#[derive(Component)]
pub struct PooledEffect {
    pub kind: EffectType,
}

#[derive(Resource, Default)]
pub struct VisualEffectPool {
    pub available: HashMap<EffectType, Vec<Entity>>,
}

/// Result of requesting a pooled effect
pub struct PooledEffectRequest {
    pub entity: Entity,
    pub is_new: bool,
}

impl VisualEffectPool {
    /// Get an available entity from the pool or prepare a new one.
    /// Returns the entity ID and a flag indicating if it's new (needs visual setup).
    /// The entity will already have the `PooledEffect` component and `Visibility::Visible`.
    pub fn spawn_or_get(
        &mut self,
        commands: &mut Commands,
        kind: EffectType,
        transform: Transform,
        lifetime: Lifetime,
    ) -> PooledEffectRequest {
        if let Some(entities) = self.available.get_mut(&kind)
            && let Some(entity) = entities.pop()
        {
            // We assume entity exists because we only add existent entities and don't despawn them elsewhere
            commands
                .entity(entity)
                .insert((Visibility::Visible, transform, lifetime));
            return PooledEffectRequest {
                entity,
                is_new: false,
            };
        }

        // Spawn new
        let entity = commands
            .spawn((
                PooledEffect { kind },
                Visibility::Visible,
                transform,
                lifetime,
            ))
            .id();

        PooledEffectRequest {
            entity,
            is_new: true,
        }
    }

    /// Return an entity to the pool.
    pub fn return_to_pool(&mut self, entity: Entity, kind: EffectType, commands: &mut Commands) {
        commands
            .entity(entity)
            .insert(Visibility::Hidden)
            .remove::<Lifetime>();
        self.available.entry(kind).or_default().push(entity);
    }
}
