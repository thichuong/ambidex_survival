use crate::systems::combat::DamageEvent;
use bevy::prelude::*;

#[derive(Component)]
pub struct DamageText {
    pub lifetime: Timer,
    pub velocity: Vec2,
}

pub fn spawn_damage_text(mut commands: Commands, mut damage_events: EventReader<DamageEvent>) {
    for event in damage_events.read() {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("{:.0}", event.damage),
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb(1.0, 1.0, 1.0),
                        ..default()
                    },
                ),
                transform: Transform::from_translation(event.position.extend(10.0)), // z-index 10
                ..default()
            },
            DamageText {
                lifetime: Timer::from_seconds(0.8, TimerMode::Once),
                velocity: Vec2::new(0.0, 50.0), // Move up
            },
        ));
    }
}

pub fn update_damage_text(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Text, &mut DamageText)>,
) {
    for (entity, mut transform, mut text, mut damage_text) in &mut query {
        damage_text.lifetime.tick(time.delta());

        if damage_text.lifetime.finished() {
            commands.entity(entity).despawn();
        } else {
            transform.translation.x += damage_text.velocity.x * time.delta_seconds();
            transform.translation.y += damage_text.velocity.y * time.delta_seconds();

            // Fade out
            let alpha = damage_text.lifetime.fraction_remaining();
            for section in &mut text.sections {
                section.style.color.set_alpha(alpha);
            }
        }
    }
}
