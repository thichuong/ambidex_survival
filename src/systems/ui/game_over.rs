use super::components::{GameOverUI, NewGameButton};
use crate::components::player::{CombatStats, Currency, Health, Player, Progression};
use bevy::prelude::*;

#[allow(clippy::too_many_lines)]
pub fn spawn_game_over_menu(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                display: Display::None,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
            GameOverUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: 100.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.0, 0.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(240.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0)),
                    NewGameButton,
                ))
                .observe(
                    |_trigger: On<Pointer<Click>>,
                     mut next_state: ResMut<NextState<crate::resources::game_state::GameState>>,
                     player: Single<
                        (
                            &mut Health,
                            &mut Currency,
                            &mut CombatStats,
                            &mut Progression,
                            &mut Transform,
                        ),
                        With<Player>,
                    >,
                     mut round_manager: ResMut<crate::resources::round::RoundManager>,
                     enemy_query: Query<Entity, With<crate::components::enemy::Enemy>>,
                     projectile_query: Query<
                        Entity,
                        With<crate::components::weapon::Projectile>,
                    >,
                     mut commands: Commands| {
                        // Reset Player
                        let (mut health, mut currency, mut combat, mut progression, mut transform) =
                            player.into_inner();

                        *health = Health::default();
                        *currency = Currency::default();
                        *combat = CombatStats::default();
                        *progression = Progression::default();
                        transform.translation = Vec3::ZERO;

                        // Reset Round
                        *round_manager = crate::resources::round::RoundManager::default();

                        // Despawn Enemies
                        for entity in &enemy_query {
                            commands.entity(entity).despawn();
                        }

                        // Despawn Projectiles
                        for entity in &projectile_query {
                            commands.entity(entity).despawn();
                        }

                        // Restart Game
                        next_state.set(crate::resources::game_state::GameState::Playing);
                    },
                )
                .observe(
                    |trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                        if let Ok(mut color) = color.get_mut(trigger.entity) {
                            *color = BackgroundColor(Color::srgba(0.4, 0.4, 0.4, 1.0));
                        }
                    },
                )
                .observe(
                    |trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                        if let Ok(mut color) = color.get_mut(trigger.entity) {
                            *color = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0));
                        }
                    },
                )
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("NEW GAME"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}
