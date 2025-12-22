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
) {
    let (mut health, mut currency, mut stats, mut combat, mut progression) =
        player_query.into_inner();
    for event in events.read() {
        let config = get_card_config(event.btn_type);
        let mut success = false;

        // Check limit
        let count = match event.btn_type {
            ShopButton::Heal => Some(progression.heal_count),
            ShopButton::DamageUp => Some(progression.damage_upgrades),
            ShopButton::MaxHealthUp => Some(progression.max_health_upgrades),
            ShopButton::CritDamageUp => Some(progression.crit_damage_upgrades),
            ShopButton::CritChanceUp => Some(progression.crit_chance_upgrades),
            ShopButton::LifestealUp => Some(progression.lifesteal_upgrades),
            ShopButton::CooldownReductionUp => Some(progression.cdr_upgrades),
        };

        let is_maxed = if let (Some(limit), Some(c)) = (config.limit, count) {
            c >= limit
        } else {
            false
        };

        if !is_maxed && currency.gold >= config.price {
            match event.btn_type {
                ShopButton::Heal => {
                    if health.current < health.max {
                        currency.gold -= config.price;
                        health.current = (health.current + config.value).min(health.max);
                        progression.heal_count += 1;
                        success = true;
                    }
                }
                ShopButton::DamageUp => {
                    currency.gold -= config.price;
                    stats.damage_multiplier += config.value;
                    progression.damage_upgrades += 1;
                    success = true;
                }
                ShopButton::MaxHealthUp => {
                    currency.gold -= config.price;
                    health.max += config.value;
                    health.current += config.value;
                    progression.max_health_upgrades += 1;
                    success = true;
                }
                ShopButton::CritDamageUp => {
                    currency.gold -= config.price;
                    combat.crit_damage += config.value;
                    progression.crit_damage_upgrades += 1;
                    success = true;
                }
                ShopButton::CritChanceUp => {
                    currency.gold -= config.price;
                    combat.crit_chance = (combat.crit_chance + config.value).min(1.0);
                    progression.crit_chance_upgrades += 1;
                    success = true;
                }
                ShopButton::LifestealUp => {
                    currency.gold -= config.price;
                    combat.lifesteal = (combat.lifesteal + config.value).min(0.5);
                    progression.lifesteal_upgrades += 1;
                    success = true;
                }
                ShopButton::CooldownReductionUp => {
                    currency.gold -= config.price;
                    combat.cooldown_reduction = (combat.cooldown_reduction + config.value).min(0.8);
                    progression.cdr_upgrades += 1;
                    success = true;
                }
            }
        }

        // Visual feedback
        if let Ok(mut color) = color_query.get_mut(event.entity) {
            if success {
                *color = BackgroundColor(Color::srgba(0.2, 0.8, 0.2, 1.0));
            } else if is_maxed || currency.gold < config.price {
                *color = BackgroundColor(Color::srgba(0.8, 0.2, 0.2, 1.0));
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
