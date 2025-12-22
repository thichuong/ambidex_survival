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
}

#[derive(Event, Message, Debug)]
pub struct PurchaseEvent {
    pub btn_type: ShopButton,
    pub entity: Entity,
}

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
