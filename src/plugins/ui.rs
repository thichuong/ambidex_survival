use crate::systems::ui::{
    PurchaseEvent, handle_purchases, setup_ui, update_cooldown_indicators, update_gold_ui,
    update_health_ui, update_hud_indicators, update_hud_magic_ui, update_menu_cdr_text,
    update_menu_crit_text, update_menu_damage_text, update_menu_gold_text, update_menu_health_text,
    update_menu_lifesteal_text, update_menu_magic_ui, update_round_text, update_shuriken_count_ui,
    update_ui_visibility,
};
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PurchaseEvent>()
            .add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui_visibility)
            .add_systems(Update, update_hud_indicators)
            .add_systems(Update, update_hud_magic_ui)
            .add_systems(Update, update_menu_magic_ui)
            .add_systems(Update, update_health_ui)
            .add_systems(Update, update_gold_ui)
            .add_systems(Update, update_round_text)
            .add_systems(Update, update_menu_gold_text)
            .add_systems(Update, update_menu_health_text)
            .add_systems(Update, update_menu_damage_text)
            .add_systems(Update, update_menu_crit_text)
            .add_systems(Update, update_menu_lifesteal_text)
            .add_systems(Update, update_menu_cdr_text)
            .add_systems(Update, update_cooldown_indicators)
            .add_systems(Update, update_shuriken_count_ui)
            .add_systems(Update, handle_purchases);
    }
}
