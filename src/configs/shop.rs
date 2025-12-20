use crate::systems::ui::ShopButton;

#[allow(dead_code)]
pub struct CardConfig {
    pub name: &'static str,
    pub price: u32,
    pub limit: Option<u32>,
    pub is_blue: bool,
    pub description: &'static str,
}

pub const fn get_card_config(btn_type: ShopButton) -> CardConfig {
    match btn_type {
        ShopButton::Heal => CardConfig {
            name: "Heal",
            price: 50,
            limit: None,
            is_blue: false,
            description: "Restore 30 HP",
        },
        ShopButton::DamageUp => CardConfig {
            name: "Damage Up",
            price: 100,
            limit: None,
            is_blue: false,
            description: "+10% Damage",
        },
        ShopButton::MaxHealthUp => CardConfig {
            name: "Max Health Up",
            price: 150,
            limit: None,
            is_blue: false,
            description: "+20 Max HP",
        },
        ShopButton::CritDamageUp => CardConfig {
            name: "Crit Damage",
            price: 200,
            limit: None,
            is_blue: false,
            description: "+50% Crit Dmg",
        },
        ShopButton::CritChanceUp => CardConfig {
            name: "Crit Chance",
            price: 250,
            limit: Some(10),
            is_blue: true,
            description: "+10% Crit Chance",
        },
        ShopButton::LifestealUp => CardConfig {
            name: "Lifesteal",
            price: 300,
            limit: Some(5),
            is_blue: true,
            description: "+10% Lifesteal",
        },
        ShopButton::CooldownReductionUp => CardConfig {
            name: "Magic CDR",
            price: 350,
            limit: Some(8),
            is_blue: true,
            description: "+10% CDR",
        },
    }
}
