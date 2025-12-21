use crate::systems::combat::DamageEvent;
use bevy::prelude::*;

#[derive(Component)]
pub struct DamageText {
    pub lifetime: Timer,
    pub velocity: Vec2,
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
pub fn spawn_damage_text(
    mut commands: Commands,
    mut damage_events: MessageReader<DamageEvent>,
) -> Result<(), String> {
    for event in damage_events.read() {
        let size = if event.is_crit {
            crate::configs::visuals::DAMAGE_TEXT_SIZE_CRIT
        } else {
            crate::configs::visuals::DAMAGE_TEXT_SIZE_NORMAL
        };
        let color = if event.is_crit {
            Color::srgb(1.0, 0.0, 0.0) // Red
        } else {
            Color::srgb(1.0, 1.0, 1.0) // White
        };

        if event.is_crit {
            // Spawn white border/shadow layers
            let offsets = [
                Vec2::new(-1.0, -1.0),
                Vec2::new(1.0, -1.0),
                Vec2::new(-1.0, 1.0),
                Vec2::new(1.0, 1.0),
            ];

            for offset in offsets {
                commands.spawn((
                    Text2d::new(format!("{:.0}", event.damage)),
                    TextFont {
                        font_size: size,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 1.0, 1.0)), // White border
                    Transform::from_translation(
                        (event.position + offset)
                            .extend(crate::configs::visuals::DAMAGE_TEXT_Z_INDEX - 1.0), // Slightly behind
                    ),
                    DamageText {
                        lifetime: Timer::from_seconds(
                            crate::configs::visuals::DAMAGE_TEXT_LIFETIME,
                            TimerMode::Once,
                        ),
                        velocity: crate::configs::visuals::DAMAGE_TEXT_VELOCITY,
                    },
                ));
            }
        }

        // Main text
        commands.spawn((
            Text2d::new(format!("{:.0}", event.damage)),
            TextFont {
                font_size: size,
                ..default()
            },
            TextColor(color),
            Transform::from_translation(
                event
                    .position
                    .extend(crate::configs::visuals::DAMAGE_TEXT_Z_INDEX),
            ),
            DamageText {
                lifetime: Timer::from_seconds(
                    crate::configs::visuals::DAMAGE_TEXT_LIFETIME,
                    TimerMode::Once,
                ),
                velocity: crate::configs::visuals::DAMAGE_TEXT_VELOCITY,
            },
        ));
    }
    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
pub fn update_damage_text(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut TextColor, &mut DamageText)>,
) -> Result<(), String> {
    for (entity, mut transform, mut text_color, mut damage_text) in &mut query {
        damage_text.lifetime.tick(time.delta());

        if damage_text.lifetime.is_finished() {
            commands.entity(entity).despawn();
        } else {
            transform.translation.x += damage_text.velocity.x * time.delta_secs();
            transform.translation.y += damage_text.velocity.y * time.delta_secs();

            // Fade out
            let alpha = damage_text.lifetime.fraction_remaining();
            text_color.0.set_alpha(alpha);
        }
    }
    Ok(())
}
