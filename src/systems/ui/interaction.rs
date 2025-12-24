use super::components::{
    PurchaseEvent, SelectCardEvent, SelectedShopCard, ShopButton, ShopBuyButton,
    ShopBuyButtonPrice, ShopBuyButtonText,
};
use crate::components::player::{CombatStats, Currency, Health, Player, PlayerStats, Progression};
use crate::configs::shop::get_card_config;
use crate::resources::game_state::GameState;
use crate::systems::ui::menu::shop::get_shop_button_content;
use bevy::prelude::*;

/// Handle card selection when clicking on shop cards
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub fn handle_card_selection(
    trigger: On<SelectCardEvent>,
    mut selected: ResMut<SelectedShopCard>,
    progression: Single<&Progression, With<Player>>,
    mut buy_btn_query: Query<&mut Node, With<ShopBuyButton>>,
    mut buy_text_query: Query<&mut Text, With<ShopBuyButtonText>>,
    mut buy_price_query: Query<&mut Text, (With<ShopBuyButtonPrice>, Without<ShopBuyButtonText>)>,
    mut card_query: Query<(&ShopButton, &mut BorderColor)>,
) {
    let event = trigger.event();
    // Check if card is already maxed
    let config = get_card_config(event.btn_type);
    let count = match event.btn_type {
        ShopButton::Heal => progression.heal_count,
        ShopButton::DamageUp => progression.damage_upgrades,
        ShopButton::MaxHealthUp => progression.max_health_upgrades,
        ShopButton::CritDamageUp => progression.crit_damage_upgrades,
        ShopButton::CritChanceUp => progression.crit_chance_upgrades,
        ShopButton::LifestealUp => progression.lifesteal_upgrades,
        ShopButton::CooldownReductionUp => progression.cdr_upgrades,
        ShopButton::NovaCore => progression.nova_core,
    };
    let is_maxed = config.limit.is_some_and(|limit| count >= limit);

    // If maxed, don't show buy button
    if is_maxed {
        return;
    }

    // Update selected card
    selected.0 = Some(event.btn_type);

    // Update buy button visibility
    for mut node in &mut buy_btn_query {
        node.display = Display::Flex;
    }

    // Update title text
    let (title, _desc, price) = get_shop_button_content(event.btn_type);
    for mut text in &mut buy_text_query {
        text.0 = format!("BUY {title}");
    }

    // Update price text
    for mut text in &mut buy_price_query {
        text.0.clone_from(&price);
    }

    // Highlight selected card, reset others
    for (btn_type, mut border) in &mut card_query {
        if *btn_type == event.btn_type {
            // Highlight selected with bright gold border
            *border = BorderColor::all(Color::srgb(1.0, 0.9, 0.3));
        } else {
            // Reset to original color
            let (original_border, _, _, _) =
                crate::systems::ui::menu::shop::get_shop_button_colors(*btn_type);
            *border = BorderColor::all(original_border);
        }
    }
}

/// Observer for buy button click - sends `PurchaseEvent`
pub fn setup_buy_button_observer(
    mut commands: Commands,
    buy_btn_query: Query<Entity, Added<ShopBuyButton>>,
) {
    for entity in &buy_btn_query {
        commands.entity(entity).observe(
            |trigger: On<Pointer<Click>>,
             selected: Res<SelectedShopCard>,
             mut commands: Commands| {
                if let Some(btn_type) = selected.0 {
                    commands.trigger(PurchaseEvent {
                        btn_type,
                        entity: trigger.entity,
                    });
                }
            },
        );
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::too_many_lines,
    clippy::needless_pass_by_value
)]
pub fn handle_purchases(
    trigger: On<PurchaseEvent>,
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
    mut selected: ResMut<SelectedShopCard>,
    mut buy_btn_query: Query<&mut Node, With<ShopBuyButton>>,
    mut card_border_query: Query<(&ShopButton, &mut BorderColor)>,
) {
    let event = trigger.event();
    let (mut health, mut currency, mut stats, mut combat, mut progression) =
        player_query.into_inner();
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
        ShopButton::NovaCore => Some(progression.nova_core),
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
            ShopButton::NovaCore => {
                currency.gold -= config.price;
                progression.nova_core += 1;
                success = true;
            }
        }
    }

    // Visual feedback on buy button
    if let Ok(mut color) = color_query.get_mut(event.entity) {
        if success {
            *color = BackgroundColor(Color::srgba(0.2, 0.8, 0.2, 1.0));
        } else if is_maxed || currency.gold < config.price {
            *color = BackgroundColor(Color::srgba(0.8, 0.2, 0.2, 1.0));
        }
    }

    // Check if card is now maxed after purchase - only then hide buy button
    if success {
        // Get updated count after purchase
        let new_count = match event.btn_type {
            ShopButton::Heal => progression.heal_count,
            ShopButton::DamageUp => progression.damage_upgrades,
            ShopButton::MaxHealthUp => progression.max_health_upgrades,
            ShopButton::CritDamageUp => progression.crit_damage_upgrades,
            ShopButton::CritChanceUp => progression.crit_chance_upgrades,
            ShopButton::LifestealUp => progression.lifesteal_upgrades,
            ShopButton::CooldownReductionUp => progression.cdr_upgrades,
            ShopButton::NovaCore => progression.nova_core,
        };

        let now_maxed = config.limit.is_some_and(|limit| new_count >= limit);

        // Only hide buy button if card is now maxed
        if now_maxed {
            selected.0 = None;
            for mut node in &mut buy_btn_query {
                node.display = Display::None;
            }
            // Reset all card borders
            for (btn_type, mut border) in &mut card_border_query {
                let (original, _, _, _) =
                    crate::systems::ui::menu::shop::get_shop_button_colors(*btn_type);
                *border = BorderColor::all(original);
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
            GameState::MainMenu | GameState::GameOver => {}
        }
    }
}
