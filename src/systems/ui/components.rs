use crate::components::player::HandType;
use crate::components::weapon::WeaponType;
use bevy::prelude::*;

#[derive(Component)]
pub struct WeaponButton {
    pub side: HandType,
    pub kind: WeaponType,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopButton {
    Heal,
    DamageUp,
    MaxHealthUp,
    CritDamageUp,        // White
    CritChanceUp,        // Blue
    LifestealUp,         // Blue
    CooldownReductionUp, // Blue
    NovaCore,            // Upgrade
}

#[derive(Event, Debug)]
pub struct PurchaseEvent {
    pub btn_type: ShopButton,
    pub entity: Entity,
}

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct WeaponMenuUI;

#[derive(Component)]
pub struct HUDHandIndicator {
    #[allow(dead_code)]
    pub side: HandType,
}

#[derive(Component)]
pub struct MenuButton;

#[derive(Component)]
pub struct MagicPanel {
    #[allow(dead_code)]
    pub side: HandType,
}

#[derive(Component)]
pub struct MagicCycleButton {
    pub side: HandType,
    pub is_primary: bool, // true = primary, false = secondary
}

#[derive(Component)]
pub struct HUDIcon {
    pub side: HandType,
}

#[derive(Component)]
pub struct CooldownOverlay {
    pub side: HandType,
}

#[derive(Component)]
pub struct ShurikenCountText {
    pub side: HandType,
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct NewGameButton;

#[derive(Component)]
pub struct GoldText;

#[derive(Component)]
pub struct RoundText;

#[derive(Component)]
pub struct MenuGoldText;

#[derive(Component)]
pub struct MenuHealthText;

#[derive(Component)]
pub struct MenuDamageText;

#[derive(Component)]
pub struct MenuCritText;

#[derive(Component)]
pub struct MenuLifestealText;

#[derive(Component)]
pub struct MenuCDRText;

#[derive(Component)]
pub struct MagicSlotIndicator {
    pub slot: crate::components::weapon::ActiveSpellSlot,
}

#[derive(Component)]
pub struct TutorialUI;

#[derive(Component)]
pub struct TutorialButton;

#[derive(Component)]
pub struct WeaponDetailPanel {
    pub side: HandType,
}

#[derive(Component)]
pub struct WeaponStateGroup {
    pub side: HandType,
    pub weapon_type: WeaponType,
}

#[derive(Component)]
pub struct HUDRoot;

#[derive(Component)]
pub struct ShopCardCount;

#[derive(Component)]
pub struct ShopCardCurrentCount;

#[derive(Component)]
pub struct ShopCardLimit;

#[derive(Component)]
pub struct InfinitySymbol;

/// Resource to track which shop card is currently selected
#[derive(Resource, Default)]
pub struct SelectedShopCard(pub Option<ShopButton>);

/// Marker for the main buy button in shop
#[derive(Component)]
pub struct ShopBuyButton;

/// Marker for buy button text (shows card name and price)
#[derive(Component)]
pub struct ShopBuyButtonText;

/// Marker for buy button price text
#[derive(Component)]
pub struct ShopBuyButtonPrice;

/// Event fired when a shop card is clicked (selection, not purchase)
#[derive(Event, Debug)]
pub struct SelectCardEvent {
    pub btn_type: ShopButton,
}

#[derive(Component)]
pub struct SettingsUI;

#[derive(Component)]
pub struct SettingsBackButton;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    LeftFire,
    RightFire,
    LeftSkill,
    RightSkill,
}

#[derive(Component)]
pub struct RebindButton(pub Action);

#[derive(Resource, Default)]
pub struct RebindState {
    pub active_action: Option<Action>,
}
