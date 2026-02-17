use super::systems::WeaponDescriptionText;
use crate::components::player::HandType;
use crate::components::weapon::{MagicLoadout, SpellType, WeaponType};
use crate::configs::visuals::{
    MAGIC_DIVIDER_COLOR, MAGIC_INFO_BG, MAGIC_SLOT_BG, MAGIC_SLOT_BG_HOVER,
    MAGIC_SLOT_BORDER_DEFAULT, MAGIC_SLOT_BORDER_HIGHLIGHT,
};
use bevy::prelude::*;

pub fn get_spell_description(spell_type: SpellType) -> String {
    match spell_type {
        SpellType::EnergyBolt => "Energy Bolt: Creates a large explosion on impact.".to_string(),
        SpellType::Laser => "Laser: Instant-hit high-velocity beam.".to_string(),
        SpellType::Nova => "Nova: Radial burst of high area damage.".to_string(),
        SpellType::Blink => "Blink: Teleport to cursor & Invulnerable.".to_string(),
        SpellType::Global => "Global: Massive strike hitting ALL enemies.".to_string(),
        SpellType::ForcePush => {
            "Force Push: Push enemies away. More damage when close.".to_string()
        }
        SpellType::ForcePull => "Force Pull: Pull enemies in. More damage when far.".to_string(),
    }
}

pub fn get_weapon_description(weapon_type: WeaponType, loadout: Option<&MagicLoadout>) -> String {
    match weapon_type {
        WeaponType::Sword => {
            "Sword (Melee)\n\nNormal Mode: Moderate range, high damage.\nShattered Mode (Skill): Blade fragments cover massive area, lower damage.".to_string()
        }
        WeaponType::Gun => {
            "Gun (Firearm)\n\nModes: Single, Shotgun, Rapid.\nSkill Cycle: Toggle between modes.\nRapid: Hold to spray.".to_string()
        }
        WeaponType::Shuriken => "Shuriken (Utility)\n\nAttack: Throw fast-moving stars (Max 12).\nSkill: Teleport to nearest shuriken.\nGreat for dodging.".to_string(),
        WeaponType::Magic => {
            loadout.map_or_else(
                || "Magic (Spellcasting)\n\nMost customizable weapon.\nTwo spell slots (Primary/Secondary).\nSelect a spell to see details.".to_string(),
                |loadout| {
                    let p_desc = get_spell_description(loadout.primary);
                    let s_desc = get_spell_description(loadout.secondary);
                    format!("Magic (Spellcasting)\n\nPrimary - {p_desc}\n\nSecondary - {s_desc}\n\nSkill: Toggle Spell Slot.\nBenefits from CDR upgrades.")
                },
            )
        }
    }
}

pub fn spawn_weapon_button(
    parent: &mut ChildSpawnerCommands,
    side: HandType,
    kind: WeaponType,
    label: &str,
) {
    use super::components::ArsenalButton;
    use crate::components::player::Hand;
    use crate::components::weapon::Weapon;

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(100.0),
                height: Val::Px(45.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(6.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.16, 0.16, 0.24, 1.0)), // Dark slate
            BorderColor::from(Color::srgba(0.3, 0.3, 0.4, 1.0)),
            BorderRadius::all(Val::Px(6.0)),
            ArsenalButton { side, kind },
        ))
        .observe(
            |trigger: On<Pointer<Click>>,
             button_query: Query<&ArsenalButton>,
             mut hand_query: Query<(&mut Hand, &mut Weapon)>,
             mut active_side: ResMut<super::resources::ActiveDescriptionSide>| {
                if let Ok(button_data) = button_query.get(trigger.entity) {
                    active_side.0 = button_data.side;
                    for (mut hand, mut weapon) in &mut hand_query {
                        if hand.side == button_data.side {
                            hand.equipped_weapon = Some(button_data.kind);
                            weapon.kind = button_data.kind;

                            match button_data.kind {
                                WeaponType::Magic => {
                                    weapon.cooldown = 0.8;
                                    weapon.damage = 0.0;
                                }
                                WeaponType::Gun => {
                                    weapon.cooldown = 0.5;
                                    weapon.damage = 0.0;
                                }
                                WeaponType::Shuriken => {
                                    weapon.cooldown = crate::configs::weapons::shuriken::COOLDOWN;
                                    weapon.skill_cooldown =
                                        crate::configs::weapons::shuriken::SKILL_COOLDOWN;
                                }
                                WeaponType::Sword => {
                                    weapon.cooldown = 0.5;
                                }
                            }
                        }
                    }
                }
            },
        )
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn spawn_magic_editor(
    parent: &mut ChildSpawnerCommands,
    side: HandType,
    asset_server: &AssetServer,
) {
    use super::components::{MagicUnifiedContainer, MagicPanel};

    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                display: Display::None,
                ..default()
            },
            MagicUnifiedContainer,
            MagicPanel { side },
        ))
        .with_children(|container| {
            // 1. Top Section: Active Slots
            spawn_magic_slots_section(container, side, asset_server);

            // 2. Middle Section: Spell Palette
            spawn_spell_palette_section(container, side, asset_server);

            // 3. Bottom Section: Info/Description
            spawn_magic_info_section(container);
        });
}

fn spawn_magic_slots_section(
    parent: &mut ChildSpawnerCommands,
    side: HandType,
    asset_server: &AssetServer,
) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Auto,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        })
        .with_children(|section| {
             // Primary Slot
             spawn_magic_slot_card(section, side, true, asset_server, "Primary (LMB/Q)");
             
             // Visual Divider or Icon
             // Visual Divider or Icon
             section.spawn((
                Text::new("⚡"),
                TextFont { font_size: 32.0, ..default() },
                TextColor(MAGIC_DIVIDER_COLOR),
             ));

             // Secondary Slot
             spawn_magic_slot_card(section, side, false, asset_server, "Secondary (RMB/E)");
        });
}

fn spawn_magic_slot_card(
    parent: &mut ChildSpawnerCommands,
    side: HandType,
    is_primary: bool,
    asset_server: &AssetServer,
    label: &str,
) {
    use super::components::{MagicSlotButton, MagicSlotIcon};
    use super::systems::magic_button_observer;

    let default_icon = if is_primary {
        "ui/icons/magic_bolt.png"
    } else {
        "ui/icons/magic_blink.png"
    };

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(160.0),
                height: Val::Px(200.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(MAGIC_SLOT_BG),
            BorderColor::from(MAGIC_SLOT_BORDER_DEFAULT),
            MagicSlotButton { side, is_primary },
        ))
        .observe(magic_button_observer)
        .observe(|trigger: On<Pointer<Over>>, mut border: Query<&mut BorderColor>| {
            if let Ok(mut border) = border.get_mut(trigger.entity) {
                *border = BorderColor::from(MAGIC_SLOT_BORDER_HIGHLIGHT);
            }
        })
        .observe(|trigger: On<Pointer<Out>>, mut border: Query<&mut BorderColor>| {
            if let Ok(mut border) = border.get_mut(trigger.entity) {
                *border = BorderColor::from(MAGIC_SLOT_BORDER_DEFAULT);
            }
        })
        .with_children(|card| {
            // Label
            card.spawn((
                Text::new(label),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::srgb(0.8, 0.8, 0.9)),
            ));

            // Icon Container
            card.spawn((
                Node {
                    width: Val::Px(96.0),
                    height: Val::Px(96.0),
                    margin: UiRect::vertical(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BorderColor::from(MAGIC_SLOT_BORDER_DEFAULT),
            )).with_children(|icon_container| {
                icon_container.spawn((
                    ImageNode::new(asset_server.load(default_icon)),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    MagicSlotIcon,
                ));
            });

            // Spell Name (Dynamic)
            card.spawn((
                Text::new("Spell Name"),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
}


fn spawn_spell_palette_section(
    parent: &mut ChildSpawnerCommands,
    _side: HandType,
    asset_server: &AssetServer,
) {
    use super::components::{SpellListButton, MagicPaletteContainer};
    use super::systems::spell_list_observer;

    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                flex_grow: 1.0, 
                ..default()
            },
            MagicPaletteContainer,
        ))
        .with_children(|container| {
            container.spawn((
                Text::new("SPELL PALETTE"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(0.6, 0.6, 0.7)),
                Node { margin: UiRect::bottom(Val::Px(10.0)), ..default() },
            ));

             container
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(15.0),
                    column_gap: Val::Px(15.0),
                    width: Val::Percent(100.0),
                    ..default()
                })
                .with_children(|grid| {
                    let spells = vec![
                        (SpellType::EnergyBolt, "ui/icons/magic_bolt.png"),
                        (SpellType::Laser, "ui/icons/magic_laser.png"),
                        (SpellType::Nova, "ui/icons/magic_nova.png"),
                        (SpellType::Blink, "ui/icons/magic_blink.png"),
                        (SpellType::Global, "ui/icons/magic_global.png"),
                        (SpellType::ForcePush, "ui/icons/magic_push.png"),
                        (SpellType::ForcePull, "ui/icons/magic_pull.png"),
                    ];

                    for (spell, icon_path) in spells {
                        grid.spawn((
                            Button,
                            Node {
                                width: Val::Px(64.0),
                                height: Val::Px(64.0),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(MAGIC_SLOT_BG),
                            BorderColor::all(Color::NONE),
                            SpellListButton(spell),
                        ))
                        .observe(spell_list_observer)
                         .observe(|trigger: On<Pointer<Over>>, mut bg: Query<&mut BackgroundColor>| {
                            if let Ok(mut bg) = bg.get_mut(trigger.entity) {
                                *bg = BackgroundColor(MAGIC_SLOT_BG_HOVER);
                            }
                        })
                        .observe(|trigger: On<Pointer<Out>>, mut bg: Query<&mut BackgroundColor>| {
                             if let Ok(mut bg) = bg.get_mut(trigger.entity) {
                                *bg = BackgroundColor(MAGIC_SLOT_BG);
                            }
                        })
                        .with_children(|btn| {
                            btn.spawn((
                                ImageNode::new(asset_server.load(icon_path)),
                                Node {
                                    width: Val::Px(48.0),
                                    height: Val::Px(48.0),
                                    ..default()
                                },
                            ));
                        });
                    }
                });
        });
}

fn spawn_magic_info_section(parent: &mut ChildSpawnerCommands) {
    use super::components::{SpellListDescriptionText, MagicInfoContainer};

    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(80.0),
            padding: UiRect::all(Val::Px(10.0)),
            margin: UiRect::top(Val::Px(20.0)),
            border: UiRect::top(Val::Px(1.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::from(MAGIC_SLOT_BORDER_DEFAULT),
        BackgroundColor(MAGIC_INFO_BG),
        MagicInfoContainer,
    )).with_children(|info| {
         info.spawn((
            Text::new("Select a spell to see details..."),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
            SpellListDescriptionText,
        ));
    });
}

pub fn spawn_weapon_detail_panel(
    parent: &mut ChildSpawnerCommands,
    side: HandType,
    asset_server: &AssetServer,
) {
    use super::components::WeaponDetailPanel;

    parent
        .spawn((
            Node {
                width: Val::Percent(50.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(10.0)),
                display: Display::None,
                ..default()
            },
            WeaponDetailPanel { side },
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new("Description..."),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    max_width: Val::Px(250.0),
                    ..default()
                },
                WeaponDescriptionText,
            ));

            spawn_sword_group(panel, side, asset_server);
            spawn_gun_group(panel, side, asset_server);
            spawn_shuriken_group(panel, side, asset_server);

            // Magic Spell List (Only visible for Magic)
            // Removed: spawn_spell_list_panel(panel, side, asset_server);
        });
}

// Function spawn_spell_list_panel removed and replaced by spawn_magic_editor

fn spawn_sword_group(
    parent: &mut ChildSpawnerCommands,
    side: HandType,
    asset_server: &AssetServer,
) {
    use super::components::WeaponStateGroup;
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None,
                ..default()
            },
            WeaponStateGroup {
                side,
                weapon_type: WeaponType::Sword,
            },
        ))
        .with_children(|group| {
            group.spawn((
                ImageNode::new(asset_server.load("ui/icons/sword_normal.png")),
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    margin: UiRect::horizontal(Val::Px(10.0)),
                    ..default()
                },
            ));
            group.spawn((
                ImageNode::new(asset_server.load("ui/icons/sword_shattered.png")),
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    margin: UiRect::horizontal(Val::Px(10.0)),
                    ..default()
                },
            ));
        });
}

fn spawn_gun_group(parent: &mut ChildSpawnerCommands, side: HandType, asset_server: &AssetServer) {
    use super::components::WeaponStateGroup;
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None,
                ..default()
            },
            WeaponStateGroup {
                side,
                weapon_type: WeaponType::Gun,
            },
        ))
        .with_children(|group| {
            group.spawn((
                ImageNode::new(asset_server.load("ui/icons/gun_single.png")),
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    margin: UiRect::horizontal(Val::Px(10.0)),
                    ..default()
                },
            ));
            group.spawn((
                ImageNode::new(asset_server.load("ui/icons/gun_shotgun.png")),
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    margin: UiRect::horizontal(Val::Px(10.0)),
                    ..default()
                },
            ));
            group.spawn((
                ImageNode::new(asset_server.load("ui/icons/gun_rapid.png")),
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    margin: UiRect::horizontal(Val::Px(10.0)),
                    ..default()
                },
            ));
        });
}

fn spawn_shuriken_group(
    parent: &mut ChildSpawnerCommands,
    side: HandType,
    asset_server: &AssetServer,
) {
    use super::components::WeaponStateGroup;
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None,
                ..default()
            },
            WeaponStateGroup {
                side,
                weapon_type: WeaponType::Shuriken,
            },
        ))
        .with_children(|group| {
            group.spawn((
                ImageNode::new(asset_server.load("ui/icons/shuriken.png")),
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    ..default()
                },
            ));
        });
}

fn spawn_left_column(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            width: Val::Percent(30.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                Text::new("LEFT HAND"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
            ));

            spawn_weapon_button(col, HandType::Left, WeaponType::Shuriken, "Shuriken");
            spawn_weapon_button(col, HandType::Left, WeaponType::Sword, "Sword");
            spawn_weapon_button(col, HandType::Left, WeaponType::Gun, "Gun");
            spawn_weapon_button(col, HandType::Left, WeaponType::Magic, "Magic");
        });
}

fn spawn_center_column(parent: &mut ChildSpawnerCommands, asset_server: &AssetServer) {
    use super::components::DescriptionWrapper;

    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            width: Val::Percent(40.0),
            height: Val::Percent(100.0),
            padding: UiRect::horizontal(Val::Px(10.0)),
            ..default()
        })
        .with_children(|center| {
            center.spawn((
                Text::new("DESCRIPTION"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Left Group: Magic (Selection) LEFT | Description RIGHT (Row)
            center
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        display: Display::None,
                        ..default()
                    },
                    DescriptionWrapper {
                        side: HandType::Left,
                    },
                ))
                .with_children(|row| {
                    spawn_magic_editor(row, HandType::Left, asset_server);
                    spawn_weapon_detail_panel(row, HandType::Left, asset_server);
                });

            // Right Group: Description LEFT | Magic (Selection) RIGHT (RowReverse)
            center
                .spawn((
                    Node {
                        flex_direction: FlexDirection::RowReverse,
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        display: Display::None,
                        ..default()
                    },
                    DescriptionWrapper {
                        side: HandType::Right,
                    },
                ))
                .with_children(|row| {
                    spawn_magic_editor(row, HandType::Right, asset_server);
                    spawn_weapon_detail_panel(row, HandType::Right, asset_server);
                });
        });
}

fn spawn_right_column(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            width: Val::Percent(30.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                Text::new("RIGHT HAND"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.7, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
            ));

            spawn_weapon_button(col, HandType::Right, WeaponType::Shuriken, "Shuriken");
            spawn_weapon_button(col, HandType::Right, WeaponType::Sword, "Sword");
            spawn_weapon_button(col, HandType::Right, WeaponType::Gun, "Gun");
            spawn_weapon_button(col, HandType::Right, WeaponType::Magic, "Magic");
        });
}

pub fn spawn_equipment_panel(parent: &mut ChildSpawnerCommands, asset_server: &AssetServer) {
    use super::components::EquipmentContainer;

    // === EQUIPMENT CONTAINER (Equip Tab) ===
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::FlexStart,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::None, // Default hidden
                ..default()
            },
            EquipmentContainer,
        ))
        .with_children(|equip| {
            spawn_left_column(equip);
            spawn_center_column(equip, asset_server);
            spawn_right_column(equip);
        });
}
