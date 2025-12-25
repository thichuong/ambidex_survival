use super::components::{
    ActiveTab, EquipmentContainer, PurchaseEvent, SelectCardEvent, SelectedShopCard, ShopButton,
    ShopBuyButton, ShopBuyButtonPrice, ShopBuyButtonText, ShopContainer, TabButton, WeaponMenuTab,
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

const fn get_progression_count(btn_type: ShopButton, progression: &Progression) -> u32 {
    match btn_type {
        ShopButton::Heal => progression.heal_count,
        ShopButton::DamageUp => progression.damage_upgrades,
        ShopButton::MaxHealthUp => progression.max_health_upgrades,
        ShopButton::CritDamageUp => progression.crit_damage_upgrades,
        ShopButton::CritChanceUp => progression.crit_chance_upgrades,
        ShopButton::LifestealUp => progression.lifesteal_upgrades,
        ShopButton::CooldownReductionUp => progression.cdr_upgrades,
        ShopButton::NovaCore => progression.nova_core,
    }
}

fn apply_upgrade_effect(
    btn_type: ShopButton,
    value: f32,
    health: &mut Health,
    stats: &mut PlayerStats,
    combat: &mut CombatStats,
    progression: &mut Progression,
) -> bool {
    match btn_type {
        ShopButton::Heal => {
            if health.current < health.max {
                health.current = (health.current + value).min(health.max);
                progression.heal_count += 1;
                true
            } else {
                false
            }
        }
        ShopButton::DamageUp => {
            stats.damage_multiplier += value;
            progression.damage_upgrades += 1;
            true
        }
        ShopButton::MaxHealthUp => {
            health.max += value;
            health.current += value;
            progression.max_health_upgrades += 1;
            true
        }
        ShopButton::CritDamageUp => {
            combat.crit_damage += value;
            progression.crit_damage_upgrades += 1;
            true
        }
        ShopButton::CritChanceUp => {
            combat.crit_chance = (combat.crit_chance + value).min(1.0);
            progression.crit_chance_upgrades += 1;
            true
        }
        ShopButton::LifestealUp => {
            combat.lifesteal = (combat.lifesteal + value).min(0.5);
            progression.lifesteal_upgrades += 1;
            true
        }
        ShopButton::CooldownReductionUp => {
            combat.cooldown_reduction = (combat.cooldown_reduction + value).min(0.8);
            progression.cdr_upgrades += 1;
            true
        }
        ShopButton::NovaCore => {
            progression.nova_core += 1;
            true
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
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
    let count = get_progression_count(event.btn_type, &progression);
    let is_maxed = config.limit.is_some_and(|limit| count >= limit);

    if !is_maxed
        && currency.gold >= config.price
        && apply_upgrade_effect(
            event.btn_type,
            config.value,
            &mut health,
            &mut stats,
            &mut combat,
            &mut progression,
        )
    {
        currency.gold -= config.price;
        success = true;
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
        let now_maxed = config
            .limit
            .is_some_and(|limit| get_progression_count(event.btn_type, &progression) >= limit);

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

const TAB_ACTIVE_BG: Color = Color::srgba(0.15, 0.25, 0.35, 1.0);
const TAB_ACTIVE_BORDER: Color = Color::srgb(1.0, 0.8, 0.2);
const TAB_INACTIVE_BG: Color = Color::srgba(0.1, 0.1, 0.12, 1.0);
const TAB_INACTIVE_BORDER: Color = Color::NONE;

const TAB_HOVER_ACTIVE_BG: Color = Color::srgba(0.2, 0.3, 0.45, 1.0);
const TAB_HOVER_INACTIVE_BG: Color = Color::srgba(0.15, 0.15, 0.18, 1.0);

#[allow(clippy::needless_pass_by_value)]
pub fn handle_tab_hover(
    trigger: On<Pointer<Over>>,
    mut q_tabs: Query<(Entity, &mut BackgroundColor), With<TabButton>>,
    q_active: Query<&ActiveTab>,
) {
    if let Ok((entity, mut bg)) = q_tabs.get_mut(trigger.entity) {
        if q_active.contains(entity) {
            *bg = BackgroundColor(TAB_HOVER_ACTIVE_BG);
        } else {
            *bg = BackgroundColor(TAB_HOVER_INACTIVE_BG);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn handle_tab_out(
    trigger: On<Pointer<Out>>,
    mut q_tabs: Query<(Entity, &mut BackgroundColor), With<TabButton>>,
    q_active: Query<&ActiveTab>,
) {
    if let Ok((entity, mut bg)) = q_tabs.get_mut(trigger.entity) {
        if q_active.contains(entity) {
            *bg = BackgroundColor(TAB_ACTIVE_BG);
        } else {
            *bg = BackgroundColor(TAB_INACTIVE_BG);
        }
    }
}

/// Handle tab switching (Card <-> Equip)
#[allow(clippy::needless_pass_by_value)]
pub fn handle_tab_interaction(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    tab_query: Query<&TabButton>,
    mut all_tabs: Query<(Entity, &TabButton, &mut BackgroundColor, &mut BorderColor)>,
    mut shop_container: Single<&mut Node, (With<ShopContainer>, Without<EquipmentContainer>)>,
    mut equip_container: Single<&mut Node, (With<EquipmentContainer>, Without<ShopContainer>)>,
) {
    if let Ok(clicked_tab) = tab_query.get(trigger.entity) {
        // Update Tab Styles
        for (entity, _tab, mut bg, mut border) in &mut all_tabs {
            if entity == trigger.entity {
                // Active Tab
                commands.entity(entity).insert(ActiveTab);
                // Since we just clicked, we are hovering
                *bg = BackgroundColor(TAB_HOVER_ACTIVE_BG);
                *border = BorderColor::all(TAB_ACTIVE_BORDER);
            } else {
                // Inactive Tab
                commands.entity(entity).remove::<ActiveTab>();
                *bg = BackgroundColor(TAB_INACTIVE_BG);
                *border = BorderColor::all(TAB_INACTIVE_BORDER);
            }
        }

        // Update Container Visibility
        match clicked_tab.tab {
            WeaponMenuTab::Card => {
                shop_container.display = Display::Flex;
                equip_container.display = Display::None;
            }
            WeaponMenuTab::Equip => {
                shop_container.display = Display::None;
                equip_container.display = Display::Flex;
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn handle_menu_toggle(
    input: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    round_manager: Res<crate::resources::round::RoundManager>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::WeaponMenu | GameState::Settings => {
                if round_manager.has_started {
                    next_state.set(GameState::Playing);
                }
            }
            GameState::Playing | GameState::Tutorial => {
                next_state.set(GameState::WeaponMenu);
            }
            GameState::GameOver => {}
        }
    }
}
