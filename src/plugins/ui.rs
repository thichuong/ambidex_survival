use crate::systems::ui::{
    PurchaseEvent, handle_purchases, setup_ui, update_cooldown_indicators, update_gold_ui,
    update_health_ui, update_hud_indicators, update_magic_ui, update_menu_cdr_text,
    update_menu_crit_text, update_menu_damage_text, update_menu_gold_text, update_menu_health_text,
    update_menu_lifesteal_text, update_round_text, update_shuriken_count_ui, update_ui_visibility,
};
use crate::utils::log_error;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PurchaseEvent>()
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (
                    update_ui_visibility.pipe(log_error),
                    update_hud_indicators.pipe(log_error),
                    update_magic_ui.pipe(log_error),
                    update_health_ui.pipe(log_error),
                    update_gold_ui,
                    update_round_text,
                    update_menu_gold_text,
                    update_menu_health_text,
                    update_menu_damage_text,
                    update_menu_crit_text,
                    update_menu_lifesteal_text,
                    update_menu_cdr_text,
                    update_cooldown_indicators,
                    update_shuriken_count_ui,
                    handle_purchases,
                ),
            );
    }
}
