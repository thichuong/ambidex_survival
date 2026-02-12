---
name: Weapon & Spell System
description: Hướng dẫn thêm weapon hoặc spell mới vào Ambidex Survival
---

# Weapon & Spell System — Hướng Dẫn Thêm Mới

Skill này hướng dẫn quy trình từng bước để thêm weapon type hoặc spell mới.

---

## Thêm Weapon Mới (ví dụ: "Spear")

### Bước 1: Thêm WeaponType enum variant

**File**: `src/components/weapon.rs`

```rust
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    Shuriken,
    Sword,
    Gun,
    Magic,
    Spear,  // ← Thêm mới
}
```

### Bước 2: Tạo config

**File mới**: `src/configs/weapons/spear.rs`

```rust
pub const SPEAR_DAMAGE: f32 = 25.0;
pub const SPEAR_COOLDOWN: f32 = 0.8;
pub const SPEAR_RANGE: f32 = 200.0;
pub const SPEAR_SPEED: f32 = 600.0;
pub const SPEAR_SKILL_COOLDOWN: f32 = 5.0;
```

**Cập nhật**: `src/configs/weapons/mod.rs`
```rust
pub mod spear;  // ← Thêm dòng này
```

### Bước 3: Tạo weapon system

**File mới**: `src/systems/combat/spear.rs`

```rust
use crate::components::player::{CombatStats, Hand, HandType, Player, PlayerStats, Progression};
use crate::components::weapon::{Weapon, WeaponType};
use crate::systems::combat::{CombatContext, CombatInputParams};
use bevy::prelude::*;

#[allow(clippy::type_complexity, clippy::needless_pass_by_value)]
pub fn spear_weapon_system(
    mut params: CombatInputParams,
    mut player_query: Query<
        (&mut Transform, &PlayerStats, &CombatStats, &Progression, Entity),
        With<Player>,
    >,
    hand_query: Query<(&Hand, &Weapon, Entity)>,
) {
    let Ok((mut transform, stats, combat_stats, progression, player_entity)) =
        player_query.single_mut()
    else {
        return;
    };

    // Tính cursor position
    let (camera, cam_transform) = *params.camera;
    let cursor_pos = params.window
        .cursor_position()
        .and_then(|pos| camera.viewport_to_world_2d(cam_transform, pos).ok())
        .unwrap_or_default();

    let spawn_pos = transform.translation.truncate();

    let mut ctx = CombatContext {
        owner_entity: player_entity,
        transform: &mut transform,
        cursor_pos,
        spawn_pos,
        damage_multiplier: stats.damage_multiplier,
        combat_stats,
        progression,
    };

    // Process mỗi hand
    for (hand, weapon, _hand_entity) in &hand_query {
        if weapon.kind != WeaponType::Spear {
            continue;
        }
        
        // Kiểm tra input theo hand side
        let should_fire = match hand.side {
            HandType::Left => params.input_settings.is_left_attack_pressed(&params.mouse_input, &params.key_input),
            HandType::Right => params.input_settings.is_right_attack_pressed(&params.mouse_input, &params.key_input),
        };

        if should_fire {
            fire_spear(&mut ctx, &mut params);
        }
    }
}

fn fire_spear(ctx: &mut CombatContext, params: &mut CombatInputParams) {
    // Weapon logic:
    // 1. Spawn projectile entity
    // 2. Set Velocity, Collider, Lifetime, Faction
    // 3. Use configs for damage, speed, range
    let _ = (ctx, params); // TODO: Implement
}
```

**Cập nhật**: `src/systems/combat/mod.rs`
```rust
pub mod spear;    // ← Thêm module
pub use spear::*; // ← Re-export
```

### Bước 4: Đăng ký trong Plugin

**File**: `src/plugins/combat.rs`

```rust
use crate::systems::combat::spear_weapon_system;

// Trong build():
app.add_systems(
    Update,
    (
        // ... existing systems ...
        spear_weapon_system,  // ← Thêm mới
    ).run_if(in_state(GameState::Playing)),
);
```

### Bước 5: Tạo Visuals

**File**: `src/visuals/world/projectiles.rs` hoặc `src/visuals/world/melee.rs`

Thêm function spawn visual entity cho spear projectile/swing.

### Bước 6: Thêm vào UI Weapon Menu

**File**: `src/systems/ui/menu/arsenal.rs`

Thêm Spear vào danh sách weapons có thể trang bị.

---

## Thêm Spell Mới (ví dụ: "Meteor")

### Bước 1: Thêm SpellType enum variant

**File**: `src/components/weapon.rs`

```rust
pub enum SpellType {
    EnergyBolt,
    Laser,
    Nova,
    Blink,
    Global,
    Meteor,  // ← Thêm mới
}
```

### Bước 2: Tạo config

**File mới**: `src/configs/spells/meteor.rs`

```rust
pub const METEOR_DAMAGE: f32 = 100.0;
pub const METEOR_RADIUS: f32 = 150.0;
pub const METEOR_COOLDOWN: f32 = 8.0;
pub const METEOR_FALL_DURATION: f32 = 1.0;
```

**Cập nhật**: `src/configs/spells/mod.rs`
```rust
pub mod meteor;  // ← Thêm dòng này
```

### Bước 3: Tạo spell system

**File mới**: `src/systems/combat/magic/meteor.rs`

```rust
use crate::systems::combat::CombatContext;
use crate::systems::combat::CombatInputParams;
use crate::components::weapon::Faction;
use crate::configs::spells::meteor::*;
use bevy::prelude::*;

pub fn cast_meteor(
    ctx: &mut CombatContext,
    params: &mut CombatInputParams,
) {
    // 1. Spawn meteor entity at cursor_pos hoặc target position
    // 2. Dùng Lifetime timer cho fall duration
    // 3. On impact: spawn AoE explosion (ExplodingProjectile)
    // 4. Set Faction::Player
    let _ = (ctx, params); // TODO: Implement
}
```

**Cập nhật**: `src/systems/combat/magic/mod.rs` — thêm match arm trong spell dispatch.

### Bước 4: Tạo Visuals

**File**: `src/visuals/world/spells.rs`

Thêm function spawn visual cho meteor (falling animation + impact explosion).

### Bước 5: Thêm vào Magic Loadout UI

Cho phép chọn Meteor trong weapon menu spell slot selection.

---

## Checklist Thêm Weapon/Spell Mới

- [ ] Enum variant trong `components/weapon.rs`
- [ ] Config file trong `configs/weapons/` hoặc `configs/spells/`
- [ ] Config module re-export trong `mod.rs`
- [ ] System file trong `systems/combat/` hoặc `systems/combat/magic/`
- [ ] System re-export trong parent `mod.rs`
- [ ] Plugin registration trong `plugins/combat.rs`
- [ ] Visual spawn trong `visuals/world/`
- [ ] UI integration (weapon menu, HUD icons)
- [ ] `Faction` component trên spawned projectiles
- [ ] `CombatContext` pattern cho system function signatures
- [ ] `cargo check` ✓
- [ ] `cargo clippy` ✓
- [ ] `cargo test` ✓
- [ ] `cargo fmt` ✓

---

## Kiến trúc Weapon Hiện Tại

```text
WeaponType::Sword    → systems/combat/sword.rs + sword_mechanics.rs
WeaponType::Gun      → systems/combat/gun.rs
WeaponType::Shuriken → systems/combat/shuriken.rs
WeaponType::Magic    → systems/combat/magic/mod.rs (dispatch)
  ├── SpellType::EnergyBolt → magic/energy_bolt.rs
  ├── SpellType::Laser      → magic/laser.rs
  ├── SpellType::Nova       → magic/nova.rs
  ├── SpellType::Blink      → magic/blink.rs
  └── SpellType::Global     → magic/global_spell.rs
```

## Các File Liên Quan

| Loại | Đường dẫn |
|------|-----------|
| Enum WeaponType/SpellType | `src/components/weapon.rs` |
| Hand & Player | `src/components/player.rs` |
| CombatContext | `src/systems/combat/mod.rs` |
| Collision Pipeline | `src/systems/combat/collision/` |
| Weapon Configs | `src/configs/weapons/` |
| Spell Configs | `src/configs/spells/` |
| Combat Plugin | `src/plugins/combat.rs` |
| Weapon Visuals | `src/visuals/world/` |
| Weapon Menu UI | `src/systems/ui/menu/` |
