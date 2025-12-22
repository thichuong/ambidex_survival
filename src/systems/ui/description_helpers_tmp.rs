use crate::components::weapon::{SpellType, WeaponType};

pub fn get_weapon_description(weapon_type: WeaponType) -> &'static str {
    match weapon_type {
        WeaponType::Sword => "Sword\n\nA balanced melee weapon.\nHigh damage, medium range.",
        WeaponType::Gun => "Gun\n\nRanged weapon.\nFast fire rate, low damage per shot.",
        WeaponType::Shuriken => "Shuriken\n\nThrowing weapon.\nPasses through enemies.",
        WeaponType::Magic => "Magic\n\nCast powerful spells.\nSelect a spell to see details.",
    }
}

pub fn get_spell_description(spell_type: SpellType) -> &'static str {
    match spell_type {
        SpellType::EnergyBolt => {
            "Energy Bolt\n\nBasic magic projectile.\nModerate damage and cooldown."
        }
        SpellType::Laser => "Laser\n\nInstant beam attack.\nPierces obstacles.",
        SpellType::Nova => "Nova\n\nArea of effect blast around you.\nGreat for crowd control.",
        SpellType::Blink => "Blink\n\nTeleport short distance.\nInvulnerable while dashing.",
        SpellType::Global => "Global\n\nStrike all enemies on screen.\nLong cooldown.",
    }
}
