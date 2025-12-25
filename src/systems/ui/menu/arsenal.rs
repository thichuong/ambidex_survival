use super::systems::WeaponDescriptionText;
use crate::components::player::HandType;
use crate::components::weapon::{MagicLoadout, SpellType, WeaponType};
use bevy::prelude::*;

pub fn get_spell_description(spell_type: SpellType) -> String {
    match spell_type {
        SpellType::EnergyBolt => "Energy Bolt: Creates a large explosion on impact.".to_string(),
        SpellType::Laser => "Laser: Instant-hit high-velocity beam.".to_string(),
        SpellType::Nova => "Nova: Radial burst of high area damage.".to_string(),
        SpellType::Blink => "Blink: Teleport to cursor & Invulnerable.".to_string(),
        SpellType::Global => "Global: Massive strike hitting ALL enemies.".to_string(),
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
                width: Val::Px(80.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0)),
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
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn spawn_magic_panel(
    parent: &mut ChildSpawnerCommands,
    side: HandType,
    asset_server: &AssetServer,
) {
    use super::components::{MagicCycleButton, MagicPanel};

    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::vertical(Val::Px(20.0)),
                padding: UiRect::all(Val::Px(10.0)),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.0, 0.2, 0.8)),
            MagicPanel { side },
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new("Select Spell"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.5, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            let label = match side {
                HandType::Left => "Left Magic (Q): ",
                HandType::Right => "Right Magic (E): ",
            };
            panel.spawn((
                Text::new(label),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            panel
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                })
                .with_children(|icons| {
                    icons.spawn((
                        ImageNode::new(asset_server.load("ui/icons/magic_bolt.png")),
                        Node {
                            width: Val::Px(80.0),
                            height: Val::Px(80.0),
                            margin: UiRect::right(Val::Px(10.0)),
                            ..default()
                        },
                        MagicCycleButton {
                            side,
                            is_primary: true,
                        },
                    ));

                    icons.spawn((
                        ImageNode::new(asset_server.load("ui/icons/magic_blink.png")),
                        Node {
                            width: Val::Px(60.0),
                            height: Val::Px(60.0),
                            ..default()
                        },
                        MagicCycleButton {
                            side,
                            is_primary: false,
                        },
                    ));
                });

            spawn_magic_cycle_button(panel, side, true, "Primary: Bolt");
            spawn_magic_cycle_button(panel, side, false, "Secondary: Blink");
        });
}

fn spawn_magic_cycle_button(
    parent: &mut ChildSpawnerCommands,
    side: HandType,
    is_primary: bool,
    text: &str,
) {
    use super::components::MagicCycleButton;
    use super::systems::magic_button_observer;

    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(90.0),
                height: Val::Px(35.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::vertical(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.4, 0.0, 0.4, 1.0)),
            MagicCycleButton { side, is_primary },
        ))
        .observe(magic_button_observer)
        .observe(
            |trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = color.get_mut(trigger.entity) {
                    *color = BackgroundColor(Color::srgba(0.5, 0.0, 0.5, 1.0));
                }
            },
        )
        .observe(
            |trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = color.get_mut(trigger.entity) {
                    *color = BackgroundColor(Color::srgba(0.4, 0.0, 0.4, 1.0));
                }
            },
        )
        .with_children(|btn| {
            btn.spawn((
                Text::new(text),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
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
                width: Val::Percent(100.0),
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
        });
}

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

#[allow(clippy::too_many_lines)]
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
            // === LEFT COLUMN (Left Hand Buttons) ===
            equip
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

            // === CENTER COLUMN (Shared Details) ===
            equip
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

                    // Spawn Panels for both sides - Visibility controlled by system
                    spawn_magic_panel(center, HandType::Left, asset_server);
                    spawn_weapon_detail_panel(center, HandType::Left, asset_server);

                    spawn_magic_panel(center, HandType::Right, asset_server);
                    spawn_weapon_detail_panel(center, HandType::Right, asset_server);
                });

            // === RIGHT COLUMN (Right Hand Buttons) ===
            equip
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
        });
}
