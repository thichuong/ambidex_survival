use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForceType {
    Push,
    Pull,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatusEffect {
    #[allow(dead_code)]
    Rooted {
        timer: Timer,
    },
    ForcedMovement {
        timer: Timer,
        direction: Vec2,
        speed: f32,
        move_type: ForceType,
    },
}

#[derive(Component, Default, Debug)]
pub struct UnitStatus {
    pub effects: Vec<StatusEffect>,
}

impl UnitStatus {
    pub fn add(&mut self, effect: StatusEffect) {
        // Remove existing similar effects if needed, or just push
        // For Force Push/Pull, we probably want to override existing force?
        // Let's just push for now, but usually you might want only one forced movement at a time.
        // If we want to replace existing forced movement:
        if let StatusEffect::ForcedMovement { .. } = effect {
            self.effects.retain(|e| !matches!(e, StatusEffect::ForcedMovement { .. }));
        }
         // If Rooted, maybe we don't want to stack multiple roots, just refresh timer?
        if let StatusEffect::Rooted { .. } = effect {
             self.effects.retain(|e| !matches!(e, StatusEffect::Rooted { .. }));
        }
        self.effects.push(effect);
    }

    #[allow(dead_code)]
    pub fn root(&mut self, duration: f32) {
        self.add(StatusEffect::Rooted {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        });
    }

    pub fn is_rooted(&self) -> bool {
        self.effects.iter().any(|e| matches!(e, StatusEffect::Rooted { .. } | StatusEffect::ForcedMovement { .. }))
    }
}

