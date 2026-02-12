---
name: Add Feature Guide
description: Hướng dẫn tổng quát thêm feature mới (enemy, shop upgrade, GameState, UI) vào Ambidex Survival
---

# Add Feature Guide — Ambidex Survival

Skill này cung cấp checklist và hướng dẫn cho các loại feature phổ biến.

---

## 1. Thêm Enemy Type Mới

### ECS Flow
```
Component (data) → Config (balance) → System (AI logic) → Plugin (register) → Visuals (render)
```

### Bước thực hiện

#### 1.1 Component marker

**File**: `src/components/enemy.rs`

```rust
/// Marker cho enemy type mới
#[derive(Component)]
pub struct BossEnemy;
```

#### 1.2 Config

**File**: `src/configs/enemy.rs`

```rust
pub const BOSS_BASE_HP: f32 = 500.0;
pub const BOSS_SPEED: f32 = 100.0;
pub const BOSS_DAMAGE: f32 = 30.0;
pub const BOSS_GOLD_REWARD: u32 = 500;
pub const BOSS_SPAWN_ROUND: u32 = 10;
pub const BOSS_COLOR: Color = Color::srgb(0.8, 0.1, 0.1);
```

#### 1.3 AI System

**File mới**: `src/systems/combat/boss_ai.rs`

```rust
use bevy::prelude::*;
use crate::components::enemy::BossEnemy;
use crate::components::player::Player;

#[allow(clippy::needless_pass_by_value)]
pub fn boss_ai_system(
    time: Res<Time>,
    mut boss_query: Query<(&mut Transform, &BossEnemy)>,
    player: Single<&Transform, (With<Player>, Without<BossEnemy>)>,
) {
    let player_pos = player.translation.truncate();
    let dt = time.delta_secs();

    for (mut transform, _boss) in &mut boss_query {
        // AI logic: chase, attack patterns, fase changes
        let boss_pos = transform.translation.truncate();
        let direction = (player_pos - boss_pos).normalize_or_zero();
        transform.translation += (direction * 100.0 * dt).extend(0.0);
    }
}
```

#### 1.4 Spawn logic

**File**: `src/systems/enemy.rs` — thêm vào `spawn_waves` function.

#### 1.5 Plugin registration

**File**: `src/plugins/combat.rs`

```rust
app.add_systems(
    Update,
    boss_ai_system.run_if(in_state(GameState::Playing)),
);
```

#### 1.6 Visuals

Thêm visual mesh/color cho enemy trong hệ thống spawn.

### Checklist Enemy Mới
- [ ] Marker component trong `components/enemy.rs`
- [ ] Config constants trong `configs/enemy.rs`
- [ ] AI system trong `systems/combat/`
- [ ] Spawn logic trong `systems/enemy.rs`
- [ ] Plugin registration trong `plugins/combat.rs`
- [ ] Visual appearance (color, size, mesh)
- [ ] HP scaling per round
- [ ] Gold reward
- [ ] Death event handling (particles, gold drop)

---

## 2. Thêm Shop Upgrade Mới

### Bước thực hiện

#### 2.1 Thêm ShopButton variant

**File**: `src/systems/ui/components.rs` (hoặc nơi `ShopButton` enum định nghĩa)

```rust
pub enum ShopButton {
    // ... existing ...
    ArmorUp,  // ← Thêm mới
}
```

#### 2.2 Thêm CardConfig

**File**: `src/configs/shop.rs`

```rust
ShopButton::ArmorUp => CardConfig {
    name: "Armor Up",
    price: 200,
    limit: Some(5),
    value: 5.0,
    description: "+5 Armor",
},
```

#### 2.3 Thêm stat tracking trong Progression

**File**: `src/components/player.rs`

```rust
#[derive(Component, Default)]
pub struct Progression {
    // ... existing fields ...
    pub armor_upgrades: u32,  // ← Thêm mới
}
```

#### 2.4 Xử lý purchase logic

**File**: `src/systems/ui/menu/interaction.rs`

Thêm match arm cho `ShopButton::ArmorUp` trong purchase handler.

#### 2.5 Áp dụng effect

Sửa system liên quan để sử dụng stat mới (ví dụ: damage reduction trong `collision/damage.rs`).

### Checklist Shop Upgrade
- [ ] `ShopButton` enum variant
- [ ] `CardConfig` trong `configs/shop.rs`
- [ ] Stat field trong `Progression` hoặc `CombatStats`
- [ ] Purchase handler trong UI interaction
- [ ] Effect applied trong relevant system
- [ ] UI card spawn trong `menu/shop.rs`
- [ ] Visual feedback (card count, limit display)

---

## 3. Thêm GameState Mới

### Bước thực hiện

#### 3.1 Thêm State variant

**File**: `src/resources/game_state.rs`

```rust
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    Playing,
    #[default]
    WeaponMenu,
    Tutorial,
    GameOver,
    Settings,
    Inventory,  // ← Thêm mới
}
```

#### 3.2 Tạo UI system

**File mới**: `src/systems/ui/inventory.rs`

```rust
use bevy::prelude::*;
use crate::resources::game_state::GameState;

pub fn spawn_inventory_ui(mut commands: Commands) {
    // Build UI hierarchy
}

pub fn cleanup_inventory_ui(
    mut commands: Commands,
    query: Query<Entity, With<InventoryRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
pub struct InventoryRoot;
```

#### 3.3 Schedule enter/exit systems

**File**: `src/plugins/ui.rs`

```rust
app.add_systems(OnEnter(GameState::Inventory), spawn_inventory_ui)
   .add_systems(OnExit(GameState::Inventory), cleanup_inventory_ui);
```

#### 3.4 Navigation

Cập nhật `PreviousMenuState` và Back button logic để support state mới.

### Checklist GameState Mới
- [ ] State variant trong `game_state.rs`
- [ ] UI spawn system (`OnEnter`)
- [ ] UI cleanup system (`OnExit`)
- [ ] Plugin registration
- [ ] Navigation (transition buttons, Back logic)
- [ ] `PreviousMenuState` integration
- [ ] System scheduling (`.run_if(in_state(...))`)

---

## 4. Thêm Visual Effect Mới

### Pattern

```rust
// src/visuals/world/my_effect.rs

use bevy::prelude::*;
use crate::resources::cached_assets::CachedAssets;

pub fn spawn_my_effect(
    commands: &mut Commands,
    position: Vec2,
    cached_assets: &CachedAssets,
) {
    commands.spawn((
        Transform::from_translation(position.extend(5.0)),
        // Sử dụng cached mesh & material
        Mesh2d(cached_assets.circle_mesh.clone()),
        MeshMaterial2d(cached_assets.some_material.clone()),
        Lifetime {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        },
    ));
}
```

### Quy tắc Visuals
- Dùng `CachedAssets` — **KHÔNG** tạo mesh/material mới mỗi frame.
- Set `z` translation cho depth ordering (UI > Effects > Entities > Background).
- Thêm `Lifetime` component để tự cleanup.
- Visual entities tách riêng khỏi logic entities.

---

## 5. Dependency Graph Tổng Quát

```text
Feature mới
    ├── 1. Component (data struct)
    │       └── components/*.rs
    ├── 2. Config (balance constants)
    │       └── configs/**/*.rs
    ├── 3. System (game logic)
    │       └── systems/**/*.rs
    ├── 4. Event (communication)
    │       └── systems/combat/events.rs
    ├── 5. Plugin (registration)
    │       └── plugins/*.rs
    ├── 6. Visuals (rendering)
    │       └── visuals/**/*.rs
    └── 7. UI (player interaction)
            └── systems/ui/**/*.rs
```

> **Implement theo thứ tự trên** — mỗi layer phụ thuộc vào layer trước.

---

## Quick Reference: Tất Cả Files Quan Trọng

| Module | Đường dẫn | Mục đích |
|--------|-----------|----------|
| Entry point | `src/main.rs` | Plugin & resource registration |
| Player components | `src/components/player.rs` | Player, Hand, Health, CombatStats, Progression |
| Weapon components | `src/components/weapon.rs` | WeaponType, SpellType, Projectile, Faction |
| Enemy components | `src/components/enemy.rs` | Enemy markers và stats |
| Physics | `src/components/physics.rs` | Velocity, Collider, UniformGrid |
| GameState | `src/resources/game_state.rs` | State machine |
| Combat context | `src/systems/combat/mod.rs` | CombatContext, CombatInputParams |
| Events | `src/systems/combat/events.rs` | CollisionEvent, EnemyDeathEvent |
| Collision pipeline | `src/systems/combat/collision/` | Detection → Damage → Effects → Death |
| Shop config | `src/configs/shop.rs` | CardConfig, prices, limits |
| Weapon configs | `src/configs/weapons/` | Per-weapon balance constants |
| Spell configs | `src/configs/spells/` | Per-spell balance constants |
| UI menu | `src/systems/ui/menu/` | Weapon menu sub-modules |
| Combat plugin | `src/plugins/combat.rs` | System registration |
| UI plugin | `src/plugins/ui.rs` | UI system registration |
