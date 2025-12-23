use crate::resources::game_state::GameState;
use crate::systems::ui::{
    PurchaseEvent, SelectCardEvent, SelectedShopCard, despawn_game_over_menu, despawn_hud,
    despawn_tutorial_ui, despawn_weapon_menu, handle_card_selection, handle_menu_toggle,
    handle_purchases, setup_buy_button_observer, spawn_game_over_menu, spawn_hud,
    spawn_tutorial_ui, spawn_weapon_menu, update_cooldown_indicators, update_gold_ui,
    update_health_ui, update_hud_indicators, update_hud_magic_ui, update_menu_cdr_text,
    update_menu_crit_text, update_menu_damage_text, update_menu_gold_text, update_menu_health_text,
    update_menu_lifesteal_text, update_menu_magic_ui, update_menu_weapon_buttons,
    update_menu_weapon_details_ui, update_round_text, update_shop_cards_ui,
    update_shuriken_count_ui,
};
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedShopCard>()
            .add_message::<PurchaseEvent>()
            .add_message::<SelectCardEvent>()
            // Main Menu / Weapon Menu
            .add_systems(OnEnter(GameState::WeaponMenu), spawn_weapon_menu)
            .add_systems(OnExit(GameState::WeaponMenu), despawn_weapon_menu)
            // HUD (Playing)
            .add_systems(OnEnter(GameState::Playing), spawn_hud)
            .add_systems(OnExit(GameState::Playing), despawn_hud)
            // Tutorial
            .add_systems(OnEnter(GameState::Tutorial), spawn_tutorial_ui)
            .add_systems(OnExit(GameState::Tutorial), despawn_tutorial_ui)
            // Game Over
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over_menu)
            .add_systems(OnExit(GameState::GameOver), despawn_game_over_menu)
            // Update Systems
            .add_systems(
                Update,
                (
                    update_hud_indicators,
                    update_hud_magic_ui,
                    update_shuriken_count_ui,
                    update_health_ui,
                    update_gold_ui,
                    update_round_text,
                    update_cooldown_indicators,
                    handle_menu_toggle,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    update_menu_magic_ui,
                    update_menu_weapon_details_ui,
                    update_menu_weapon_buttons,
                    update_menu_gold_text,
                    update_menu_health_text,
                    update_menu_damage_text,
                    update_menu_crit_text,
                    update_menu_lifesteal_text,
                    update_menu_cdr_text,
                    update_shop_cards_ui,
                    handle_card_selection,
                    setup_buy_button_observer,
                    handle_purchases,
                )
                    .run_if(in_state(GameState::WeaponMenu)),
            );
    }
}
