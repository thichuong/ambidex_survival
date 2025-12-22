use super::components::{PurchaseEvent, ShopButton};
use crate::components::player::{CombatStats, Currency, Health, Player, PlayerStats, Progression};
use crate::configs::shop::get_card_config;
use crate::resources::game_state::GameState;
use bevy::prelude::*;

#[allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::too_many_lines,
    clippy::needless_pass_by_value
)]
pub fn handle_purchases(
    mut events: MessageReader<PurchaseEvent>,
    player_query: Single<
        (
            &mut Health,
            &mut Currency,
            &mut PlayerStats,
            &mut CombatStats,
            &mut Progression,
        ),
        With<Player>,
    >,
    mut color_query: Query<&mut BackgroundColor>,
    text_query: Query<(&Text, &Children)>,
    mut sub_text_query: Query<&mut Text, Without<Children>>,
) {
    let (mut health, mut currency, mut stats, mut combat, mut progression) =
        player_query.into_inner();
    for event in events.read() {
        let config = get_card_config(event.btn_type);
        let mut success = false;
        let mut message = String::new();

        // Check limit
        let count = match event.btn_type {
            ShopButton::CritChanceUp => Some(progression.crit_chance_upgrades),
            ShopButton::LifestealUp => Some(progression.lifesteal_upgrades),
            ShopButton::CooldownReductionUp => Some(progression.cdr_upgrades),
            _ => None,
        };

        if let (Some(limit), Some(c)) = (config.limit, count)
            && c >= limit
        {
            message = "MAXED".to_string();
        }

        if message.is_empty() && currency.gold >= config.price {
            match event.btn_type {
                ShopButton::Heal => {
                    if health.current < health.max {
                        currency.gold -= config.price;
                        health.current = (health.current + 30.0).min(health.max);
                        success = true;
                    }
                }
                ShopButton::DamageUp => {
                    currency.gold -= config.price;
                    stats.damage_multiplier += 0.1;
                    success = true;
                }
                ShopButton::MaxHealthUp => {
                    currency.gold -= config.price;
                    health.max += 20.0;
                    health.current += 20.0;
                    success = true;
                }
                ShopButton::CritDamageUp => {
                    currency.gold -= config.price;
                    combat.crit_damage += 0.5;
                    success = true;
                }
                ShopButton::CritChanceUp => {
                    currency.gold -= config.price;
                    combat.crit_chance = (combat.crit_chance + 0.1).min(1.0);
                    progression.crit_chance_upgrades += 1;
                    success = true;
                }
                ShopButton::LifestealUp => {
                    currency.gold -= config.price;
                    combat.lifesteal = (combat.lifesteal + 0.1).min(0.5);
                    progression.lifesteal_upgrades += 1;
                    success = true;
                }
                ShopButton::CooldownReductionUp => {
                    currency.gold -= config.price;
                    combat.cooldown_reduction = (combat.cooldown_reduction + 0.1).min(0.8);
                    progression.cdr_upgrades += 1;
                    success = true;
                }
            }
        }

        // Visual feedback
        if let Ok(mut color) = color_query.get_mut(event.entity) {
            if success {
                *color = BackgroundColor(Color::srgba(0.2, 0.8, 0.2, 1.0));
            } else {
                *color = BackgroundColor(Color::srgba(0.8, 0.2, 0.2, 1.0));
            }
        }

        // Update card text if it was a blue card or reached max
        if (success || !message.is_empty())
            && let Ok((_, children)) = text_query.get(event.entity)
        {
            for &child in children {
                if let Ok(mut text) = sub_text_query.get_mut(child) {
                    match event.btn_type {
                        ShopButton::CritChanceUp => {
                            let c = progression.crit_chance_upgrades;
                            text.0 = format!("Crit Chance\n(+10%)\n250G\n[{c}/10]");
                            if c >= 10 {
                                text.0 = "Crit Chance\nMAXED".to_string();
                            }
                        }
                        ShopButton::LifestealUp => {
                            let c = progression.lifesteal_upgrades;
                            text.0 = format!("Lifesteal\n(+10%)\n300G\n[{c}/5]");
                            if c >= 5 {
                                text.0 = "Lifesteal\nMAXED".to_string();
                            }
                        }
                        ShopButton::CooldownReductionUp => {
                            let c = progression.cdr_upgrades;
                            text.0 = format!("Magic CDR\n(+10%)\n350G\n[{c}/8]");
                            if c >= 8 {
                                text.0 = "Magic CDR\nMAXED".to_string();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn handle_menu_toggle(
    input: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::WeaponMenu => {
                next_state.set(GameState::Playing);
            }
            GameState::Playing | GameState::Tutorial => {
                next_state.set(GameState::WeaponMenu);
            }
            GameState::GameOver => {}
        }
    }
}
