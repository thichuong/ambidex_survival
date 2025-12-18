use crate::systems::combat::DamageEvent;
use bevy::prelude::*;

#[derive(Component)]
pub struct DamageText {
    pub lifetime: Timer,
    pub velocity: Vec2,
}

#[allow(clippy::needless_pass_by_value)]
pub fn spawn_damage_text(mut commands: Commands, mut damage_events: MessageReader<DamageEvent>) {
    for event in damage_events.read() {
        commands.spawn((
            Text2d::new(format!("{:.0}", event.damage)),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 1.0)),
            Transform::from_translation(event.position.extend(10.0)), // z-index 10
            DamageText {
                lifetime: Timer::from_seconds(0.8, TimerMode::Once),
                velocity: Vec2::new(0.0, 50.0), // Move up
            },
        ));
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_damage_text(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut TextColor, &mut DamageText)>,
) {
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
}
